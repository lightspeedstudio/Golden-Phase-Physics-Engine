/*
 * golden_phase.h — Golden Phase SDK v0.1 Public C ABI
 *
 * Copyright (c) 2026 LightSpeed Studio LLC. All Rights Reserved.
 * Community License: see LICENSE-COMMUNITY.txt
 *
 * Applications never call gphase_kernel directly. This header is the
 * sole public entry point. The runtime contains the kernel.
 *
 * Usage:
 *   1. Start the separately provided runtime. It is not in this repository.
 *   2. Link against goldenphase.dll / libgoldenphase.so
 *   3. Call the functions below
 */

#ifndef GOLDEN_PHASE_H
#define GOLDEN_PHASE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ── Handle types ──────────────────────────────────────────────────── */

typedef uint64_t GP_Engine;
typedef uint64_t GP_Body;

/* ── Result codes ──────────────────────────────────────────────────── */

typedef enum {
    GP_OK               = 0,
    GP_MISSING_LAW      = 1,
    GP_REFUSED           = 2,
    GP_INVALID_BODY      = 3,
    GP_INVALID_STATE     = 4,
    GP_BOUNDARY_REFUSAL  = 5,
    GP_ILLEGAL_TRANSITION = 6,
    GP_NOT_CONNECTED     = 253,
    GP_PROTOCOL_ERROR    = 254,
    GP_INTERNAL_ERROR    = 255
} GP_Result;

/* ── Knob identifiers ─────────────────────────────────────────────── */

typedef enum {
    GP_KNOB_X   = 0,   /* Position (3 lanes) */
    GP_KNOB_Q   = 1,   /* Orientation quaternion (4 lanes) */
    GP_KNOB_P   = 2,   /* Canonical momentum (3 lanes) */
    GP_KNOB_L   = 3,   /* Angular momentum (3 lanes) */
    GP_KNOB_E   = 4,   /* Energy (1 lane) */
    GP_KNOB_PHI = 5,   /* Phase (1 lane) */
    GP_KNOB_M   = 6,   /* Mode — NOT physical mass (1 lane) */
    GP_KNOB_H   = 7    /* History / integrity (1 lane) */
} GP_Knob;

/* ── Provenance tags ──────────────────────────────────────────────── */

typedef enum {
    GP_PROVENANCE_DERIVED     = 0,
    GP_PROVENANCE_DECLARED    = 1,
    GP_PROVENANCE_MEASURED    = 2,
    GP_PROVENANCE_LAW_DEFINED = 3,
    GP_PROVENANCE_MISSING_LAW = 4,
    GP_PROVENANCE_REFUSED     = 5
} GP_Provenance;

/* ── State observation (decoded, not internal packing) ────────────── */

typedef uint8_t GP_Root;   /* 0..3 representing Phi_0 .. Phi_3 */

typedef struct {
    GP_Root x[3];          /* Position roots */
    GP_Root q[4];          /* Orientation quaternion roots */
    GP_Root p[3];          /* Canonical momentum roots */
    GP_Root l[3];          /* Angular momentum roots */
    GP_Root energy;        /* Energy root */
    GP_Root phase;         /* Phase root */
    uint8_t mode;          /* Mode (NOT physical mass) */
    uint8_t history;       /* History / integrity */
} GP_State;

/* ── Hamiltonian observation ──────────────────────────────────────── */

typedef struct {
    double h_free;
    double g8_inverse;
} GP_Hamiltonian;

/* ── Temporal clock ───────────────────────────────────────────────── */

typedef struct {
    uint64_t tick;
    uint8_t  phase;
    uint8_t  crossing;
} GP_TemporalClock;

/* ── Kernel view ──────────────────────────────────────────────────── */

typedef struct {
    double phi;
    double alpha;
    double roots[4];
} GP_KernelView;

/* ── Lifecycle ────────────────────────────────────────────────────── */

GP_Result gp_engine_create(GP_Engine* out_engine);
GP_Result gp_engine_destroy(GP_Engine engine);
GP_Result gp_body_create(GP_Engine engine, GP_Body* out_body);
GP_Result gp_body_remove(GP_Engine engine, GP_Body body);

/* ── Commands (requests, not direct memory mutation) ──────────────── */

GP_Result gp_command_phase(
    GP_Engine engine, GP_Body body,
    uint8_t knob, uint8_t lane, uint8_t phase
);

GP_Result gp_apply_impulse(
    GP_Engine engine, GP_Body body,
    double x, double y, double z
);

GP_Result gp_apply_torque_impulse(
    GP_Engine engine, GP_Body body,
    double x, double y, double z
);

/* ── Simulation ───────────────────────────────────────────────────── */

GP_Result gp_tick(GP_Engine engine, uint32_t ticks, uint64_t* out_tick);

/* ── Observations ─────────────────────────────────────────────────── */

GP_Result gp_body_state(GP_Engine engine, GP_Body body, GP_State* out);
GP_Result gp_read_position(GP_Engine engine, GP_Body body, uint8_t out_roots[3]);
GP_Result gp_read_hamiltonian(GP_Engine engine, GP_Body body, GP_Hamiltonian* out);

/* ── Temporal API ─────────────────────────────────────────────────── */

GP_Result gp_temporal_clock(GP_Engine engine, GP_TemporalClock* out);
GP_Result gp_temporal_seconds(GP_Engine engine, double* out_seconds);

/* ── Science: Electromagnetic ─────────────────────────────────────── */

GP_Result gp_em_charge_force(
    int32_t q_a, int32_t q_b,
    uint8_t g_a, uint8_t g_b,
    int8_t* out_force
);

/* ── Kernel ───────────────────────────────────────────────────────── */

GP_Result gp_kernel_view(GP_KernelView* out);

/* ── Error detail ─────────────────────────────────────────────────── */

/* Returns C string valid until next call. NULL on failure. */
const char* gp_last_missing_feature(void);

/* ── Version ──────────────────────────────────────────────────────── */

const char* gp_runtime_version(void);
uint32_t    gp_abi_version(void);
uint32_t    gp_kernel_schema_version(void);

/*
 * Deliberately absent from the public API:
 *
 *   set_raw_position()
 *   set_raw_momentum()
 *   write_state_memory()
 *   set_mass()
 *   apply_newton_force()
 *
 * Velocity is dH/d(canonical momentum), never P/m.
 * State changes through commands and the kernel setter only.
 */

#ifdef __cplusplus
}
#endif

#endif /* GOLDEN_PHASE_H */
