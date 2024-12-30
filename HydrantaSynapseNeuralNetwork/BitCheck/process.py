import typing as ty
from lava.magma.core.process.process import LogConfig, AbstractProcess
from lava.magma.core.process.ports.ports import RefPort
from lava.magma.core.process.variable import Var


class BitCheck(AbstractProcess):
    BITS_MIN = 1
    BITS_MAX = 31

    def __init__(
        self,
        *,
        layerid: ty.Optional[int] = None,
        debug: ty.Optional[int] = 0,
        bits: ty.Optional[int] = 24,
        name: ty.Optional[str] = None,
        log_config: ty.Optional[LogConfig] = None,
        **kwargs,
    ) -> None:
        """
        BitCheck process.
        This process checks for potential bit overflow
        when running on bit-limited hardware.

        Parameters
        ----------
        layerid : Optional[int]
            Layer ID of the network. Default is None.
        debug : Optional[int]
            Enable (1) or disable (0) debug print. Default is 0.
        bits : Optional[int]
            Number of bits (1-31) used for overflow checking. Default is 24.
        name : Optional[str]
            Name of the process. Default is None.
        log_config : Optional[LogConfig]
            Logging configuration. Default is None.
        """
        super().__init__(name=name, log_config=log_config, **kwargs)

        self.state = RefPort((1,))
        self.layerid: Var = Var(shape=(1,), init=layerid)
        self.debug: Var = Var(shape=(1,), init=debug)

        if not (self.BITS_MIN <= bits <= self.BITS_MAX):
            raise ValueError(
                f"Invalid 'bits' value: {bits}. Must be between {self.BITS_MIN} and {self.BITS_MAX}."
            )
        self.bits: Var = Var(shape=(1,), init=bits)
        self._overflowed: Var = Var(shape=(1,), init=0)

    def connect_var(self, var: Var) -> None:
        """
        Connect a variable to the BitCheck process and reinitialize variables.

        Parameters
        ----------
        var : Var
            The variable to connect.
        """
        self.state = RefPort(var.shape)
        self.state.connect_var(var)

        self.layerid = Var(name="layerid", shape=var.shape, init=self.layerid.init)
        self.debug = Var(name="debug", shape=var.shape, init=self.debug.init)
        self.bits = Var(name="bits", shape=var.shape, init=self.bits.init)
        self._overflowed = Var(name="overflowed", shape=var.shape, init=self._overflowed.init)

        self._post_init()

    @property
    def shape(self) -> ty.Tuple[int, ...]:
        """Return the shape of the process."""
        return self.state.shape

    @property
    def overflowed(self) -> int:
        """
        Return whether the process has overflowed.

        Returns
        -------
        int
            1 if overflowed, 0 otherwise.
        """
        return self._overflowed.get()
