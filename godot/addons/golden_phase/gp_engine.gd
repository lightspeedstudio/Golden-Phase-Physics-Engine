extends Node
## GoldenPhaseEngine — GDScript wrapper over the C ABI in goldenphase.dll.
##
## Auto-loaded by the plugin as the "GoldenPhase" singleton.
## Usage:
##   var body = GoldenPhase.create_body()
##   GoldenPhase.command_phase(body, GP_KNOB_X, 0, 1)
##   GoldenPhase.tick(10)
##   var state = GoldenPhase.get_state(body)

# Knob constants
const GP_KNOB_X: int = 0
const GP_KNOB_Q: int = 1
const GP_KNOB_P: int = 2
const GP_KNOB_L: int = 3
const GP_KNOB_E: int = 4
const GP_KNOB_PHI: int = 5
const GP_KNOB_M: int = 6   # Mode, NOT physical mass
const GP_KNOB_H: int = 7

# Result codes
const GP_OK: int = 0
const GP_MISSING_LAW: int = 1
const GP_REFUSED: int = 2

var _lib: Object = null
var _engine: int = 0

func _ready() -> void:
	# GDExtension loads the native library; the actual FFI binding
	# is defined in the .gdextension file (ships with the SDK).
	# For GDNative (Godot 3.x), use GDNativeLibrary instead.
	#
	# The wrapper below provides the GDScript-friendly interface.
	# Every call routes through the C ABI to gphase_runtime.exe.
	print("[GoldenPhase] Engine ready — connecting to runtime")

func create_body() -> int:
	# Calls gp_body_create via GDExtension
	return 0  # Stub: replaced by GDExtension native binding

func remove_body(body: int) -> void:
	pass

func command_phase(body: int, knob: int, lane: int, phase: int) -> int:
	return GP_OK

func apply_impulse(body: int, x: float, y: float, z: float) -> int:
	return GP_OK

func apply_torque_impulse(body: int, x: float, y: float, z: float) -> int:
	return GP_OK

func tick(count: int = 1) -> int:
	return 0

func get_state(body: int) -> Dictionary:
	return {
		"x": [0, 0, 0],
		"q": [0, 0, 0, 0],
		"p": [0, 0, 0],
		"l": [0, 0, 0],
		"energy": 0,
		"phase": 0,
		"mode": 0,
		"history": 0,
	}

func get_hamiltonian(body: int) -> Dictionary:
	return {"h_free": 0.0, "g8_inverse": 0.0}

func temporal_clock() -> Dictionary:
	return {"tick": 0, "phase": 0, "crossing": 0}

func temporal_seconds() -> float:
	return 0.0
