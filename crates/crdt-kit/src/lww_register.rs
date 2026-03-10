use crate::clock::{HybridClock, HybridTimestamp};
use crate::{Crdt, DeltaCrdt};

/// A last-writer-wins register (LWW-Register).
///
/// Resolves concurrent writes by keeping the value with the highest
/// [`HybridTimestamp`]. This provides causally consistent ordering even
/// with clock drift between nodes.
///
/// # Example
///
/// ```
/// use crdt_kit::prelude::*;
/// use crdt_kit::clock::HybridClock;
///
/// let mut clock1 = HybridClock::new(1);
/// let mut clock2 = HybridClock::new(2);
///
/// let mut r1 = LWWRegister::new("hello", &mut clock1);
/// let mut r2 = LWWRegister::new("world", &mut clock2);
///
/// r1.merge(&r2);
/// // Value is determined by HLC timestamp ordering
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LWWRegister<T: Clone> {
    value: T,
    timestamp: HybridTimestamp,
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
    /// Uses the provided [`HybridClock`] for a causally consistent timestamp.
    pub fn new(value: T, clock: &mut HybridClock) -> Self {
        Self {
            value,
            timestamp: clock.now(),
        }
    }

    /// Create a new LWW-Register with an explicit timestamp.
    ///
    /// Useful for testing or deserialization.
    pub fn with_timestamp(value: T, timestamp: HybridTimestamp) -> Self {
        Self { value, timestamp }
    }

    /// Update the register's value.
    ///
    /// Uses the provided [`HybridClock`] for a causally consistent timestamp.
    pub fn set(&mut self, value: T, clock: &mut HybridClock) {
        let ts = clock.now();
        if ts >= self.timestamp {
            self.value = value;
            self.timestamp = ts;
        }
    }

    /// Update the register's value with an explicit timestamp.
    ///
    /// Only applies if the new timestamp is >= the current one.
    pub fn set_with_timestamp(&mut self, value: T, timestamp: HybridTimestamp) {
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
    pub fn timestamp(&self) -> HybridTimestamp {
        self.timestamp
    }
}

/// Delta for [`LWWRegister`]: the register state if newer, or `None` if the
/// other replica is already up to date.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LWWRegisterDelta<T: Clone> {
    /// `Some((value, timestamp))` if the source is newer, `None` otherwise.
    pub update: Option<(T, HybridTimestamp)>,
}

impl<T: Clone> DeltaCrdt for LWWRegister<T> {
    type Delta = LWWRegisterDelta<T>;

    fn delta(&self, other: &Self) -> LWWRegisterDelta<T> {
        if self.timestamp > other.timestamp {
            LWWRegisterDelta {
                update: Some((self.value.clone(), self.timestamp)),
            }
        } else {
            LWWRegisterDelta { update: None }
        }
    }

    fn apply_delta(&mut self, delta: &LWWRegisterDelta<T>) {
        if let Some((ref value, ts)) = delta.update {
            if ts > self.timestamp {
                self.value = value.clone();
                self.timestamp = ts;
            }
        }
    }
}

impl<T: Clone> Crdt for LWWRegister<T> {
    fn merge(&mut self, other: &Self) {
        if other.timestamp > self.timestamp {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::HybridTimestamp;

    fn ts(physical: u64, logical: u16, node_id: u16) -> HybridTimestamp {
        HybridTimestamp {
            physical,
            logical,
            node_id,
        }
    }

    #[test]
    fn new_register_holds_value() {
        let r = LWWRegister::with_timestamp(42, ts(1, 0, 1));
        assert_eq!(*r.value(), 42);
    }

    #[test]
    fn set_updates_value() {
        let mut r = LWWRegister::with_timestamp(1, ts(1, 0, 1));
        r.set_with_timestamp(2, ts(2, 0, 1));
        assert_eq!(*r.value(), 2);
    }

    #[test]
    fn merge_keeps_later_timestamp() {
        let mut r1 = LWWRegister::with_timestamp("old", ts(1, 0, 1));
        let r2 = LWWRegister::with_timestamp("new", ts(2, 0, 2));

        r1.merge(&r2);
        assert_eq!(*r1.value(), "new");
    }

    #[test]
    fn merge_keeps_self_if_later() {
        let mut r1 = LWWRegister::with_timestamp("new", ts(2, 0, 1));
        let r2 = LWWRegister::with_timestamp("old", ts(1, 0, 2));

        r1.merge(&r2);
        assert_eq!(*r1.value(), "new");
    }

    #[test]
    fn merge_breaks_tie_by_node_id() {
        let mut r1 = LWWRegister::with_timestamp("first", ts(1, 0, 1));
        let r2 = LWWRegister::with_timestamp("second", ts(1, 0, 2));

        r1.merge(&r2);
        // node_id 2 > node_id 1, so r2 wins the tie
        assert_eq!(*r1.value(), "second");
    }

    #[test]
    fn with_clock_creates_register() {
        let mut clock = HybridClock::new(1);
        let r = LWWRegister::new("hello", &mut clock);
        assert_eq!(*r.value(), "hello");
    }

    #[test]
    fn set_with_clock_advances_timestamp() {
        let mut clock = HybridClock::new(1);
        let mut r = LWWRegister::new("v1", &mut clock);
        let ts1 = r.timestamp();

        r.set("v2", &mut clock);
        assert_eq!(*r.value(), "v2");
        assert!(r.timestamp() > ts1);
    }

    #[test]
    fn hlc_register_merge_respects_causality() {
        let mut clock1 = HybridClock::new(1);
        let mut clock2 = HybridClock::new(2);

        let mut r1 = LWWRegister::new("first", &mut clock1);
        let r2 = LWWRegister::new("second", &mut clock2);

        r1.merge(&r2);
        assert!(*r1.value() == "first" || *r1.value() == "second");
    }

    #[test]
    fn merge_is_idempotent() {
        let mut r1 = LWWRegister::with_timestamp("x", ts(1, 0, 1));
        let r2 = LWWRegister::with_timestamp("y", ts(2, 0, 2));

        r1.merge(&r2);
        let after_first = r1.clone();
        r1.merge(&r2);

        assert_eq!(r1, after_first);
    }

    #[test]
    fn delta_apply_equivalent_to_merge() {
        let r1 = LWWRegister::with_timestamp("old", ts(1, 0, 1));
        let r2 = LWWRegister::with_timestamp("new", ts(2, 0, 2));

        let mut via_merge = r1.clone();
        via_merge.merge(&r2);

        let mut via_delta = r1.clone();
        let d = r2.delta(&r1);
        via_delta.apply_delta(&d);

        assert_eq!(*via_merge.value(), *via_delta.value());
    }

    #[test]
    fn delta_is_empty_when_other_is_newer() {
        let r1 = LWWRegister::with_timestamp("old", ts(1, 0, 1));
        let r2 = LWWRegister::with_timestamp("new", ts(2, 0, 2));

        let d = r1.delta(&r2);
        assert!(d.update.is_none());
    }
}
