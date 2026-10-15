"""
hamiltonian — Golden Phase SDK: Hamiltonian observation.

Demonstrates that H_free is conserved in the free sector and
that velocity is dH/dPi, never P/m.
"""

from goldenphase import Engine

engine = Engine()
body = engine.create_body()

h0 = engine.hamiltonian(body)
print(f"Initial H_free = {h0.h_free}")
print(f"g8_inverse     = {h0.g8_inverse}")

engine.tick(20)

h1 = engine.hamiltonian(body)
print(f"After 20 ticks:")
print(f"  H_free = {h1.h_free}")
print(f"  Conserved: {abs(h1.h_free - h0.h_free) < 1e-12}")

kv = Engine.kernel_view()
print(f"\nKernel: phi={kv.phi:.15f}  alpha={kv.alpha:.10f}")
print(f"  Roots: {kv.roots}")
for r in kv.roots:
    kg = r**4 - kv.phi * r**2 + kv.alpha
    print(f"  K_G({r:.6f}) = {kg:.2e}")
