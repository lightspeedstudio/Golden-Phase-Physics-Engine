//! Golden Phase SDK — thin C-ABI client.
//!
//! This crate compiles to `goldenphase.dll` / `libgoldenphase.so`.
//! It connects to `gphase_runtime` via TCP and exposes a stable C ABI.
//! **Contains no kernel code.**
//!
//! ```text
//!   Application
//!       ↓
//!   goldenphase.dll  (this crate)
//!       ↓ TCP
//!   gphase_runtime.exe  (kernel lives here)
//! ```

use gphase_protocol::*;
use std::io::Cursor;
use std::net::TcpStream;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

// ── Types matching golden_phase.h ────────────────────────────────────

pub type GpEngine = u64;
pub type GpBody = u64;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpResult {
    Ok = 0,
    MissingLaw = 1,
    Refused = 2,
    InvalidBody = 3,
    InvalidState = 4,
    BoundaryRefusal = 5,
    IllegalTransition = 6,
    NotConnected = 253,
    ProtocolError = 254,
    InternalError = 255,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GpStateC {
    pub x: [u8; 3],
    pub q: [u8; 4],
    pub p: [u8; 3],
    pub l: [u8; 3],
    pub energy: u8,
    pub phase: u8,
    pub mode: u8,
    pub history: u8,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GpHamiltonianC {
    pub h_free: f64,
    pub g8_inverse: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GpTemporalClockC {
    pub tick: u64,
    pub phase: u8,
    pub crossing: u8,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GpKernelViewC {
    pub phi: f64,
    pub alpha: f64,
    pub roots: [f64; 4],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GpProvenanceC {
    Derived = 0,
    Declared = 1,
    Measured = 2,
    LawDefined = 3,
    MissingLaw = 4,
    Refused = 5,
}

// ── Connection state ─────────────────────────────────────────────────

static REQ_COUNTER: AtomicU32 = AtomicU32::new(1);

struct Connection {
    stream: TcpStream,
}

static CONN: Mutex<Option<Connection>> = Mutex::new(None);

fn result_from_status(s: StatusCode) -> GpResult {
    match s {
        StatusCode::Ok => GpResult::Ok,
        StatusCode::MissingLaw => GpResult::MissingLaw,
        StatusCode::Refused => GpResult::Refused,
        StatusCode::InvalidBody => GpResult::InvalidBody,
        StatusCode::InvalidState => GpResult::InvalidState,
        StatusCode::BoundaryRefusal => GpResult::BoundaryRefusal,
        StatusCode::IllegalTransition => GpResult::IllegalTransition,
        StatusCode::NotConnected => GpResult::NotConnected,
        StatusCode::ProtocolError => GpResult::ProtocolError,
        StatusCode::InternalError => GpResult::InternalError,
    }
}

fn ensure_connected() -> Result<(), GpResult> {
    let mut guard = CONN.lock().map_err(|_| GpResult::InternalError)?;
    if guard.is_some() {
        return Ok(());
    }
    let port = std::env::var("GP_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(RUNTIME_DEFAULT_PORT);
    let addr = format!("127.0.0.1:{port}");
    match TcpStream::connect(&addr) {
        Ok(s) => {
            s.set_nodelay(true).ok();
            *guard = Some(Connection { stream: s });
            Ok(())
        }
        Err(_) => Err(GpResult::NotConnected),
    }
}

fn call(msg: MsgType, payload: &[u8]) -> Result<(StatusCode, Vec<u8>), GpResult> {
    ensure_connected()?;
    let mut guard = CONN.lock().map_err(|_| GpResult::InternalError)?;
    let conn = guard.as_mut().ok_or(GpResult::NotConnected)?;
    let rid = REQ_COUNTER.fetch_add(1, Ordering::Relaxed);
    write_request(&mut conn.stream, rid, msg, payload).map_err(|_| GpResult::NotConnected)?;
    let (remaining, _resp_id, status_raw) =
        read_frame_header(&mut conn.stream).map_err(|_| GpResult::NotConnected)?;
    let resp_payload =
        read_payload(&mut conn.stream, remaining).map_err(|_| GpResult::NotConnected)?;
    Ok((StatusCode::from_u16(status_raw), resp_payload))
}

// ── C ABI: Lifecycle ─────────────────────────────────────────────────

/// Connect to the Golden Phase runtime and create an engine.
#[no_mangle]
pub extern "C" fn gp_engine_create(out_engine: *mut GpEngine) -> GpResult {
    let (status, payload) = match call(MsgType::EngineCreate, &[]) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match read_u64(&mut cur) {
        Ok(id) => {
            unsafe { *out_engine = id };
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

/// Destroy an engine.
#[no_mangle]
pub extern "C" fn gp_engine_destroy(engine: GpEngine) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    match call(MsgType::EngineDestroy, &buf) {
        Ok((s, _)) => result_from_status(s),
        Err(e) => e,
    }
}

/// Create a body in the engine.
#[no_mangle]
pub extern "C" fn gp_body_create(engine: GpEngine, out_body: *mut GpBody) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    let (status, payload) = match call(MsgType::BodyCreate, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match read_u64(&mut cur) {
        Ok(id) => {
            unsafe { *out_body = id };
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

/// Remove a body.
#[no_mangle]
pub extern "C" fn gp_body_remove(engine: GpEngine, body: GpBody) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    match call(MsgType::BodyRemove, &buf) {
        Ok((s, _)) => result_from_status(s),
        Err(e) => e,
    }
}

// ── C ABI: Commands ──────────────────────────────────────────────────

/// Queue a phase command on a knob lane.
#[no_mangle]
pub extern "C" fn gp_command_phase(
    engine: GpEngine,
    body: GpBody,
    knob: u8,
    lane: u8,
    phase: u8,
) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    write_u8(&mut buf, knob).unwrap();
    write_u8(&mut buf, lane).unwrap();
    write_u8(&mut buf, phase).unwrap();
    match call(MsgType::CmdPhase, &buf) {
        Ok((s, _)) => result_from_status(s),
        Err(e) => e,
    }
}

/// Apply a central impulse (sign only — magnitude is not interpreted).
#[no_mangle]
pub extern "C" fn gp_apply_impulse(
    engine: GpEngine,
    body: GpBody,
    x: f64,
    y: f64,
    z: f64,
) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    write_f64(&mut buf, x).unwrap();
    write_f64(&mut buf, y).unwrap();
    write_f64(&mut buf, z).unwrap();
    match call(MsgType::CmdImpulse, &buf) {
        Ok((s, _)) => result_from_status(s),
        Err(e) => e,
    }
}

/// Apply a torque impulse (sign only).
#[no_mangle]
pub extern "C" fn gp_apply_torque_impulse(
    engine: GpEngine,
    body: GpBody,
    x: f64,
    y: f64,
    z: f64,
) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    write_f64(&mut buf, x).unwrap();
    write_f64(&mut buf, y).unwrap();
    write_f64(&mut buf, z).unwrap();
    match call(MsgType::CmdTorque, &buf) {
        Ok((s, _)) => result_from_status(s),
        Err(e) => e,
    }
}

// ── C ABI: Simulation ────────────────────────────────────────────────

/// Advance the engine by `ticks` steps. Returns new tick count in `out_tick`.
#[no_mangle]
pub extern "C" fn gp_tick(engine: GpEngine, ticks: u32, out_tick: *mut u64) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u32(&mut buf, ticks).unwrap();
    let (status, payload) = match call(MsgType::Tick, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    if !out_tick.is_null() {
        let mut cur = Cursor::new(&payload);
        if let Ok(t) = read_u64(&mut cur) {
            unsafe { *out_tick = t };
        }
    }
    GpResult::Ok
}

// ── C ABI: Observations ──────────────────────────────────────────────

/// Read the decoded body state.
#[no_mangle]
pub extern "C" fn gp_body_state(
    engine: GpEngine,
    body: GpBody,
    out_state: *mut GpStateC,
) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    let (status, payload) = match call(MsgType::BodyState, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match GpState::read_from(&mut cur) {
        Ok(st) => {
            unsafe {
                (*out_state).x = st.x;
                (*out_state).q = st.q;
                (*out_state).p = st.p;
                (*out_state).l = st.l;
                (*out_state).energy = st.energy;
                (*out_state).phase = st.phase;
                (*out_state).mode = st.mode;
                (*out_state).history = st.history;
            }
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

/// Read the Hamiltonian observation.
#[no_mangle]
pub extern "C" fn gp_read_hamiltonian(
    engine: GpEngine,
    body: GpBody,
    out: *mut GpHamiltonianC,
) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    let (status, payload) = match call(MsgType::HamiltonianEq, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match GpHamiltonianEq::read_from(&mut cur) {
        Ok(h) => {
            unsafe {
                (*out).h_free = h.h_free;
                (*out).g8_inverse = h.g8_inverse;
            }
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

/// Read spatial root codes (3 lanes).
#[no_mangle]
pub extern "C" fn gp_read_position(
    engine: GpEngine,
    body: GpBody,
    out_roots: *mut [u8; 3],
) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    write_u64(&mut buf, body).unwrap();
    let (status, payload) = match call(MsgType::SpatialRoots, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    if payload.len() >= 3 {
        unsafe {
            (*out_roots) = [payload[0], payload[1], payload[2]];
        }
        GpResult::Ok
    } else {
        GpResult::ProtocolError
    }
}

// ── C ABI: Temporal ──────────────────────────────────────────────────

/// Read the temporal clock.
#[no_mangle]
pub extern "C" fn gp_temporal_clock(engine: GpEngine, out: *mut GpTemporalClockC) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    let (status, payload) = match call(MsgType::TemporalClock, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match GpTemporalClock::read_from(&mut cur) {
        Ok(tc) => {
            unsafe {
                (*out).tick = tc.tick;
                (*out).phase = tc.phase;
                (*out).crossing = tc.crossing;
            }
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

/// Read elapsed seconds.
#[no_mangle]
pub extern "C" fn gp_temporal_seconds(engine: GpEngine, out: *mut f64) -> GpResult {
    let mut buf = Vec::new();
    write_u64(&mut buf, engine).unwrap();
    let (status, payload) = match call(MsgType::TemporalSeconds, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match read_f64(&mut cur) {
        Ok(s) => {
            unsafe { *out = s };
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

// ── C ABI: Science — EM ──────────────────────────────────────────────

/// Coulomb pair mapping: returns signed force byte.
#[no_mangle]
pub extern "C" fn gp_em_charge_force(
    q_a: i32,
    q_b: i32,
    g_a: u8,
    g_b: u8,
    out_force: *mut i8,
) -> GpResult {
    let mut buf = Vec::new();
    write_i32(&mut buf, q_a).unwrap();
    write_i32(&mut buf, q_b).unwrap();
    write_u8(&mut buf, g_a).unwrap();
    write_u8(&mut buf, g_b).unwrap();
    let (status, payload) = match call(MsgType::EmChargeForce, &buf) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match read_i8(&mut cur) {
        Ok(f) => {
            unsafe { *out_force = f };
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

// ── C ABI: Kernel ────────────────────────────────────────────────────

/// Read the kernel view (phi, alpha, roots).
#[no_mangle]
pub extern "C" fn gp_kernel_view(out: *mut GpKernelViewC) -> GpResult {
    let (status, payload) = match call(MsgType::KernelView, &[]) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if status != StatusCode::Ok {
        return result_from_status(status);
    }
    let mut cur = Cursor::new(&payload);
    match GpKernelView::read_from(&mut cur) {
        Ok(kv) => {
            unsafe {
                (*out).phi = kv.phi;
                (*out).alpha = kv.alpha;
                (*out).roots = kv.roots;
            }
            GpResult::Ok
        }
        Err(_) => GpResult::ProtocolError,
    }
}

// ── C ABI: Error info ────────────────────────────────────────────────

static LAST_FEATURE: Mutex<String> = Mutex::new(String::new());

/// Retrieve the last MissingLaw feature name. Returns a C string valid
/// until the next call.
#[no_mangle]
pub extern "C" fn gp_last_missing_feature() -> *const c_char {
    let (_, payload) = match call(MsgType::LastMissingFeature, &[]) {
        Ok(v) => v,
        Err(_) => return std::ptr::null(),
    };
    let mut cur = Cursor::new(&payload);
    if let Ok(detail) = GpErrorDetail::read_from(&mut cur) {
        if let Ok(mut guard) = LAST_FEATURE.lock() {
            *guard = detail.feature;
            guard.push('\0');
            return guard.as_ptr() as *const c_char;
        }
    }
    std::ptr::null()
}

// ── C ABI: Version ───────────────────────────────────────────────────

static VERSION_BUF: Mutex<String> = Mutex::new(String::new());

#[no_mangle]
pub extern "C" fn gp_runtime_version() -> *const c_char {
    let (_, payload) = match call(MsgType::RuntimeVersion, &[]) {
        Ok(v) => v,
        Err(_) => return std::ptr::null(),
    };
    let mut cur = Cursor::new(&payload);
    if let Ok(ver) = read_str(&mut cur) {
        if let Ok(mut guard) = VERSION_BUF.lock() {
            *guard = ver;
            guard.push('\0');
            return guard.as_ptr() as *const c_char;
        }
    }
    std::ptr::null()
}

#[no_mangle]
pub extern "C" fn gp_abi_version() -> u32 {
    let (_, payload) = match call(MsgType::AbiVersion, &[]) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let mut cur = Cursor::new(&payload);
    read_u32(&mut cur).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn gp_kernel_schema_version() -> u32 {
    let (_, payload) = match call(MsgType::KernelSchemaVersion, &[]) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    let mut cur = Cursor::new(&payload);
    read_u32(&mut cur).unwrap_or(0)
}
