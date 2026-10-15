# Golden Phase SDK — Architecture

## Boundary Diagram

```
Application (game, simulation, research tool)
    ↓
Golden Phase SDK  (goldenphase.dll / libgoldenphase.so)
    ↓ TCP (v0.1) or Named Pipe / UDS (future)
Runtime  (not in this repository)
    ↓
one setter commit, and one clock advance when that commit publishes
```

## The Key Rule

**Applications never call `gphase_kernel` directly.**
`Engine::step` is one setter commit and, when that commit publishes, one clock advance. A tick is not `0.1 s`.

This repository does not contain the runtime or the kernel source.
The SDK DLL is a thin IPC client that contains no kernel code. It
connects only when a separately provided runtime is running.

## What Lives Where

| Component | Contains | Distributed |
|-----------|----------|-------------|
| runtime | Kernel, Engine, laws, setter | Not in this repository |
| `goldenphase` client | IPC client, C ABI, handle management | Source. `cargo build` produces the DLL |
| `golden_phase.h` | Type definitions, function declarations | Source |
| Python / C# | Wrappers over the C ABI | Source |
| Godot addon | Knob ids match the header. GDScript does not call the DLL | Source |

## Protocol

The SDK communicates with the runtime via a length-prefixed binary
protocol. Each message:

```
[u32 payload_len][u32 request_id][u16 msg_type][payload…]
```

Responses include a status code:

```
[u32 payload_len][u32 request_id][u16 status_code][payload…]
```

The protocol is transport-agnostic. v0.1 uses TCP on `127.0.0.1:31415`.
Named pipes (Windows) and Unix domain sockets (Linux/macOS) are planned.

## One Protocol, Two Transports

```python
# Local
gp = GoldenPhase.local()

# Cloud (future)
gp = GoldenPhase.cloud(api_key="...")
```

Same protocol, same API. The developer does not rewrite their project.

## State Observation

The SDK never exposes internal packed state (`StateBits256`).
Developers receive decoded observations through `GP_State`:

```c
typedef struct {
    GP_Root x[3];      // Position roots
    GP_Root q[4];      // Orientation quaternion roots
    GP_Root p[3];      // Canonical momentum roots
    GP_Root l[3];      // Angular momentum roots
    GP_Root energy;    // Energy root
    GP_Root phase;     // Phase root
    uint8_t mode;      // Mode (NOT physical mass)
    uint8_t history;   // History lane. Not h_free.
} GP_State;
```

`gp_read_hamiltonian` returns `h_free` and `g8_inverse`. That readout
is not `GP_State.history`.

## Version Independence

```
Golden Phase SDK:     0.1.0
Golden Phase ABI:     1
Kernel Schema:        1
```

The ABI and schema are versioned independently from the SDK and
runtime. A newer runtime with ABI 1 works with any SDK built for
ABI 1.
