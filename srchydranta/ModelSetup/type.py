import typing as ty
from dataclasses import dataclass
from lava.magma.core.model.py.ports import PyInPort, PyOutPort, PyRefPort


@dataclass
class LavaPyType:
    cls: ty.Union[
        type, ty.Type[PyInPort], ty.Type[PyOutPort], ty.Type[PyRefPort]
    ]
    d_type: type
    precision: int = None  # If None, infinite precision is assumed
