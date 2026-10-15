"""
hello_phase — Golden Phase SDK: minimal example.

Creates an engine, spawns a body, ticks, and prints state.
"""

from goldenphase import Engine

print("Golden Phase SDK — Hello Phase")
print(f"Runtime: {Engine.runtime_version()}")
print(f"ABI:     {Engine.abi_version()}")
print()

engine = Engine()
body = engine.create_body()
print(f"Created body {body}")

state = engine.state(body)
print(f"Initial state: X={state.x}  Q={state.q}  P={state.p}")

tick = engine.tick(10)
print(f"After 10 ticks (tick={tick}):")

state = engine.state(body)
print(f"  X={state.x}  P={state.p}  E={state.energy}  Φ={state.phase}")
print(f"  Elapsed: {engine.temporal_seconds():.1f} s")
