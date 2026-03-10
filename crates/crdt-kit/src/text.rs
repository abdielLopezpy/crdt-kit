use core::fmt;

use crate::rga::{Rga, RgaDelta, RgaError};
use crate::{Crdt, DeltaCrdt, NodeId};

/// Error type for TextCrdt operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextError {
    /// Index is out of bounds for the current visible text length.
    IndexOutOfBounds {
        /// The index that was requested.
        index: usize,
        /// The current length of the visible text.
        len: usize,
    },
    /// Range is out of bounds for the current visible text length.
    RangeOutOfBounds {
        /// Start of the range.
        start: usize,
        /// End of the range (exclusive).
        end: usize,
        /// The current length of the visible text.
        len: usize,
    },
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexOutOfBounds { index, len } => {
                write!(f, "index {index} out of bounds for text of length {len}")
            }
            Self::RangeOutOfBounds { start, end, len } => {
                write!(f, "range {start}..{end} out of bounds for text of length {len}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TextError {}

/// A collaborative text CRDT based on RGA (Replicated Growable Array).
///
/// This is a thin wrapper around [`Rga<char>`] that provides text-specific
/// convenience methods like `insert_str`, `remove_range`, and `Display`.
///
/// # Example
///
/// ```
/// use crdt_kit::prelude::*;
///
/// let mut t1 = TextCrdt::new(1);
/// t1.insert_str(0, "hello").unwrap();
///
/// let mut t2 = TextCrdt::new(2);
/// t2.insert_str(0, "world").unwrap();
///
/// t1.merge(&t2);
/// t2.merge(&t1);
///
/// // Both replicas converge to the same text.
/// assert_eq!(t1.to_string(), t2.to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextCrdt(Rga<char>);

/// Delta for [`TextCrdt`] — delegates to [`RgaDelta<char>`].
pub type TextDelta = RgaDelta<char>;

impl TextCrdt {
    /// Create a new empty text CRDT for the given node.
    pub fn new(actor: NodeId) -> Self {
        Self(Rga::new(actor))
    }

    /// Create a fork of this replica with a different node ID.
    pub fn fork(&self, new_actor: NodeId) -> Self {
        Self(self.0.fork(new_actor))
    }

    /// Insert a character at the given visible index.
    pub fn insert(&mut self, index: usize, ch: char) -> Result<(), TextError> {
        self.0.insert_at(index, ch).map_err(|e| match e {
            RgaError::IndexOutOfBounds { index, len } => {
                TextError::IndexOutOfBounds { index, len }
            }
        })
    }

    /// Insert a string at the given visible index.
    pub fn insert_str(&mut self, index: usize, s: &str) -> Result<(), TextError> {
        if index > self.0.len() {
            return Err(TextError::IndexOutOfBounds {
                index,
                len: self.0.len(),
            });
        }
        for (i, ch) in s.chars().enumerate() {
            self.insert(index + i, ch)?;
        }
        Ok(())
    }

    /// Remove (tombstone) the character at the given visible index.
    pub fn remove(&mut self, index: usize) -> Result<(), TextError> {
        self.0.remove(index).map(|_| ()).map_err(|e| match e {
            RgaError::IndexOutOfBounds { index, len } => {
                TextError::IndexOutOfBounds { index, len }
            }
        })
    }

    /// Remove a range of characters starting at `start` with the given `count`.
    pub fn remove_range(&mut self, start: usize, count: usize) -> Result<(), TextError> {
        let len = self.0.len();
        if start + count > len {
            return Err(TextError::RangeOutOfBounds {
                start,
                end: start + count,
                len,
            });
        }
        // Remove from back to front so indices stay valid
        for _ in 0..count {
            self.remove(start)?;
        }
        Ok(())
    }

    /// Return the number of visible (non-deleted) characters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check whether the visible text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get this replica's node ID.
    #[must_use]
    pub fn actor(&self) -> NodeId {
        self.0.actor()
    }
}

impl Crdt for TextCrdt {
    fn merge(&mut self, other: &Self) {
        self.0.merge(&other.0);
    }
}

impl DeltaCrdt for TextCrdt {
    type Delta = TextDelta;

    fn delta(&self, other: &Self) -> TextDelta {
        self.0.delta(&other.0)
    }

    fn apply_delta(&mut self, delta: &TextDelta) {
        self.0.apply_delta(delta);
    }
}

impl fmt::Display for TextCrdt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.0.iter() {
            write!(f, "{}", ch)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_text_is_empty() {
        let t = TextCrdt::new(1);
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert_eq!(t.to_string(), "");
    }

    #[test]
    fn insert_single_char() {
        let mut t = TextCrdt::new(1);
        t.insert(0, 'x').unwrap();
        assert_eq!(t.to_string(), "x");
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn insert_at_beginning_middle_end() {
        let mut t = TextCrdt::new(1);
        t.insert(0, 'a').unwrap();
        t.insert(1, 'c').unwrap();
        t.insert(1, 'b').unwrap();
        t.insert(0, 'z').unwrap();
        assert_eq!(t.to_string(), "zabc");
    }

    #[test]
    fn delete_char() {
        let mut t = TextCrdt::new(1);
        t.insert_str(0, "hello").unwrap();
        t.remove(1).unwrap();
        assert_eq!(t.to_string(), "hllo");
        assert_eq!(t.len(), 4);
    }

    #[test]
    fn insert_str_basic() {
        let mut t = TextCrdt::new(1);
        t.insert_str(0, "hello").unwrap();
        assert_eq!(t.to_string(), "hello");
        assert_eq!(t.len(), 5);
    }

    #[test]
    fn remove_range_basic() {
        let mut t = TextCrdt::new(1);
        t.insert_str(0, "hello world").unwrap();
        t.remove_range(5, 6).unwrap();
        assert_eq!(t.to_string(), "hello");
    }

    #[test]
    fn merge_disjoint_inserts() {
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new(2);
        t2.insert_str(0, "world").unwrap();

        t1.merge(&t2);
        assert_eq!(t1.len(), 10);
    }

    #[test]
    fn merge_propagates_tombstones() {
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, "abc").unwrap();

        let mut t2 = t1.fork(2);
        t2.remove(1).unwrap();

        t1.merge(&t2);
        assert_eq!(t1.to_string(), "ac");
    }

    #[test]
    fn merge_commutativity() {
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new(2);
        t2.insert_str(0, "world").unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        assert_eq!(left.to_string(), right.to_string());
    }

    #[test]
    fn merge_idempotency() {
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new(2);
        t2.insert_str(0, "world").unwrap();

        t1.merge(&t2);
        let after_first = t1.clone();
        t1.merge(&t2);

        assert_eq!(t1.to_string(), after_first.to_string());
    }

    #[test]
    fn concurrent_inserts_at_same_position() {
        let mut t1 = TextCrdt::new(1);
        t1.insert(0, 'a').unwrap();

        let mut t2 = TextCrdt::new(2);
        t2.insert(0, 'b').unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        assert_eq!(left.to_string(), right.to_string());
        assert_eq!(left.len(), 2);
    }

    #[test]
    fn delta_apply_equivalent_to_merge() {
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new(2);
        t2.insert_str(0, "world").unwrap();

        let mut via_merge = t2.clone();
        via_merge.merge(&t1);

        let mut via_delta = t2.clone();
        let d = t1.delta(&t2);
        via_delta.apply_delta(&d);

        assert_eq!(via_merge.to_string(), via_delta.to_string());
    }

    #[test]
    fn triple_merge_convergence() {
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, "base").unwrap();

        let mut t2 = t1.fork(2);
        let mut t3 = t1.fork(3);

        t1.insert(4, '!').unwrap();
        t2.insert(0, '>').unwrap();
        t3.remove(2).unwrap();

        let mut r1 = t1.clone();
        r1.merge(&t2);
        r1.merge(&t3);

        let mut r2 = t2.clone();
        r2.merge(&t3);
        r2.merge(&t1);

        let mut r3 = t3.clone();
        r3.merge(&t1);
        r3.merge(&t2);

        assert_eq!(r1.to_string(), r2.to_string());
        assert_eq!(r2.to_string(), r3.to_string());
    }

    #[test]
    fn insert_out_of_bounds_returns_error() {
        let mut t = TextCrdt::new(1);
        let err = t.insert(1, 'x');
        assert_eq!(err, Err(TextError::IndexOutOfBounds { index: 1, len: 0 }));
    }

    #[test]
    fn remove_out_of_bounds_returns_error() {
        let mut t = TextCrdt::new(1);
        let err = t.remove(0);
        assert_eq!(err, Err(TextError::IndexOutOfBounds { index: 0, len: 0 }));
    }

    #[test]
    fn display_trait() {
        let mut t = TextCrdt::new(1);
        t.insert_str(0, "hello").unwrap();
        assert_eq!(format!("{t}"), "hello");
    }
}
