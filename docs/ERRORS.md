# Golden Phase SDK — Error Handling

## Every Result Includes Provenance

```c
typedef enum {
    GP_PROVENANCE_DERIVED,      // Mathematically proved
    GP_PROVENANCE_DECLARED,     // Owner decision, not derived
    GP_PROVENANCE_MEASURED,     // Empirical measurement
    GP_PROVENANCE_LAW_DEFINED,  // Law exists, not yet verified
    GP_PROVENANCE_MISSING_LAW,  // No derived law
    GP_PROVENANCE_REFUSED       // Architectural constraint
} GP_Provenance;
```

## Error Flow

1. Call returns `GP_Result` (never crashes, never falls back)
2. If `GP_MISSING_LAW`: call `gp_last_missing_feature()` for the name
3. If `GP_REFUSED`: the operation is architecturally absent

## Feature Status Lifecycle

```
Proposed → LawDefined → Verified → Production
Refused   (architectural constraint; will never derive a law)
```

## Current Feature Census (SDK v0.1)

### Refused (5)

| Feature | Reason |
|---------|--------|
| DirectKinematicWrite | Z' = J∇H; force/impulse/velocity categorically absent |
| ContinuumDynamics | Catalogue search available; tick dynamics intentionally absent |
| SoftBodyDeformation | No per-vertex state family in Z |
| Joints | No derived joint constraint; LAW19 is canonical J_K |
| BondEnergy | No kernel bond energy quantity; delta_nu is exact |

### LawDefined (9)

ContactResolution, PhysicalUnits, ThermalEnergyLaw, MagnetismLaw,
AcousticLaw, PlanckBoundary, VoxelVolumeCalibration, BioTransitionLaw,
GravitationalCoupling

### Proposed / Open (7)

MassEnergyLaw, MetreLaw, PairReaction, BioFieldEnergy,
LiftEvolution, QuantumAmplitudeLaw, FieldCoupling

## Fail Loudly, Never Fall Back

A missing mechanism returns a typed error (`GP_MISSING_LAW`) and
leaves the world byte-identical. It does not quietly route to a
second implementation, substitute Newtonian physics, or degrade
gracefully.
