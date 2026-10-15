# Golden Phase SDK — API Reference

## ABI Version: 1 | Kernel Schema: 1 | SDK: 0.1.0

---

## Handle Types

| Type | Description |
|------|-------------|
| `GP_Engine` | `uint64` — engine instance handle |
| `GP_Body` | `uint64` — body handle within an engine |

## Result Codes

| Code | Name | Meaning |
|------|------|---------|
| 0 | `GP_OK` | Success |
| 1 | `GP_MISSING_LAW` | Operation has no derived law — use `gp_last_missing_feature()` |
| 2 | `GP_REFUSED` | Architectural refusal (e.g. direct kinematic write) |
| 3 | `GP_INVALID_BODY` | Body handle does not exist |
| 4 | `GP_INVALID_STATE` | State is non-finite or unrepresentable |
| 5 | `GP_BOUNDARY_REFUSAL` | Boundary condition refused the operation |
| 6 | `GP_ILLEGAL_TRANSITION` | Requested root transition is not adjacent |
| 253 | `GP_NOT_CONNECTED` | Runtime is not running |

## Lifecycle

```c
GP_Result gp_engine_create(GP_Engine* out_engine);
GP_Result gp_engine_destroy(GP_Engine engine);
GP_Result gp_body_create(GP_Engine engine, GP_Body* out_body);
GP_Result gp_body_remove(GP_Engine engine, GP_Body body);
```

## Commands

Commands are requests, not direct memory mutation. The kernel setter
decides the next state.

```c
GP_Result gp_command_phase(engine, body, knob, lane, phase);
GP_Result gp_apply_impulse(engine, body, x, y, z);
GP_Result gp_apply_torque_impulse(engine, body, x, y, z);
```

## Simulation

```c
GP_Result gp_tick(GP_Engine engine, uint32_t ticks, uint64_t* out_tick);
```

One `gp_tick` request is one setter commit. When that commit publishes, the clock advances once. A tick is not `0.1 s`.

`gp_read_hamiltonian` fills `h_free` and `g8_inverse`. That pair is not the History lane. History is the `history` field on `GP_State` and `GP_KNOB_H`.

## Observations

```c
GP_Result gp_body_state(engine, body, GP_State* out);
GP_Result gp_read_position(engine, body, uint8_t out_roots[3]);
GP_Result gp_read_hamiltonian(engine, body, GP_Hamiltonian* out);
GP_Result gp_temporal_clock(engine, GP_TemporalClock* out);
GP_Result gp_temporal_seconds(engine, double* out);
```

## Science Domains

```c
GP_Result gp_em_charge_force(q_a, q_b, g_a, g_b, int8_t* out_force);
```

## Kernel

```c
GP_Result gp_kernel_view(GP_KernelView* out);
```

Returns `phi`, `alpha`, and the four roots of `K_G(u) = u^4 - phi*u^2 + alpha`.

## Error Detail

```c
const char* gp_last_missing_feature(void);
```

Returns the feature name from the most recent `GP_MISSING_LAW` error.
Machine-readable values include: `MassEnergyLaw`, `MetreLaw`,
`QuantumAmplitudeLaw`, `PairReaction`, `LiftEvolution`.

## Version

```c
const char* gp_runtime_version(void);
uint32_t    gp_abi_version(void);
uint32_t    gp_kernel_schema_version(void);
```

## Deliberately Absent

The following are **never** exposed:

- `set_raw_position()` / `set_raw_momentum()` / `write_state_memory()`
- `set_mass()` / `apply_newton_force()`
- Direct state memory access
- Internal packing format (`StateBits256`)

Velocity is `dH/d(canonical momentum)`, never `P/m`.
State changes through commands and the kernel setter only.
