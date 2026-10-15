# Golden Phase SDK — Science API Reference

## Domain Namespaces

All science observations route through the same engine. The namespaces
group related operations without creating separate objects.

| Prefix | Domain | Status |
|--------|--------|--------|
| `gp_temporal_*` | Time / crossing | Available |
| `gp_spatial_*` | Position / roots | Available |
| `gp_em_*` | Electromagnetic | Available |
| `gp_gravity_*` | Gravitational | Available (LAW42 pair) |
| `gp_quantum_*` | Quantum | MissingLaw |
| `gp_chem_*` | Chemistry | MissingLaw |
| `gp_bio_*` | Biology | Available (sheath transition) |

## Temporal

```c
GP_Result gp_temporal_clock(GP_Engine engine, GP_TemporalClock* out);
GP_Result gp_temporal_seconds(GP_Engine engine, double* out);
```

- One tick is one setter commit. When that commit publishes, the clock advances once.
- A tick is not `0.1 s`, and this page does not define a light-tick distance.

## Electromagnetic

```c
GP_Result gp_em_charge_force(int32_t q_a, int32_t q_b,
                              uint8_t g_a, uint8_t g_b,
                              int8_t* out_force);
```

Coulomb-labelled pair mapping:
- Same-sign → repulsion mapping
- Opposite-sign → attraction mapping

This tests the declared Coulomb interpretation of the LAW42/dual tables;
it does not prove that LAW42 is uniquely Coulomb.

## Gravitational

Kernel inverse-square path (no host `1/r²`):

1. `r_K = PHASE_ABS(X_A, X_B)` — root distance
2. `r_K² = PHASE_SQ(r_K)` — squared root
3. `r_K^{-2} = PHASE_INV(r_K²)` — canonical inverse
4. `F_K = PHASE_MUL(F_0, r_K^{-2})` — inverse-square magnitude
5. LAW42 supplies ternary pair direction
6. Zero-distance inversion refuses

No host `/`, `powf`, `F=ma`, or side solver.

## Metatron Domain Parameters `(m_D, f_D)`

| Domain | m_D | f_D |
|--------|-----|-----|
| Electromagnetic | 1 | α |
| Quantum | α | 1 |
| Biological | α | 1 |
| Chemical | 1 | 1 |
| Gravitational | α | α |
| Temporal | π/2 | 1 |
| Spatial | 0 | 1 |

Where α = fine-structure constant (exact from the kernel).

## MissingLaw Reporting

Unsupported features return `GP_MISSING_LAW` with a machine-readable
feature name:

| Feature | Status |
|---------|--------|
| `MassEnergyLaw` | OPEN — candidate readout, no mass carrier |
| `MetreLaw` | OPEN — GAP A2, absolute placement |
| `QuantumAmplitudeLaw` | OPEN — storage layout, not measurement |
| `PairReaction` | OPEN — W3 parked |
| `LiftEvolution` | OPEN — Xi advance refused |
| `FieldCoupling` | OPEN — sampling available, coupling not |
| `BioFieldEnergy` | OPEN — test measurement only |

The SDK never silently substitutes conventional physics.
