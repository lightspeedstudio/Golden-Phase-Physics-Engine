"""Golden Phase SDK type definitions."""

from dataclasses import dataclass
from enum import IntEnum
from typing import List, Tuple


class Result(IntEnum):
    OK = 0
    MISSING_LAW = 1
    REFUSED = 2
    INVALID_BODY = 3
    INVALID_STATE = 4
    BOUNDARY_REFUSAL = 5
    ILLEGAL_TRANSITION = 6
    NOT_CONNECTED = 253
    PROTOCOL_ERROR = 254
    INTERNAL_ERROR = 255


class Knob(IntEnum):
    X = 0      # Position (3 lanes)
    Q = 1      # Orientation quaternion (4 lanes)
    P = 2      # Canonical momentum (3 lanes)
    L = 3      # Angular momentum (3 lanes)
    E = 4      # Energy (1 lane)
    PHI = 5    # Phase (1 lane)
    M = 6      # Mode — NOT physical mass (1 lane)
    H = 7      # History / integrity (1 lane)


class Provenance(IntEnum):
    DERIVED = 0
    DECLARED = 1
    MEASURED = 2
    LAW_DEFINED = 3
    MISSING_LAW = 4
    REFUSED = 5


@dataclass
class State:
    """Decoded body state — root codes in [0,3] for Phi_0..Phi_3."""
    x: Tuple[int, int, int]
    q: Tuple[int, int, int, int]
    p: Tuple[int, int, int]
    l: Tuple[int, int, int]
    energy: int
    phase: int
    mode: int
    history: int


@dataclass
class Hamiltonian:
    h_free: float
    g8_inverse: float


@dataclass
class TemporalClock:
    tick: int
    phase: int
    crossing: int


@dataclass
class KernelView:
    phi: float
    alpha: float
    roots: Tuple[float, float, float, float]
