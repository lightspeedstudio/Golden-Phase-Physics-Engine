//! Golden Phase SDK wire protocol.
//!
//! Transport-agnostic request/response types with manual binary
//! serialization. **Zero kernel dependencies** — this crate may be
//! published in the public SDK repository.
//!
//! Wire format (both request and response):
//! ```text
//!   [u32 payload_len][u32 request_id][u16 msg_type][payload …]
//! ```
//!
//! Response payload is prefixed by `[u16 status_code]`.

#![forbid(unsafe_code)]

use std::io::{self, Read, Write};

// ── ABI / Version constants ──────────────────────────────────────────

pub const ABI_VERSION: u32 = 1;
pub const KERNEL_SCHEMA_VERSION: u32 = 1;
pub const SDK_VERSION: &str = "0.1.0";
pub const RUNTIME_DEFAULT_PORT: u16 = 31415;
pub const PIPE_NAME: &str = r"\\.\pipe\golden_phase_runtime";

// ── Status codes ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum StatusCode {
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

impl StatusCode {
    pub fn from_u16(v: u16) -> Self {
        match v {
            0 => Self::Ok,
            1 => Self::MissingLaw,
            2 => Self::Refused,
            3 => Self::InvalidBody,
            4 => Self::InvalidState,
            5 => Self::BoundaryRefusal,
            6 => Self::IllegalTransition,
            253 => Self::NotConnected,
            254 => Self::ProtocolError,
            _ => Self::InternalError,
        }
    }
}

// ── Message types ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum MsgType {
    // Lifecycle
    EngineCreate = 1,
    EngineDestroy = 2,
    BodyCreate = 3,
    BodyRemove = 4,

    // Commands
    CmdPhase = 10,
    CmdQ = 11,
    CmdImpulse = 12,
    CmdTorque = 13,

    // Simulation
    Tick = 20,
    CurrentTick = 21,

    // State observation
    BodyState = 30,
    BodyCount = 31,
    BodyIds = 32,

    // Hamiltonian
    HamiltonianEq = 40,

    // Spatial
    SpatialRoots = 50,

    // Temporal
    TemporalClock = 60,
    TemporalSeconds = 61,

    // Sheath
    SheathObs = 70,

    // Science domains
    GravityPairForce = 80,
    EmChargeForce = 81,
    EmFrequency = 82,

    // Kernel
    KernelView = 110,

    // Feature / error info
    LastMissingFeature = 120,

    // Version
    RuntimeVersion = 200,
    AbiVersion = 201,
    KernelSchemaVersion = 202,
    BuildHash = 203,

    // Shutdown
    Shutdown = 255,
}

impl MsgType {
    pub fn from_u16(v: u16) -> Option<Self> {
        match v {
            1 => Some(Self::EngineCreate),
            2 => Some(Self::EngineDestroy),
            3 => Some(Self::BodyCreate),
            4 => Some(Self::BodyRemove),
            10 => Some(Self::CmdPhase),
            11 => Some(Self::CmdQ),
            12 => Some(Self::CmdImpulse),
            13 => Some(Self::CmdTorque),
            20 => Some(Self::Tick),
            21 => Some(Self::CurrentTick),
            30 => Some(Self::BodyState),
            31 => Some(Self::BodyCount),
            32 => Some(Self::BodyIds),
            40 => Some(Self::HamiltonianEq),
            50 => Some(Self::SpatialRoots),
            60 => Some(Self::TemporalClock),
            61 => Some(Self::TemporalSeconds),
            70 => Some(Self::SheathObs),
            80 => Some(Self::GravityPairForce),
            81 => Some(Self::EmChargeForce),
            82 => Some(Self::EmFrequency),
            110 => Some(Self::KernelView),
            120 => Some(Self::LastMissingFeature),
            200 => Some(Self::RuntimeVersion),
            201 => Some(Self::AbiVersion),
            202 => Some(Self::KernelSchemaVersion),
            203 => Some(Self::BuildHash),
            255 => Some(Self::Shutdown),
            _ => None,
        }
    }
}

// ── Wire helpers ─────────────────────────────────────────────────────

pub fn write_u8(w: &mut impl Write, v: u8) -> io::Result<()> {
    w.write_all(&[v])
}
pub fn write_u16(w: &mut impl Write, v: u16) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
pub fn write_u32(w: &mut impl Write, v: u32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
pub fn write_u64(w: &mut impl Write, v: u64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
pub fn write_i8(w: &mut impl Write, v: i8) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
pub fn write_i32(w: &mut impl Write, v: i32) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
pub fn write_f64(w: &mut impl Write, v: f64) -> io::Result<()> {
    w.write_all(&v.to_le_bytes())
}
pub fn write_str(w: &mut impl Write, s: &str) -> io::Result<()> {
    let b = s.as_bytes();
    write_u16(w, b.len() as u16)?;
    w.write_all(b)
}

pub fn read_u8(r: &mut impl Read) -> io::Result<u8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(b[0])
}
pub fn read_u16(r: &mut impl Read) -> io::Result<u16> {
    let mut b = [0u8; 2];
    r.read_exact(&mut b)?;
    Ok(u16::from_le_bytes(b))
}
pub fn read_u32(r: &mut impl Read) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}
pub fn read_u64(r: &mut impl Read) -> io::Result<u64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b))
}
pub fn read_i8(r: &mut impl Read) -> io::Result<i8> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    Ok(i8::from_le_bytes(b))
}
pub fn read_i32(r: &mut impl Read) -> io::Result<i32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(i32::from_le_bytes(b))
}
pub fn read_f64(r: &mut impl Read) -> io::Result<f64> {
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(f64::from_le_bytes(b))
}
pub fn read_str(r: &mut impl Read) -> io::Result<String> {
    let len = read_u16(r)? as usize;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

// ── Framed message I/O ──────────────────────────────────────────────

/// Write a framed request: `[u32 len][u32 req_id][u16 msg_type][payload]`.
pub fn write_request(
    w: &mut impl Write,
    req_id: u32,
    msg: MsgType,
    payload: &[u8],
) -> io::Result<()> {
    let total = 4 + 2 + payload.len();
    write_u32(w, total as u32)?;
    write_u32(w, req_id)?;
    write_u16(w, msg as u16)?;
    w.write_all(payload)?;
    w.flush()
}

/// Write a framed response: `[u32 len][u32 req_id][u16 status][payload]`.
pub fn write_response(
    w: &mut impl Write,
    req_id: u32,
    status: StatusCode,
    payload: &[u8],
) -> io::Result<()> {
    let total = 4 + 2 + payload.len();
    write_u32(w, total as u32)?;
    write_u32(w, req_id)?;
    write_u16(w, status as u16)?;
    w.write_all(payload)?;
    w.flush()
}

/// Read a frame header: returns `(payload_remaining, req_id, type_or_status)`.
pub fn read_frame_header(r: &mut impl Read) -> io::Result<(usize, u32, u16)> {
    let total = read_u32(r)? as usize;
    let req_id = read_u32(r)?;
    let tag = read_u16(r)?;
    let remaining = total.saturating_sub(6);
    Ok((remaining, req_id, tag))
}

/// Read exactly `n` bytes of payload.
pub fn read_payload(r: &mut impl Read, n: usize) -> io::Result<Vec<u8>> {
    let mut buf = vec![0u8; n];
    r.read_exact(&mut buf)?;
    Ok(buf)
}

// ── Public state types (decoded observation, NOT internal packing) ───

/// Decoded body state — the public observation a developer receives.
/// Root codes are u8 in `[0,3]` representing `Φ0..Φ3`.
#[derive(Debug, Clone, PartialEq)]
pub struct GpState {
    pub x: [u8; 3],
    pub q: [u8; 4],
    pub p: [u8; 3],
    pub l: [u8; 3],
    pub energy: u8,
    pub phase: u8,
    pub mode: u8,
    pub history: u8,
}

impl GpState {
    pub fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        for &v in &self.x {
            write_u8(w, v)?;
        }
        for &v in &self.q {
            write_u8(w, v)?;
        }
        for &v in &self.p {
            write_u8(w, v)?;
        }
        for &v in &self.l {
            write_u8(w, v)?;
        }
        write_u8(w, self.energy)?;
        write_u8(w, self.phase)?;
        write_u8(w, self.mode)?;
        write_u8(w, self.history)
    }

    pub fn read_from(r: &mut impl Read) -> io::Result<Self> {
        let mut x = [0u8; 3];
        let mut q = [0u8; 4];
        let mut p = [0u8; 3];
        let mut l = [0u8; 3];
        for v in &mut x {
            *v = read_u8(r)?;
        }
        for v in &mut q {
            *v = read_u8(r)?;
        }
        for v in &mut p {
            *v = read_u8(r)?;
        }
        for v in &mut l {
            *v = read_u8(r)?;
        }
        Ok(Self {
            x,
            q,
            p,
            l,
            energy: read_u8(r)?,
            phase: read_u8(r)?,
            mode: read_u8(r)?,
            history: read_u8(r)?,
        })
    }
}

/// Hamiltonian observation.
#[derive(Debug, Clone, PartialEq)]
pub struct GpHamiltonianEq {
    pub h_free: f64,
    pub g8_inverse: f64,
}

impl GpHamiltonianEq {
    pub fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        write_f64(w, self.h_free)?;
        write_f64(w, self.g8_inverse)
    }
    pub fn read_from(r: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            h_free: read_f64(r)?,
            g8_inverse: read_f64(r)?,
        })
    }
}

/// Temporal clock observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpTemporalClock {
    pub tick: u64,
    pub phase: u8,
    pub crossing: u8,
}

impl GpTemporalClock {
    pub fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        write_u64(w, self.tick)?;
        write_u8(w, self.phase)?;
        write_u8(w, self.crossing)
    }
    pub fn read_from(r: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            tick: read_u64(r)?,
            phase: read_u8(r)?,
            crossing: read_u8(r)?,
        })
    }
}

/// Kernel view observation.
#[derive(Debug, Clone, PartialEq)]
pub struct GpKernelView {
    pub phi: f64,
    pub alpha: f64,
    pub roots: [f64; 4],
}

impl GpKernelView {
    pub fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        write_f64(w, self.phi)?;
        write_f64(w, self.alpha)?;
        for &r in &self.roots {
            write_f64(w, r)?;
        }
        Ok(())
    }
    pub fn read_from(r: &mut impl Read) -> io::Result<Self> {
        let phi = read_f64(r)?;
        let alpha = read_f64(r)?;
        let mut roots = [0.0f64; 4];
        for v in &mut roots {
            *v = read_f64(r)?;
        }
        Ok(Self { phi, alpha, roots })
    }
}

/// Provenance tag for every SDK observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Provenance {
    Derived = 0,
    Declared = 1,
    Measured = 2,
    LawDefined = 3,
    MissingLaw = 4,
    Refused = 5,
}

impl Provenance {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::Derived,
            1 => Self::Declared,
            2 => Self::Measured,
            3 => Self::LawDefined,
            4 => Self::MissingLaw,
            5 => Self::Refused,
            _ => Self::MissingLaw,
        }
    }
}

/// Knob identifiers (public SDK mirror — same encoding as kernel).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum GpKnob {
    X = 0,
    Q = 1,
    P = 2,
    L = 3,
    E = 4,
    Phi = 5,
    M = 6,
    H = 7,
}

impl GpKnob {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::X),
            1 => Some(Self::Q),
            2 => Some(Self::P),
            3 => Some(Self::L),
            4 => Some(Self::E),
            5 => Some(Self::Phi),
            6 => Some(Self::M),
            7 => Some(Self::H),
            _ => None,
        }
    }
}

/// Error detail returned with non-OK responses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpErrorDetail {
    pub feature: String,
    pub reason: String,
}

impl GpErrorDetail {
    pub fn write_to(&self, w: &mut impl Write) -> io::Result<()> {
        write_str(w, &self.feature)?;
        write_str(w, &self.reason)
    }
    pub fn read_from(r: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            feature: read_str(r)?,
            reason: read_str(r)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_round_trips() {
        let st = GpState {
            x: [1, 2, 3],
            q: [0, 1, 2, 3],
            p: [1, 0, 2],
            l: [3, 2, 1],
            energy: 1,
            phase: 2,
            mode: 0,
            history: 0,
        };
        let mut buf = Vec::new();
        st.write_to(&mut buf).unwrap();
        let mut cursor = std::io::Cursor::new(&buf);
        let st2 = GpState::read_from(&mut cursor).unwrap();
        assert_eq!(st, st2);
    }

    #[test]
    fn frame_round_trips() {
        let mut buf = Vec::new();
        write_request(&mut buf, 42, MsgType::EngineCreate, &[]).unwrap();
        let mut cursor = std::io::Cursor::new(&buf);
        let (remaining, req_id, tag) = read_frame_header(&mut cursor).unwrap();
        assert_eq!(req_id, 42);
        assert_eq!(tag, MsgType::EngineCreate as u16);
        assert_eq!(remaining, 0);
    }

    #[test]
    fn kernel_view_round_trips() {
        let kv = GpKernelView {
            phi: 1.618033988749895,
            alpha: 0.007297352,
            roots: [0.0, 1.0, -1.0, 0.5],
        };
        let mut buf = Vec::new();
        kv.write_to(&mut buf).unwrap();
        let mut cursor = std::io::Cursor::new(&buf);
        let kv2 = GpKernelView::read_from(&mut cursor).unwrap();
        assert_eq!(kv, kv2);
    }
}
