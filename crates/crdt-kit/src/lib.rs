//! # crdt-kit
//!
//! CRDTs optimized for edge computing and local-first applications.
//!
//! A CRDT (Conflict-free Replicated Data Type) is a data structure that can be
//! replicated across multiple devices and updated independently. When replicas
//! are merged, they are guaranteed to converge to the same state without
//! requiring coordination or consensus.
//!
//! ## `no_std` Support
//!
//! This crate supports `no_std` environments with the `alloc` crate.
//! Disable the default `std` feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! crdt-kit = { version = "0.5.0", default-features = false }
//! ```
//!
//! ## Quick Start
//!
//! ```
//! use crdt_kit::prelude::*;
//!
//! // Grow-only counter
//! let mut c1 = GCounter::new(1);
//! c1.increment();
//!
//! let mut c2 = GCounter::new(2);
//! c2.increment();
//!
//! c1.merge(&c2);
//! assert_eq!(c1.value(), 2);
//! ```
//!
//! ## Available CRDTs
//!
//! ### Counters
//! - [`GCounter`] - Grow-only counter (increment only)
//! - [`PNCounter`] - Positive-negative counter (increment and decrement)
//!
//! ### Registers
//! - [`LWWRegister`] - Last-writer-wins register (HLC-based resolution)
//! - [`MVRegister`] - Multi-value register (preserves concurrent writes)
//!
//! ### Sets
//! - [`GSet`] - Grow-only set (add only)
//! - [`TwoPSet`] - Two-phase set (add and remove, remove is permanent)
//! - [`ORSet`] - Observed-remove set (add and remove freely)
//!
//! ### Maps
//! - [`LWWMap`] - Last-writer-wins map (per-key HLC timestamp resolution)
//! - [`AWMap`] - Add-wins map (OR-Set semantics for keys, concurrent add beats remove)
//!
//! ### Sequences
//! - [`Rga`] - Replicated Growable Array (ordered sequence)
//! - [`TextCrdt`] - Collaborative text (thin wrapper over `Rga<char>`)
//!
//! ## The `Crdt` Trait
//!
//! All types implement the [`Crdt`] trait, which provides the [`Crdt::merge`]
//! method. Merge is guaranteed to be commutative, associative, and idempotent.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;

mod aw_map;
mod crdt;
mod gcounter;
mod gset;
mod lww_map;
mod lww_register;
mod mv_register;
mod or_set;
mod pncounter;
/// Replicated Growable Array (RGA) — ordered sequence CRDT.
pub mod rga;
mod text;
mod twop_set;
/// Versioned serialization and envelope format.
pub mod version;
#[cfg(feature = "wasm")]
mod wasm;

pub mod clock;
pub mod prelude;

pub use aw_map::{AWMap, AWMapDelta};
pub use crdt::{Crdt, DeltaCrdt, NodeId};
pub use gcounter::{GCounter, GCounterDelta};
pub use lww_map::{LWWMap, LWWMapDelta};
pub use gset::{GSet, GSetDelta};
pub use lww_register::{LWWRegister, LWWRegisterDelta};
pub use mv_register::{MVRegister, MVRegisterDelta};
pub use or_set::{ORSet, ORSetDelta};
pub use pncounter::{PNCounter, PNCounterDelta};
pub use rga::{Rga, RgaDelta, RgaError, RgaNode};
pub use text::{TextCrdt, TextDelta, TextError};
pub use twop_set::{TwoPSet, TwoPSetDelta};
pub use version::{
    CrdtType, EnvelopeError, VersionError, Versioned, VersionedEnvelope, ENVELOPE_HEADER_SIZE,
    MAGIC_BYTE,
};
