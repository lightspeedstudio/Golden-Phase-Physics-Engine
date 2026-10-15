"""Golden Phase Engine — Python wrapper over the C ABI."""

import ctypes
import os
import sys
from pathlib import Path
from typing import Optional

from goldenphase.types import (
    Hamiltonian, KernelView, Knob, Result, State, TemporalClock,
)


class GoldenPhaseError(Exception):
    """Raised when a Golden Phase API call fails."""

    def __init__(self, result: Result, feature: str = "", reason: str = ""):
        self.result = result
        self.feature = feature
        self.reason = reason
        super().__init__(
            f"GP error {result.name}"
            + (f": {feature} — {reason}" if feature else "")
        )


# ── C struct mirrors ──────────────────────────────────────────────────

class _GP_State(ctypes.Structure):
    _fields_ = [
        ("x", ctypes.c_uint8 * 3),
        ("q", ctypes.c_uint8 * 4),
        ("p", ctypes.c_uint8 * 3),
        ("l", ctypes.c_uint8 * 3),
        ("energy", ctypes.c_uint8),
        ("phase", ctypes.c_uint8),
        ("mode", ctypes.c_uint8),
        ("history", ctypes.c_uint8),
    ]


class _GP_Hamiltonian(ctypes.Structure):
    _fields_ = [
        ("h_free", ctypes.c_double),
        ("g8_inverse", ctypes.c_double),
    ]


class _GP_TemporalClock(ctypes.Structure):
    _fields_ = [
        ("tick", ctypes.c_uint64),
        ("phase", ctypes.c_uint8),
        ("crossing", ctypes.c_uint8),
    ]


class _GP_KernelView(ctypes.Structure):
    _fields_ = [
        ("phi", ctypes.c_double),
        ("alpha", ctypes.c_double),
        ("roots", ctypes.c_double * 4),
    ]


# ── Library loader ────────────────────────────────────────────────────

def _find_library() -> ctypes.CDLL:
    """Locate and load goldenphase.dll / libgoldenphase.so."""
    if sys.platform == "win32":
        names = ["goldenphase.dll"]
    elif sys.platform == "darwin":
        names = ["libgoldenphase.dylib"]
    else:
        names = ["libgoldenphase.so"]

    search = [
        Path(__file__).parent,
        Path(__file__).parent.parent,
        Path.cwd(),
    ]
    env_path = os.environ.get("GP_LIB_PATH")
    if env_path:
        search.insert(0, Path(env_path))

    for d in search:
        for name in names:
            p = d / name
            if p.exists():
                return ctypes.CDLL(str(p))

    # Fall back to system search
    return ctypes.CDLL(names[0])


_lib: Optional[ctypes.CDLL] = None


def _get_lib() -> ctypes.CDLL:
    global _lib
    if _lib is None:
        _lib = _find_library()
        _setup_signatures(_lib)
    return _lib


def _setup_signatures(lib: ctypes.CDLL) -> None:
    """Declare C function signatures for type safety."""
    lib.gp_engine_create.argtypes = [ctypes.POINTER(ctypes.c_uint64)]
    lib.gp_engine_create.restype = ctypes.c_uint32

    lib.gp_engine_destroy.argtypes = [ctypes.c_uint64]
    lib.gp_engine_destroy.restype = ctypes.c_uint32

    lib.gp_body_create.argtypes = [ctypes.c_uint64, ctypes.POINTER(ctypes.c_uint64)]
    lib.gp_body_create.restype = ctypes.c_uint32

    lib.gp_body_remove.argtypes = [ctypes.c_uint64, ctypes.c_uint64]
    lib.gp_body_remove.restype = ctypes.c_uint32

    lib.gp_command_phase.argtypes = [
        ctypes.c_uint64, ctypes.c_uint64,
        ctypes.c_uint8, ctypes.c_uint8, ctypes.c_uint8,
    ]
    lib.gp_command_phase.restype = ctypes.c_uint32

    lib.gp_apply_impulse.argtypes = [
        ctypes.c_uint64, ctypes.c_uint64,
        ctypes.c_double, ctypes.c_double, ctypes.c_double,
    ]
    lib.gp_apply_impulse.restype = ctypes.c_uint32

    lib.gp_apply_torque_impulse.argtypes = [
        ctypes.c_uint64, ctypes.c_uint64,
        ctypes.c_double, ctypes.c_double, ctypes.c_double,
    ]
    lib.gp_apply_torque_impulse.restype = ctypes.c_uint32

    lib.gp_tick.argtypes = [
        ctypes.c_uint64, ctypes.c_uint32,
        ctypes.POINTER(ctypes.c_uint64),
    ]
    lib.gp_tick.restype = ctypes.c_uint32

    lib.gp_body_state.argtypes = [
        ctypes.c_uint64, ctypes.c_uint64,
        ctypes.POINTER(_GP_State),
    ]
    lib.gp_body_state.restype = ctypes.c_uint32

    lib.gp_read_hamiltonian.argtypes = [
        ctypes.c_uint64, ctypes.c_uint64,
        ctypes.POINTER(_GP_Hamiltonian),
    ]
    lib.gp_read_hamiltonian.restype = ctypes.c_uint32

    lib.gp_temporal_clock.argtypes = [
        ctypes.c_uint64,
        ctypes.POINTER(_GP_TemporalClock),
    ]
    lib.gp_temporal_clock.restype = ctypes.c_uint32

    lib.gp_temporal_seconds.argtypes = [
        ctypes.c_uint64,
        ctypes.POINTER(ctypes.c_double),
    ]
    lib.gp_temporal_seconds.restype = ctypes.c_uint32

    lib.gp_kernel_view.argtypes = [ctypes.POINTER(_GP_KernelView)]
    lib.gp_kernel_view.restype = ctypes.c_uint32

    lib.gp_em_charge_force.argtypes = [
        ctypes.c_int32, ctypes.c_int32,
        ctypes.c_uint8, ctypes.c_uint8,
        ctypes.POINTER(ctypes.c_int8),
    ]
    lib.gp_em_charge_force.restype = ctypes.c_uint32

    lib.gp_runtime_version.argtypes = []
    lib.gp_runtime_version.restype = ctypes.c_char_p

    lib.gp_abi_version.argtypes = []
    lib.gp_abi_version.restype = ctypes.c_uint32

    lib.gp_kernel_schema_version.argtypes = []
    lib.gp_kernel_schema_version.restype = ctypes.c_uint32

    lib.gp_last_missing_feature.argtypes = []
    lib.gp_last_missing_feature.restype = ctypes.c_char_p


def _check(result_code: int) -> None:
    r = Result(result_code)
    if r != Result.OK:
        lib = _get_lib()
        feat = lib.gp_last_missing_feature()
        feature = feat.decode("utf-8") if feat else ""
        raise GoldenPhaseError(r, feature=feature)


# ── Engine class ──────────────────────────────────────────────────────

class Engine:
    """
    Golden Phase engine.

    Usage:
        engine = Engine()
        body = engine.create_body()
        engine.command_phase(body, Knob.X, 0, 1)
        engine.tick(10)
        state = engine.state(body)
    """

    def __init__(self) -> None:
        lib = _get_lib()
        handle = ctypes.c_uint64(0)
        _check(lib.gp_engine_create(ctypes.byref(handle)))
        self._handle = handle.value
        self._lib = lib

    def __del__(self) -> None:
        if hasattr(self, "_handle") and self._handle:
            self._lib.gp_engine_destroy(self._handle)

    def create_body(self) -> int:
        body = ctypes.c_uint64(0)
        _check(self._lib.gp_body_create(self._handle, ctypes.byref(body)))
        return body.value

    def remove_body(self, body: int) -> None:
        _check(self._lib.gp_body_remove(self._handle, body))

    def command_phase(self, body: int, knob: Knob, lane: int, phase: int) -> None:
        _check(self._lib.gp_command_phase(
            self._handle, body, int(knob), lane, phase
        ))

    def apply_impulse(self, body: int, x: float, y: float, z: float) -> None:
        _check(self._lib.gp_apply_impulse(self._handle, body, x, y, z))

    def apply_torque_impulse(self, body: int, x: float, y: float, z: float) -> None:
        _check(self._lib.gp_apply_torque_impulse(self._handle, body, x, y, z))

    def tick(self, count: int = 1) -> int:
        out = ctypes.c_uint64(0)
        _check(self._lib.gp_tick(self._handle, count, ctypes.byref(out)))
        return out.value

    def state(self, body: int) -> State:
        s = _GP_State()
        _check(self._lib.gp_body_state(self._handle, body, ctypes.byref(s)))
        return State(
            x=tuple(s.x),
            q=tuple(s.q),
            p=tuple(s.p),
            l=tuple(s.l),
            energy=s.energy,
            phase=s.phase,
            mode=s.mode,
            history=s.history,
        )

    def hamiltonian(self, body: int) -> Hamiltonian:
        h = _GP_Hamiltonian()
        _check(self._lib.gp_read_hamiltonian(
            self._handle, body, ctypes.byref(h)
        ))
        return Hamiltonian(h_free=h.h_free, g8_inverse=h.g8_inverse)

    def temporal_clock(self) -> TemporalClock:
        tc = _GP_TemporalClock()
        _check(self._lib.gp_temporal_clock(self._handle, ctypes.byref(tc)))
        return TemporalClock(
            tick=tc.tick, phase=tc.phase, crossing=tc.crossing
        )

    def temporal_seconds(self) -> float:
        out = ctypes.c_double(0.0)
        _check(self._lib.gp_temporal_seconds(
            self._handle, ctypes.byref(out)
        ))
        return out.value

    @staticmethod
    def em_charge_force(q_a: int, q_b: int, g_a: int, g_b: int) -> int:
        lib = _get_lib()
        out = ctypes.c_int8(0)
        _check(lib.gp_em_charge_force(q_a, q_b, g_a, g_b, ctypes.byref(out)))
        return out.value

    @staticmethod
    def kernel_view() -> KernelView:
        lib = _get_lib()
        kv = _GP_KernelView()
        _check(lib.gp_kernel_view(ctypes.byref(kv)))
        return KernelView(
            phi=kv.phi,
            alpha=kv.alpha,
            roots=tuple(kv.roots),
        )

    @staticmethod
    def runtime_version() -> str:
        lib = _get_lib()
        v = lib.gp_runtime_version()
        return v.decode("utf-8") if v else "unknown"

    @staticmethod
    def abi_version() -> int:
        return _get_lib().gp_abi_version()

    @staticmethod
    def kernel_schema_version() -> int:
        return _get_lib().gp_kernel_schema_version()
