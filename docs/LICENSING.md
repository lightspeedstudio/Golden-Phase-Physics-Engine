# Golden Phase SDK — Licensing

## License Tiers

### Golden Phase Community

- **Free**
- Individuals, inventors, students
- Noncommercial experimentation
- Full SDK access
- Community support

### Golden Phase Academic

- Classrooms, research labs, institution deployments
- Paid academic license
- Priority support for institutions

### Golden Phase Commercial

- Companies and commercial software
- Production use
- Annual license
- Technical support

### Golden Phase Enterprise / OEM

- On-premise and private runtime server
- Redistribution rights
- Embedded runtime
- Support / SLA
- Custom integrations

## What Is Distributed

The same client source ships to all tiers. The license determines deployment rights. This repository does not include a runtime binary.

| Artifact | In this repository | Source |
|----------|--------------------|--------|
| `gphase_runtime` | No. Provided separately | No |
| `goldenphase` client | Yes | Yes |
| `gphase_protocol` | Yes | Yes |
| `golden_phase.h` | Yes | Yes |
| Python / C# SDK | Yes | Yes |
| Godot addon | Yes. GDScript does not call the DLL | Yes |
| Examples | Yes | Yes |
| `gphase-verify` | No | No |
| Documentation | Yes | Yes |

## What Is Protected

These remain private:

- Kernel source code
- Law derivation source
- Research tooling and law finder
- Private law databases
- Build / signing keys
- Unpublished domain mappings

## Runtime

The runtime that executes a tick is not in this repository. Clients need that runtime, obtained separately, before `gp_engine_create` can connect.
