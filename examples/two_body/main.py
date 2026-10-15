"""
two_body — Golden Phase SDK: two-body interaction.

Creates two bodies, applies impulses, and observes LAW42 antisymmetry.
"""

from goldenphase import Engine, Knob

engine = Engine()
a = engine.create_body()
b = engine.create_body()
print(f"Body A={a}, Body B={b}")

engine.apply_impulse(a, 1.0, 0.0, 0.0)
engine.apply_impulse(b, -1.0, 0.0, 0.0)

for i in range(5):
    tick = engine.tick(10)
    sa = engine.state(a)
    sb = engine.state(b)
    print(f"Crossing {i+1} (tick={tick}):")
    print(f"  A: X={sa.x}  P={sa.p}")
    print(f"  B: X={sb.x}  P={sb.p}")
