@tool
extends EditorPlugin
## Golden Phase plugin — loads the SDK and provides GoldenPhaseEngine.

func _enter_tree() -> void:
	add_autoload_singleton("GoldenPhase", "res://addons/golden_phase/gp_engine.gd")
	print("[GoldenPhase] Plugin enabled — SDK v0.1")

func _exit_tree() -> void:
	remove_autoload_singleton("GoldenPhase")
