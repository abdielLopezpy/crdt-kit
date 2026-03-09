use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::{Crdt, DeltaCrdt};

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

/// A collaborative text CRDT based on RGA (Replicated Growable Array) principles.
///
/// Each character is assigned a unique identifier `(actor, counter)` and is
/// stored in an internal sequence. Deletions use tombstones: characters are
/// marked as deleted but remain in the internal list so that concurrent
/// operations from different replicas can be merged deterministically.
///
/// Ordering of concurrent inserts at the same position is resolved by
/// comparing `(counter, actor)` tuples — higher counters come first, and
/// ties are broken lexicographically by actor ID.
///
/// # Example
///
/// ```
/// use crdt_kit::prelude::*;
///
/// let mut t1 = TextCrdt::new("alice");
/// t1.insert_str(0, "hello").unwrap();
///
/// let mut t2 = TextCrdt::new("bob");
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
pub struct TextCrdt {
    actor: String,
    counter: u64,
    /// The ordered sequence of elements (including tombstones).
    elements: Vec<Element>,
    /// Tracks the maximum counter observed per actor, used during merge to
    /// avoid re-inserting elements that are already present.
    version: BTreeMap<String, u64>,
    /// Cached count of visible (non-deleted) elements.
    visible_len: usize,
}

/// A single element in the text sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Element {
    /// Unique identifier: (actor, counter).
    pub id: (String, u64),
    /// The character value.
    pub value: char,
    /// Whether this element has been tombstoned (logically deleted).
    pub deleted: bool,
}

impl TextCrdt {
    /// Create a new empty text CRDT for the given actor.
    pub fn new(actor: impl Into<String>) -> Self {
        Self {
            actor: actor.into(),
            counter: 0,
            elements: Vec::new(),
            version: BTreeMap::new(),
            visible_len: 0,
        }
    }

    /// Create a fork of this replica with a different actor ID.
    ///
    /// The returned replica contains an identical copy of the current content
    /// and version state, but subsequent inserts will use the new actor,
    /// preventing ID collisions between the two replicas.
    pub fn fork(&self, new_actor: impl Into<String>) -> Self {
        Self {
            actor: new_actor.into(),
            counter: self.counter,
            elements: self.elements.clone(),
            version: self.version.clone(),
            visible_len: self.visible_len,
        }
    }

    /// Insert a character at the given visible index.
    ///
    /// Returns `Err(TextError::IndexOutOfBounds)` if `index` is greater than `self.len()`.
    pub fn insert(&mut self, index: usize, ch: char) -> Result<(), TextError> {
        if index > self.visible_len {
            return Err(TextError::IndexOutOfBounds { index, len: self.visible_len });
        }

        self.counter += 1;
        let id = (self.actor.clone(), self.counter);
        self.version
            .entry(self.actor.clone())
            .and_modify(|c| *c = (*c).max(self.counter))
            .or_insert(self.counter);

        let elem = Element {
            id,
            value: ch,
            deleted: false,
        };

        let raw_index = self.raw_index_for_insert(index);
        self.elements.insert(raw_index, elem);
        self.visible_len += 1;
        Ok(())
    }

    /// Insert a string at the given visible index.
    ///
    /// Characters are inserted left-to-right so that the resulting visible
    /// text contains the string starting at `index`.
    ///
    /// Returns `Err(TextError::IndexOutOfBounds)` if `index` is greater than `self.len()`.
    pub fn insert_str(&mut self, index: usize, s: &str) -> Result<(), TextError> {
        if index > self.visible_len {
            return Err(TextError::IndexOutOfBounds { index, len: self.visible_len });
        }

        for (i, ch) in s.chars().enumerate() {
            self.insert(index + i, ch)?;
        }
        Ok(())
    }

    /// Remove (tombstone) the character at the given visible index.
    ///
    /// Returns `Err(TextError::IndexOutOfBounds)` if `index >= self.len()`.
    pub fn remove(&mut self, index: usize) -> Result<(), TextError> {
        if index >= self.visible_len {
            return Err(TextError::IndexOutOfBounds { index, len: self.visible_len });
        }

        let raw = self.visible_to_raw(index);
        self.elements[raw].deleted = true;
        self.visible_len -= 1;
        Ok(())
    }

    /// Remove a range of characters starting at `start` with the given `count`.
    ///
    /// Returns `Err(TextError::RangeOutOfBounds)` if `start + count > self.len()`.
    pub fn remove_range(&mut self, start: usize, count: usize) -> Result<(), TextError> {
        if start + count > self.visible_len {
            return Err(TextError::RangeOutOfBounds {
                start,
                end: start + count,
                len: self.visible_len,
            });
        }

        // Remove from right to left so that indices remain valid.
        for i in (0..count).rev() {
            self.remove(start + i)?;
        }
        Ok(())
    }

    /// Return the number of visible (non-deleted) characters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.visible_len
    }

    /// Check whether the visible text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get this replica's actor ID.
    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }

    // ---- internal helpers ----

    /// Convert a visible index to a raw index in `self.elements`.
    ///
    /// Returns the raw index of the `n`-th visible element.
    fn visible_to_raw(&self, visible: usize) -> usize {
        let mut seen = 0;
        for (raw, elem) in self.elements.iter().enumerate() {
            if !elem.deleted {
                if seen == visible {
                    return raw;
                }
                seen += 1;
            }
        }
        panic!(
            "visible index {} not found (only {} visible elements)",
            visible, seen
        );
    }

    /// Determine the raw position at which to insert a new element so that it
    /// appears at the given visible index.
    ///
    /// If `index == self.len()` (append), the new element goes at the end.
    /// Otherwise it is placed just before the element currently at `index`.
    fn raw_index_for_insert(&self, visible_index: usize) -> usize {
        if visible_index == 0 {
            return 0;
        }

        let visible_count = self.len();
        if visible_index >= visible_count {
            // For append, go past all existing elements (including trailing
            // tombstones that belong after the last visible character).
            return self.elements.len();
        }

        // Insert just before the element that is currently at visible_index.
        self.visible_to_raw(visible_index)
    }

    /// Determine where an element from a remote replica should be inserted
    /// based on RGA ordering.
    ///
    /// We scan from `start` looking for the correct position. Among
    /// consecutive elements with higher or equal `(counter, actor)` we skip
    /// forward; the new element is placed before the first element that is
    /// strictly less.
    fn find_insert_position(&self, elem: &Element, after_raw: Option<usize>) -> usize {
        let start = match after_raw {
            Some(idx) => idx + 1,
            None => 0,
        };

        let new_key = (elem.id.1, &elem.id.0); // (counter, actor)

        for i in start..self.elements.len() {
            let existing = &self.elements[i];
            let existing_key = (existing.id.1, &existing.id.0);
            // The new element goes before any element that has a strictly
            // smaller ordering key. We use reverse ordering: larger counter
            // first, then larger actor first, so the new element is inserted
            // *before* the first element whose key is strictly less.
            if existing_key < new_key {
                return i;
            }
        }

        self.elements.len()
    }

}

/// Delta for [`TextCrdt`]: new elements and tombstone updates the other
/// replica is missing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextDelta {
    /// Elements the other replica doesn't have yet.
    pub new_elements: Vec<Element>,
    /// IDs of elements that are deleted in source but not in other.
    pub tombstoned_ids: Vec<(String, u64)>,
    /// Version vector of the source.
    pub version: BTreeMap<String, u64>,
}

impl DeltaCrdt for TextCrdt {
    type Delta = TextDelta;

    fn delta(&self, other: &Self) -> TextDelta {
        let new_elements: Vec<_> = self
            .elements
            .iter()
            .filter(|e| {
                let actor_max = other.version.get(&e.id.0).copied().unwrap_or(0);
                e.id.1 > actor_max
            })
            .cloned()
            .collect();

        let tombstoned_ids: Vec<_> = self
            .elements
            .iter()
            .filter(|e| {
                e.deleted && {
                    // Only include if the other has this element but hasn't deleted it
                    let actor_max = other.version.get(&e.id.0).copied().unwrap_or(0);
                    e.id.1 <= actor_max
                }
            })
            .map(|e| e.id.clone())
            .collect();

        TextDelta {
            new_elements,
            tombstoned_ids,
            version: self.version.clone(),
        }
    }

    fn apply_delta(&mut self, delta: &TextDelta) {
        // Build index for O(log n) lookups.
        let mut id_index: BTreeMap<(String, u64), usize> = self
            .elements
            .iter()
            .enumerate()
            .map(|(i, e)| (e.id.clone(), i))
            .collect();

        // Apply tombstones to existing elements.
        for id in &delta.tombstoned_ids {
            if let Some(&raw) = id_index.get(id) {
                if !self.elements[raw].deleted {
                    self.elements[raw].deleted = true;
                    self.visible_len -= 1;
                }
            }
        }

        // Insert new elements at correct positions.
        for (delta_idx, elem) in delta.new_elements.iter().enumerate() {
            if !id_index.contains_key(&elem.id) {
                let predecessor_raw = if delta_idx == 0 {
                    None
                } else {
                    (0..delta_idx)
                        .rev()
                        .find_map(|i| id_index.get(&delta.new_elements[i].id).copied())
                };

                let pos = self.find_insert_position(elem, predecessor_raw);
                self.elements.insert(pos, elem.clone());
                if !elem.deleted {
                    self.visible_len += 1;
                }

                for v in id_index.values_mut() {
                    if *v >= pos {
                        *v += 1;
                    }
                }
                id_index.insert(elem.id.clone(), pos);
            }
        }

        // Merge version vectors.
        for (actor, &cnt) in &delta.version {
            let entry = self.version.entry(actor.clone()).or_insert(0);
            *entry = (*entry).max(cnt);
        }

        if let Some(&max_cnt) = self.version.values().max() {
            self.counter = self.counter.max(max_cnt);
        }
    }
}

impl Crdt for TextCrdt {
    fn merge(&mut self, other: &Self) {
        // Build an index for O(log n) ID→position lookups instead of O(n)
        // linear scans. This reduces merge from O(m·n) to O(m·log n) for the
        // lookup-dominated phase.
        let mut id_index: BTreeMap<(String, u64), usize> = self
            .elements
            .iter()
            .enumerate()
            .map(|(i, e)| (e.id.clone(), i))
            .collect();

        for (other_idx, other_elem) in other.elements.iter().enumerate() {
            if let Some(&raw) = id_index.get(&other_elem.id) {
                // Element already present — propagate tombstones (delete wins).
                if other_elem.deleted && !self.elements[raw].deleted {
                    self.elements[raw].deleted = true;
                    self.visible_len -= 1;
                }
            } else {
                // New element — find its causal predecessor using the index.
                let predecessor_raw = if other_idx == 0 {
                    None
                } else {
                    (0..other_idx)
                        .rev()
                        .find_map(|i| id_index.get(&other.elements[i].id).copied())
                };

                let pos = self.find_insert_position(other_elem, predecessor_raw);
                self.elements.insert(pos, other_elem.clone());
                if !other_elem.deleted {
                    self.visible_len += 1;
                }

                // Keep the index consistent after the Vec insertion.
                for v in id_index.values_mut() {
                    if *v >= pos {
                        *v += 1;
                    }
                }
                id_index.insert(other_elem.id.clone(), pos);
            }
        }

        // Merge version vectors.
        for (actor, &cnt) in &other.version {
            let entry = self.version.entry(actor.clone()).or_insert(0);
            *entry = (*entry).max(cnt);
        }

        // Advance local counter past everything we have seen.
        if let Some(&max_cnt) = self.version.values().max() {
            self.counter = self.counter.max(max_cnt);
        }
    }
}

impl core::fmt::Display for TextCrdt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for elem in &self.elements {
            if !elem.deleted {
                write!(f, "{}", elem.value)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_text_is_empty() {
        let t = TextCrdt::new("a");
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert_eq!(t.to_string(), "");
    }

    #[test]
    fn insert_single_char() {
        let mut t = TextCrdt::new("a");
        t.insert(0, 'x').unwrap();
        assert_eq!(t.to_string(), "x");
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn insert_at_beginning_middle_end() {
        let mut t = TextCrdt::new("a");
        t.insert(0, 'a').unwrap(); // "a"
        t.insert(1, 'c').unwrap(); // "ac"
        t.insert(1, 'b').unwrap(); // "abc"
        t.insert(0, 'z').unwrap(); // "zabc"
        assert_eq!(t.to_string(), "zabc");
    }

    #[test]
    fn delete_char() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "hello").unwrap();
        assert_eq!(t.to_string(), "hello");

        t.remove(1).unwrap(); // remove 'e'
        assert_eq!(t.to_string(), "hllo");
        assert_eq!(t.len(), 4);
    }

    #[test]
    fn insert_str_basic() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "hello").unwrap();
        assert_eq!(t.to_string(), "hello");
        assert_eq!(t.len(), 5);
    }

    #[test]
    fn insert_str_at_middle() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "hd").unwrap();
        t.insert_str(1, "ello worl").unwrap();
        assert_eq!(t.to_string(), "hello world");
    }

    #[test]
    fn remove_range_basic() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "hello world").unwrap();
        t.remove_range(5, 6).unwrap(); // remove " world"
        assert_eq!(t.to_string(), "hello");
    }

    #[test]
    fn remove_range_from_start() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "hello").unwrap();
        t.remove_range(0, 3).unwrap(); // remove "hel"
        assert_eq!(t.to_string(), "lo");
    }

    #[test]
    fn remove_all() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "abc").unwrap();
        t.remove_range(0, 3).unwrap();
        assert!(t.is_empty());
        assert_eq!(t.to_string(), "");
    }

    #[test]
    fn merge_disjoint_inserts() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new("bob");
        t2.insert_str(0, "world").unwrap();

        t1.merge(&t2);

        // Both sets of characters should be present.
        let result = t1.to_string();
        assert!(result.contains("hello") || result.contains("world"));
        assert_eq!(t1.len(), 10);
    }

    #[test]
    fn merge_propagates_tombstones() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "abc").unwrap();

        // Fork with a different actor to simulate a second replica that
        // received the same state. Only deletes here, so no new IDs needed,
        // but fork is still the safe pattern.
        let mut t2 = t1.fork("bob");
        t2.remove(1).unwrap(); // delete 'b' on t2

        t1.merge(&t2);
        assert_eq!(t1.to_string(), "ac");
    }

    #[test]
    fn merge_commutativity() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new("bob");
        t2.insert_str(0, "world").unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        assert_eq!(left.to_string(), right.to_string());
    }

    #[test]
    fn merge_idempotency() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new("bob");
        t2.insert_str(0, "world").unwrap();

        t1.merge(&t2);
        let after_first = t1.clone();
        t1.merge(&t2);

        assert_eq!(t1.to_string(), after_first.to_string());
        assert_eq!(t1.len(), after_first.len());
    }

    #[test]
    fn concurrent_inserts_at_same_position() {
        // Both replicas start empty and insert at position 0.
        let mut t1 = TextCrdt::new("alice");
        t1.insert(0, 'a').unwrap();

        let mut t2 = TextCrdt::new("bob");
        t2.insert(0, 'b').unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        // The order must be deterministic and identical on both replicas.
        assert_eq!(left.to_string(), right.to_string());
        assert_eq!(left.len(), 2);
        // Both characters must be present.
        let s = left.to_string();
        assert!(s.contains('a'));
        assert!(s.contains('b'));
    }

    #[test]
    fn concurrent_inserts_at_same_position_in_existing_text() {
        // Both replicas share the same base text and insert at the same index.
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "ac").unwrap();

        // Fork to a different actor so new inserts get unique IDs.
        let mut t2 = t1.fork("bob");

        // Both insert at position 1 (between 'a' and 'c').
        t1.insert(1, 'X').unwrap();
        t2.insert(1, 'Y').unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        assert_eq!(left.to_string(), right.to_string());
        let s = left.to_string();
        assert!(s.starts_with('a'));
        assert!(s.ends_with('c'));
        assert!(s.contains('X'));
        assert!(s.contains('Y'));
    }

    #[test]
    fn concurrent_insert_and_delete() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "abc").unwrap();

        let mut t2 = t1.fork("bob");

        // alice deletes 'b'
        t1.remove(1).unwrap();
        // bob inserts 'X' at position 1
        t2.insert(1, 'X').unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        // Both should converge.
        assert_eq!(left.to_string(), right.to_string());
        // 'b' should be deleted but 'X' should be present.
        let s = left.to_string();
        assert!(!s.contains('b'));
        assert!(s.contains('X'));
        assert!(s.contains('a'));
        assert!(s.contains('c'));
    }

    #[test]
    fn merge_after_local_edits_on_both_sides() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = t1.fork("bob");

        // alice appends " world"
        t1.insert_str(5, " world").unwrap();
        // bob deletes "llo" and inserts "p"
        t2.remove_range(2, 3).unwrap();
        t2.insert(2, 'p').unwrap();

        let mut left = t1.clone();
        left.merge(&t2);

        let mut right = t2.clone();
        right.merge(&t1);

        assert_eq!(left.to_string(), right.to_string());
    }

    #[test]
    fn display_trait() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "hello").unwrap();
        assert_eq!(format!("{t}"), "hello");
    }

    #[test]
    fn actor_getter() {
        let t = TextCrdt::new("my-node");
        assert_eq!(t.actor(), "my-node");
    }

    #[test]
    fn insert_out_of_bounds_returns_error() {
        let mut t = TextCrdt::new("a");
        let err = t.insert(1, 'x');
        assert_eq!(err, Err(TextError::IndexOutOfBounds { index: 1, len: 0 }));
    }

    #[test]
    fn remove_out_of_bounds_returns_error() {
        let t = TextCrdt::new("a");
        assert_eq!(t.len(), 0);
        let mut t2 = TextCrdt::new("a");
        let err = t2.remove(0);
        assert_eq!(err, Err(TextError::IndexOutOfBounds { index: 0, len: 0 }));
    }

    #[test]
    fn remove_range_out_of_bounds_returns_error() {
        let mut t = TextCrdt::new("a");
        t.insert_str(0, "abc").unwrap();
        let err = t.remove_range(1, 5);
        assert_eq!(
            err,
            Err(TextError::RangeOutOfBounds { start: 1, end: 6, len: 3 })
        );
    }

    #[test]
    fn triple_merge_convergence() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "base").unwrap();

        let mut t2 = t1.fork("bob");
        let mut t3 = t1.fork("carol");

        t1.insert(4, '!').unwrap();
        t2.insert(0, '>').unwrap();
        t3.remove(2).unwrap(); // remove 's'

        // Merge in different orders and verify convergence.
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
    fn delta_apply_equivalent_to_merge() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "hello").unwrap();

        let mut t2 = TextCrdt::new("bob");
        t2.insert_str(0, "world").unwrap();

        let mut via_merge = t2.clone();
        via_merge.merge(&t1);

        let mut via_delta = t2.clone();
        let d = t1.delta(&t2);
        via_delta.apply_delta(&d);

        assert_eq!(via_merge.to_string(), via_delta.to_string());
    }

    #[test]
    fn delta_with_tombstones() {
        let mut t1 = TextCrdt::new("alice");
        t1.insert_str(0, "abc").unwrap();

        let t2 = t1.fork("bob");
        t1.remove(1).unwrap(); // delete 'b'

        let mut via_merge = t2.clone();
        via_merge.merge(&t1);

        let mut via_delta = t2.clone();
        let d = t1.delta(&t2);
        via_delta.apply_delta(&d);

        assert_eq!(via_merge.to_string(), via_delta.to_string());
        assert_eq!(via_delta.to_string(), "ac");
    }
}
