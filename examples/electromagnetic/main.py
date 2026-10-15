"""
electromagnetic — Golden Phase SDK: EM charge force mapping.

Demonstrates the Coulomb-labelled pair mapping from the LAW42/dual
tables. Same-sign → repulsion, opposite-sign → attraction.

This tests the declared Coulomb interpretation; it does not prove
that LAW42 is uniquely Coulomb.
"""

from goldenphase import Engine

print("Electromagnetic pair mapping via LAW42")
print("=" * 50)

# Root indices: Phi_1=1, Phi_2=2 (distinct roots)
PHI1, PHI2 = 1, 2

# Same sign (+1, +1) → repulsion
repel = Engine.em_charge_force(+1, +1, PHI1, PHI2)
print(f"Same sign    (+1, +1) @ (Φ1, Φ2): force byte = {repel}")

# Opposite sign (+1, -1) → attraction
attract = Engine.em_charge_force(+1, -1, PHI1, PHI2)
print(f"Opposite sign(+1, -1) @ (Φ1, Φ2): force byte = {attract}")

assert repel != 0 and attract != 0, "Force bytes must be nonzero"
assert (repel > 0) != (attract > 0), "Signs must differ"
print("\n✓ Same-sign repels, opposite-sign attracts")

print(f"\nKernel view:")
kv = Engine.kernel_view()
print(f"  phi   = {kv.phi}")
print(f"  alpha = {kv.alpha}")
print(f"  roots = {kv.roots}")
