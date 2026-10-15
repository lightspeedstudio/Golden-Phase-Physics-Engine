"""
Golden Phase SDK for Python — v0.1

Thin wrapper around the Golden Phase C ABI. Connects to
gphase_runtime via goldenphase.dll / libgoldenphase.so.

Usage:
    from goldenphase import Engine

    engine = Engine()
    body = engine.create_body()
    engine.tick(10)
    state = engine.state(body)
    print(state.x, state.p, state.energy)
"""

from goldenphase.engine import Engine, GoldenPhaseError
from goldenphase.types import (
    State, Hamiltonian, TemporalClock, KernelView,
    Knob, Provenance, Result,
)

__version__ = "0.1.0"
__all__ = [
    "Engine",
    "GoldenPhaseError",
    "State",
    "Hamiltonian",
    "TemporalClock",
    "KernelView",
    "Knob",
    "Provenance",
    "Result",
]
