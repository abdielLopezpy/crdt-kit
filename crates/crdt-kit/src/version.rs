//! Versioned serialization for CRDTs.
//!
//! Every CRDT type can be serialized with a version envelope that enables
//! transparent migration when schemas evolve.

use alloc::vec::Vec;
use core::fmt;

/// Magic byte identifying crdt-kit serialized data.
pub const MAGIC_BYTE: u8 = 0xCF;

/// Size of the version envelope header in bytes.
pub const ENVELOPE_HEADER_SIZE: usize = 3;

/// Identifies the type of CRDT for the version envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CrdtType {
    /// Grow-only counter.
    GCounter = 1,
    /// Positive-negative counter.
    PNCounter = 2,
    /// Grow-only set.
    GSet = 3,
    /// Two-phase set.
    TwoPSet = 4,
    /// Last-writer-wins register.
    LWWRegister = 5,
    /// Multi-value register.
    MVRegister = 6,
    /// Observed-remove set.
    ORSet = 7,
    /// Replicated Growable Array.
    Rga = 8,
    /// Collaborative text.
    TextCrdt = 9,
    /// Last-writer-wins map.
    LWWMap = 10,
    /// Add-wins map.
    AWMap = 11,
}

impl CrdtType {
    /// Convert from a raw byte.
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            1 => Some(Self::GCounter),
            2 => Some(Self::PNCounter),
            3 => Some(Self::GSet),
            4 => Some(Self::TwoPSet),
            5 => Some(Self::LWWRegister),
            6 => Some(Self::MVRegister),
            7 => Some(Self::ORSet),
            8 => Some(Self::Rga),
            9 => Some(Self::TextCrdt),
            10 => Some(Self::LWWMap),
            11 => Some(Self::AWMap),
            _ => None,
        }
    }
}

/// Trait for CRDT types that support versioned serialization.
///
/// Types implementing this trait can be serialized with a 3-byte version
/// envelope, enabling automatic migration when data schemas change.
pub trait Versioned: Sized {
    /// Current schema version for this CRDT type's serialization format.
    const CURRENT_VERSION: u8;

    /// The CRDT type identifier for the envelope.
    const CRDT_TYPE: CrdtType;
}

/// Error during versioned serialization.
#[derive(Debug, Clone)]
pub enum VersionError {
    /// Serialization failed.
    Serialize(alloc::string::String),
    /// Deserialization failed.
    Deserialize(alloc::string::String),
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(msg) => write!(f, "serialization error: {msg}"),
            Self::Deserialize(msg) => write!(f, "deserialization error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VersionError {}

// ── Versioned Envelope ─────────────────────────────────────────────

/// Error parsing a version envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvelopeError {
    /// Data is too short to contain a valid envelope.
    TooShort,
    /// Missing or incorrect magic byte.
    InvalidMagic(u8),
    /// Unknown CRDT type byte.
    UnknownCrdtType(u8),
}

impl fmt::Display for EnvelopeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort => write!(f, "data too short for version envelope"),
            Self::InvalidMagic(b) => write!(f, "invalid magic byte: 0x{b:02X}, expected 0xCF"),
            Self::UnknownCrdtType(b) => write!(f, "unknown CRDT type: {b}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EnvelopeError {}

/// A version envelope wrapping serialized CRDT data.
///
/// Binary format (3 bytes overhead):
/// ```text
/// [MAGIC: 0xCF][VERSION: u8][CRDT_TYPE: u8][PAYLOAD: N bytes]
/// ```
///
/// # Example
///
/// ```
/// use crdt_kit::version::{VersionedEnvelope, CrdtType};
///
/// let data = b"some serialized crdt state";
/// let envelope = VersionedEnvelope::new(1, CrdtType::GCounter, data.to_vec());
///
/// let bytes = envelope.to_bytes();
/// let decoded = VersionedEnvelope::from_bytes(&bytes).unwrap();
///
/// assert_eq!(decoded.version, 1);
/// assert_eq!(decoded.crdt_type, CrdtType::GCounter);
/// assert_eq!(decoded.payload, data);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct VersionedEnvelope {
    /// Schema version of the payload.
    pub version: u8,
    /// Type of CRDT contained.
    pub crdt_type: CrdtType,
    /// Serialized CRDT data.
    pub payload: Vec<u8>,
}

impl VersionedEnvelope {
    /// Create a new envelope.
    pub fn new(version: u8, crdt_type: CrdtType, payload: Vec<u8>) -> Self {
        Self {
            version,
            crdt_type,
            payload,
        }
    }

    /// Serialize the envelope to bytes.
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ENVELOPE_HEADER_SIZE + self.payload.len());
        bytes.push(MAGIC_BYTE);
        bytes.push(self.version);
        bytes.push(self.crdt_type as u8);
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    /// Parse an envelope from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<Self, EnvelopeError> {
        if data.len() < ENVELOPE_HEADER_SIZE {
            return Err(EnvelopeError::TooShort);
        }
        if data[0] != MAGIC_BYTE {
            return Err(EnvelopeError::InvalidMagic(data[0]));
        }
        let version = data[1];
        let crdt_type =
            CrdtType::from_byte(data[2]).ok_or(EnvelopeError::UnknownCrdtType(data[2]))?;
        let payload = data[ENVELOPE_HEADER_SIZE..].to_vec();
        Ok(Self {
            version,
            crdt_type,
            payload,
        })
    }

    /// Peek at the version without fully parsing the envelope.
    pub fn peek_version(data: &[u8]) -> Result<u8, EnvelopeError> {
        if data.len() < 2 {
            return Err(EnvelopeError::TooShort);
        }
        if data[0] != MAGIC_BYTE {
            return Err(EnvelopeError::InvalidMagic(data[0]));
        }
        Ok(data[1])
    }

    /// Check if bytes look like a versioned envelope (starts with magic byte).
    #[must_use]
    pub fn is_versioned(data: &[u8]) -> bool {
        data.first() == Some(&MAGIC_BYTE)
    }
}

// --- Versioned implementations for all 11 CRDT types ---

impl Versioned for crate::GCounter {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::GCounter;
}

impl Versioned for crate::PNCounter {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::PNCounter;
}

impl<T: Ord + Clone> Versioned for crate::GSet<T> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::GSet;
}

impl<T: Ord + Clone> Versioned for crate::TwoPSet<T> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::TwoPSet;
}

impl<T: Clone> Versioned for crate::LWWRegister<T> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::LWWRegister;
}

impl<T: Clone + Ord> Versioned for crate::MVRegister<T> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::MVRegister;
}

impl<T: Ord + Clone> Versioned for crate::ORSet<T> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::ORSet;
}

impl<T: Clone + Ord> Versioned for crate::Rga<T> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::Rga;
}

impl Versioned for crate::TextCrdt {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::TextCrdt;
}

impl<K: Ord + Clone, V: Clone> Versioned for crate::LWWMap<K, V> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::LWWMap;
}

impl<K: Ord + Clone, V: Clone + Eq> Versioned for crate::AWMap<K, V> {
    const CURRENT_VERSION: u8 = 1;
    const CRDT_TYPE: CrdtType = CrdtType::AWMap;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_roundtrip() {
        let original = VersionedEnvelope::new(3, CrdtType::ORSet, b"test-payload".to_vec());
        let bytes = original.to_bytes();
        let decoded = VersionedEnvelope::from_bytes(&bytes).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn envelope_header_size() {
        let envelope = VersionedEnvelope::new(1, CrdtType::GCounter, vec![]);
        let bytes = envelope.to_bytes();
        assert_eq!(bytes.len(), ENVELOPE_HEADER_SIZE);
    }

    #[test]
    fn envelope_peek_version() {
        let envelope = VersionedEnvelope::new(42, CrdtType::TextCrdt, b"data".to_vec());
        let bytes = envelope.to_bytes();
        assert_eq!(VersionedEnvelope::peek_version(&bytes).unwrap(), 42);
    }

    #[test]
    fn envelope_is_versioned() {
        assert!(VersionedEnvelope::is_versioned(&[MAGIC_BYTE, 1, 1]));
        assert!(!VersionedEnvelope::is_versioned(&[0x00, 1, 1]));
        assert!(!VersionedEnvelope::is_versioned(&[]));
    }

    #[test]
    fn envelope_error_too_short() {
        assert_eq!(
            VersionedEnvelope::from_bytes(&[MAGIC_BYTE]),
            Err(EnvelopeError::TooShort)
        );
    }

    #[test]
    fn envelope_error_invalid_magic() {
        assert_eq!(
            VersionedEnvelope::from_bytes(&[0xAB, 1, 1]),
            Err(EnvelopeError::InvalidMagic(0xAB))
        );
    }

    #[test]
    fn envelope_error_unknown_crdt_type() {
        assert_eq!(
            VersionedEnvelope::from_bytes(&[MAGIC_BYTE, 1, 200]),
            Err(EnvelopeError::UnknownCrdtType(200))
        );
    }

    #[test]
    fn all_crdt_types_roundtrip() {
        let types = [
            CrdtType::GCounter,
            CrdtType::PNCounter,
            CrdtType::GSet,
            CrdtType::TwoPSet,
            CrdtType::LWWRegister,
            CrdtType::MVRegister,
            CrdtType::ORSet,
            CrdtType::Rga,
            CrdtType::TextCrdt,
            CrdtType::LWWMap,
            CrdtType::AWMap,
        ];
        for ct in types {
            let envelope = VersionedEnvelope::new(1, ct, b"x".to_vec());
            let decoded = VersionedEnvelope::from_bytes(&envelope.to_bytes()).unwrap();
            assert_eq!(decoded.crdt_type, ct);
        }
    }

    #[test]
    fn crdt_type_from_byte_unknown() {
        assert_eq!(CrdtType::from_byte(0), None);
        assert_eq!(CrdtType::from_byte(12), None);
        assert_eq!(CrdtType::from_byte(255), None);
    }
}
