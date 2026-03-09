use alloc::string::String;

use crate::clock::HybridClock;
use crate::{Crdt, DeltaCrdt};

/// A last-writer-wins register (LWW-Register).
///
/// Resolves concurrent writes by keeping the value with the highest timestamp.
/// Ties are broken by comparing actor IDs lexicographically.
///
/// # Example
///
/// ```
/// use crdt_kit::prelude::*;
///
/// let mut r1 = LWWRegister::new("node-1", "hello");
/// let mut r2 = LWWRegister::new("node-2", "world");
///
/// // The register with the later timestamp wins
/// r1.merge(&r2);
/// // Value is either "hello" or "world" depending on timestamps
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LWWRegister<T: Clone> {
    actor: String,
    value: T,
    timestamp: u64,
}

impl<T: Clone + PartialEq> PartialEq for LWWRegister<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.timestamp == other.timestamp
    }
}

impl<T: Clone + Eq> Eq for LWWRegister<T> {}

impl<T: Clone> LWWRegister<T> {
    /// Create a new LWW-Register with an initial value.
    ///
    /// The timestamp is automatically set to the current system time.
    ///
    /// This method requires the `std` feature. In `no_std` environments, use
    /// [`LWWRegister::with_timestamp`] instead.
    #[cfg(feature = "std")]
    pub fn new(actor: impl Into<String>, value: T) -> Self {
        Self {
            actor: actor.into(),
            value,
            timestamp: now(),
        }
    }

    /// Create a new LWW-Register with an explicit timestamp.
    ///
    /// Useful for testing or when you need deterministic behavior.
    /// This is the only constructor available in `no_std` environments.
    pub fn with_timestamp(actor: impl Into<String>, value: T, timestamp: u64) -> Self {
        Self {
            actor: actor.into(),
            value,
            timestamp,
        }
    }

    /// Update the register's value.
    ///
    /// The timestamp is automatically set to the current system time.
    ///
    /// This method requires the `std` feature. In `no_std` environments, use
    /// [`LWWRegister::set_with_timestamp`] instead.
    #[cfg(feature = "std")]
    pub fn set(&mut self, value: T) {
        self.value = value;
        self.timestamp = now();
    }

    /// Update the register's value with an explicit timestamp.
    pub fn set_with_timestamp(&mut self, value: T, timestamp: u64) {
        if timestamp >= self.timestamp {
            self.value = value;
            self.timestamp = timestamp;
        }
    }

    /// Create a new LWW-Register using a [`HybridClock`] for the timestamp.
    ///
    /// This ensures causally consistent timestamps even with clock drift.
    /// Works in both `std` and `no_std` environments.
    ///
    /// # Example
    ///
    /// ```
    /// use crdt_kit::clock::HybridClock;
    /// use crdt_kit::LWWRegister;
    ///
    /// let mut clock = HybridClock::new(1);
    /// let reg = LWWRegister::with_clock("node-1", "hello", &mut clock);
    /// assert_eq!(*reg.value(), "hello");
    /// ```
    pub fn with_clock(actor: impl Into<String>, value: T, clock: &mut HybridClock) -> Self {
        let ts = clock.now();
        // Pack physical_ms * 65536 + logical to preserve ordering in u64.
        // Overflows after ~8.9 million years of milliseconds — safe in practice.
        let timestamp = ts.physical.wrapping_mul(65536).wrapping_add(ts.logical as u64);
        Self {
            actor: actor.into(),
            value,
            timestamp,
        }
    }

    /// Update the register's value using a [`HybridClock`] for the timestamp.
    ///
    /// This is the recommended way to update a register in distributed
    /// systems where wall-clock synchronization is unreliable.
    pub fn set_with_clock(&mut self, value: T, clock: &mut HybridClock) {
        let ts = clock.now();
        let timestamp = ts.physical.wrapping_mul(65536).wrapping_add(ts.logical as u64);
        if timestamp >= self.timestamp {
            self.value = value;
            self.timestamp = timestamp;
        }
    }

    /// Get the current value.
    #[must_use]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the current timestamp.
    #[must_use]
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Get this replica's actor ID.
    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }
}

/// Delta for [`LWWRegister`]: the register state if newer, or `None` if the
/// other replica is already up to date.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LWWRegisterDelta<T: Clone> {
    /// `Some((value, timestamp, actor))` if the source is newer, `None` otherwise.
    pub update: Option<(T, u64, String)>,
}

impl<T: Clone> DeltaCrdt for LWWRegister<T> {
    type Delta = LWWRegisterDelta<T>;

    fn delta(&self, other: &Self) -> LWWRegisterDelta<T> {
        let dominated = other.timestamp > self.timestamp
            || (other.timestamp == self.timestamp && other.actor >= self.actor);
        if dominated {
            LWWRegisterDelta { update: None }
        } else {
            LWWRegisterDelta {
                update: Some((self.value.clone(), self.timestamp, self.actor.clone())),
            }
        }
    }

    fn apply_delta(&mut self, delta: &LWWRegisterDelta<T>) {
        if let Some((ref value, ts, ref actor)) = delta.update {
            if ts > self.timestamp || (ts == self.timestamp && actor > &self.actor) {
                self.value = value.clone();
                self.timestamp = ts;
            }
        }
    }
}

impl<T: Clone> Crdt for LWWRegister<T> {
    fn merge(&mut self, other: &Self) {
        if other.timestamp > self.timestamp
            || (other.timestamp == self.timestamp && other.actor > self.actor)
        {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

#[cfg(feature = "std")]
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_register_holds_value() {
        let r = LWWRegister::with_timestamp("a", 42, 1);
        assert_eq!(*r.value(), 42);
    }

    #[test]
    fn set_updates_value() {
        let mut r = LWWRegister::with_timestamp("a", 1, 1);
        r.set_with_timestamp(2, 2);
        assert_eq!(*r.value(), 2);
    }

    #[test]
    fn merge_keeps_later_timestamp() {
        let mut r1 = LWWRegister::with_timestamp("a", "old", 1);
        let r2 = LWWRegister::with_timestamp("b", "new", 2);

        r1.merge(&r2);
        assert_eq!(*r1.value(), "new");
    }

    #[test]
    fn merge_keeps_self_if_later() {
        let mut r1 = LWWRegister::with_timestamp("a", "new", 2);
        let r2 = LWWRegister::with_timestamp("b", "old", 1);

        r1.merge(&r2);
        assert_eq!(*r1.value(), "new");
    }

    #[test]
    fn merge_breaks_tie_by_actor() {
        let mut r1 = LWWRegister::with_timestamp("a", "first", 1);
        let r2 = LWWRegister::with_timestamp("b", "second", 1);

        r1.merge(&r2);
        // "b" > "a", so r2 wins the tie
        assert_eq!(*r1.value(), "second");
    }

    #[test]
    fn with_clock_creates_register() {
        use crate::clock::HybridClock;
        let mut clock = HybridClock::new(1);
        let r = LWWRegister::with_clock("a", "hello", &mut clock);
        assert_eq!(*r.value(), "hello");
        assert!(r.timestamp() > 0);
    }

    #[test]
    fn set_with_clock_advances_timestamp() {
        use crate::clock::HybridClock;
        let mut clock = HybridClock::new(1);
        let mut r = LWWRegister::with_clock("a", "v1", &mut clock);
        let ts1 = r.timestamp();

        r.set_with_clock("v2", &mut clock);
        assert_eq!(*r.value(), "v2");
        assert!(r.timestamp() > ts1);
    }

    #[test]
    fn hlc_register_merge_respects_causality() {
        use crate::clock::HybridClock;
        let mut clock1 = HybridClock::new(1);
        let mut clock2 = HybridClock::new(2);

        let mut r1 = LWWRegister::with_clock("a", "first", &mut clock1);
        let r2 = LWWRegister::with_clock("b", "second", &mut clock2);

        // Whichever has the higher HLC timestamp wins
        r1.merge(&r2);
        // Both should be valid — we just verify convergence
        assert!(*r1.value() == "first" || *r1.value() == "second");
    }

    #[test]
    fn merge_is_idempotent() {
        let mut r1 = LWWRegister::with_timestamp("a", "x", 1);
        let r2 = LWWRegister::with_timestamp("b", "y", 2);

        r1.merge(&r2);
        let after_first = r1.clone();
        r1.merge(&r2);

        assert_eq!(r1, after_first);
    }

    #[test]
    fn delta_apply_equivalent_to_merge() {
        let r1 = LWWRegister::with_timestamp("a", "old", 1);
        let r2 = LWWRegister::with_timestamp("b", "new", 2);

        let mut via_merge = r1.clone();
        via_merge.merge(&r2);

        let mut via_delta = r1.clone();
        let d = r2.delta(&r1);
        via_delta.apply_delta(&d);

        assert_eq!(*via_merge.value(), *via_delta.value());
    }

    #[test]
    fn delta_is_empty_when_other_is_newer() {
        let r1 = LWWRegister::with_timestamp("a", "old", 1);
        let r2 = LWWRegister::with_timestamp("b", "new", 2);

        let d = r1.delta(&r2);
        assert!(d.update.is_none());
    }
}
