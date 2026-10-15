"""
gravity — Golden Phase SDK: gravitational pair attraction.

Demonstrates the kernel inverse-square path:
  PHASE_ABS → PHASE_SQ → PHASE_INV → PHASE_MUL
with LAW42 supplying ternary pair direction.
No host 1/r² or F=ma integration.
"""

from goldenphase import Engine

engine = Engine()
a = engine.create_body()
b = engine.create_body()

print("Gravitational pair attraction via LAW42")
print("=" * 50)

sa = engine.state(a)
sb = engine.state(b)
print(f"A position roots: {sa.x}")
print(f"B position roots: {sb.x}")

engine.apply_impulse(a, 0.0, 0.0, 1.0)

for i in range(10):
    tick = engine.tick(10)
    sa = engine.state(a)
    sb = engine.state(b)
    ha = engine.hamiltonian(a)
    print(
        f"Crossing {i+1}: "
        f"A.X={sa.x} A.P={sa.p} H_free={ha.h_free:.6f}"
    )

print(f"\nElapsed readout: {engine.temporal_seconds():.1f} s")
