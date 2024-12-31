from typing import Optional, Dict, Any, List
from contextlib import ContextDecorator
from datetime import datetime
from multiprocessing import Process, Event, Queue, Manager
from concurrent.futures import ThreadPoolExecutor
from abc import abstractmethod
from dataclasses import dataclass
import queue
import atexit


@dataclass
class EventMetadata:
    """Stores an event to monitor and associated metadata."""
    event: Event
    channel_name: str
    method_type: str


class EventCompletionMonitor:
    """Monitors events from a queue and logs timeouts."""
    @staticmethod
    def monitor(queue: Queue, timeout: float, thread_pool_size: int = 5):
        with ThreadPoolExecutor(max_workers=thread_pool_size) as executor:
            while True:
                try:
                    metadata: Optional[EventMetadata] = queue.get(timeout=1)
                    if metadata is None:  # Sentinel to exit the loop
                        break
                    executor.submit(EventCompletionMonitor.event_monitor, metadata, timeout)
                except queue.Empty:
                    continue

    @staticmethod
    def event_monitor(metadata: EventMetadata, timeout: float):
        while not metadata.event.wait(timeout):
            now = datetime.now().strftime("%H:%M:%S")
            print(f"{now} : Blocked on {metadata.channel_name} :: {metadata.method_type}", flush=True)
        # Event completed successfully


@dataclass
class WatchdogToken:
    """Encapsulates related entries for a Watchdog."""
    event: Event
    queue: Queue
    channel_name: str
    method_type: str


class Watchdog(ContextDecorator):
    """Monitors an event, printing timeouts if they occur."""
    def __init__(self, token: Optional[WatchdogToken]):
        self.token = token

    def __enter__(self):
        if self.token:
            self.token.event.clear()
            self.token.queue.put(EventMetadata(
                event=self.token.event,
                channel_name=self.token.channel_name,
                method_type=self.token.method_type
            ))
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.token:
            self.token.event.set()
        return False  # Do not suppress exceptions


class WatchdogManagerInterface:
    """Generic interface for managing watchdogs."""
    @abstractmethod
    def lq(self) -> Optional[Queue]:
        pass

    @abstractmethod
    def sq(self) -> Optional[Queue]:
        pass

    @abstractmethod
    def create_watchdog(self, queue: Queue, channel_name: str, method_type: str) -> Watchdog:
        pass

    def start(self):
        pass

    def stop(self):
        pass


class WatchdogManager(WatchdogManagerInterface):
    """Manages event queues and monitoring processes."""
    def __init__(self, long_event_timeout: float, short_event_timeout: float, thread_pool_size: int = 5):
        self._manager = Manager()
        self._lq = self._manager.Queue()
        self._sq = self._manager.Queue()
        self._long_event_timeout = long_event_timeout
        self._short_event_timeout = short_event_timeout
        self._thread_pool_size = thread_pool_size
        self._long_monitor = None
        self._short_monitor = None
        atexit.register(self.stop)

    @property
    def lq(self) -> Optional[Queue]:
        return self._lq

    @property
    def sq(self) -> Optional[Queue]:
        return self._sq

    def create_watchdog(self, queue: Queue, channel_name: str, method_type: str) -> Watchdog:
        return Watchdog(WatchdogToken(
            event=self._manager.Event(),
            queue=queue,
            channel_name=channel_name,
            method_type=method_type
        ))

    def start(self):
        self._long_monitor = Process(target=EventCompletionMonitor.monitor, args=(self._lq, self._long_event_timeout, self._thread_pool_size))
        self._short_monitor = Process(target=EventCompletionMonitor.monitor, args=(self._sq, self._short_event_timeout, self._thread_pool_size))
        self._long_monitor.start()
        self._short_monitor.start()

    def stop(self):
        if self._lq:
            self._lq.put(None)  # Signal long event monitor to stop
        if self._sq:
            self._sq.put(None)  # Signal short event monitor to stop

        if self._long_monitor:
            self._long_monitor.join()
        if self._short_monitor:
            self._short_monitor.join()

        self._manager.shutdown()


class NoOPWatchdogManager(WatchdogManagerInterface):
    """No-op implementation for testing or disabled mode."""
    @property
    def lq(self) -> Optional[Queue]:
        return None

    @property
    def sq(self) -> Optional[Queue]:
        return None

    def create_watchdog(self, queue: Queue, channel_name: str, method_type: str) -> Watchdog:
        return Watchdog(None)

    def start(self):
        pass

    def stop(self):
        pass


class WatchdogManagerBuilder:
    """Builds a WatchdogManager with configuration."""
    def __init__(self, config: Dict[str, Any]):
        self._long_event_timeout = config.get("long_event_timeout", 10.0)
        self._short_event_timeout = config.get("short_event_timeout", 5.0)
        self._use_watchdog = config.get("use_watchdog", False)

    def build(self) -> WatchdogManagerInterface:
        if self._use_watchdog:
            return WatchdogManager(self._long_event_timeout, self._short_event_timeout)
        return NoOPWatchdogManager()
