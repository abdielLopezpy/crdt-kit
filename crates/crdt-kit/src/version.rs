//! Versioned serialization for CRDTs.
//!
//! Every CRDT type can be serialized with a version envelope that enables
//! transparent migration when schemas evolve.

use core::fmt;

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
