use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::{Crdt, DeltaCrdt};

/// Error type for RGA operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RgaError {
    /// Index is out of bounds for the current visible sequence length.
    IndexOutOfBounds {
        /// The index that was requested.
        index: usize,
        /// The current length of the visible sequence.
        len: usize,
    },
}

impl fmt::Display for RgaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IndexOutOfBounds { index, len } => {
                write!(f, "index {index} out of bounds for length {len}")
            }
        }
    }
}

/// A single node in the RGA sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RgaNode<T: Clone + Ord> {
    /// Unique identifier: (actor, counter).
    pub id: (String, u64),
    /// The element value.
    pub value: T,
    /// Whether this element has been tombstoned (logically deleted).
    pub deleted: bool,
}

/// A Replicated Growable Array (RGA) — an ordered sequence CRDT.
///
/// RGA supports insert and delete at arbitrary positions while
/// guaranteeing convergence across replicas. Each element is assigned
/// a unique identifier `(actor, counter)` which determines causal
/// ordering. When two replicas concurrently insert at the same
/// position, the conflict is resolved deterministically by comparing
/// the unique identifiers, ensuring all replicas converge to the
/// same sequence after merging.
///
/// # Example
///
/// ```
/// use crdt_kit::prelude::*;
///
/// let mut list1 = Rga::new("node-1");
/// list1.insert_at(0, 'H').unwrap();
/// list1.insert_at(1, 'i').unwrap();
///
/// let mut list2 = Rga::new("node-2");
/// list2.insert_at(0, '!').unwrap();
///
/// list1.merge(&list2);
/// list2.merge(&list1);
///
/// // Both replicas converge to the same sequence
/// let v1: Vec<&char> = list1.iter().collect();
/// let v2: Vec<&char> = list2.iter().collect();
/// assert_eq!(v1, v2);
/// assert_eq!(list1.len(), 3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rga<T: Clone + Ord> {
    actor: String,
    counter: u64,
    /// Flat ordered sequence of elements (including tombstones).
    elements: Vec<RgaNode<T>>,
    /// Version vector: max counter observed per actor.
    version: BTreeMap<String, u64>,
    /// Cached count of visible (non-tombstoned) elements.
    visible_len: usize,
}

impl<T: Clone + Ord> Rga<T> {
    /// Create a new empty RGA for the given actor.
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

    /// Insert a value at the given index in the visible sequence.
    ///
    /// Returns `Err(RgaError::IndexOutOfBounds)` if `index > self.len()`.
    pub fn insert_at(&mut self, index: usize, value: T) -> Result<(), RgaError> {
        if index > self.visible_len {
            return Err(RgaError::IndexOutOfBounds {
                index,
                len: self.visible_len,
            });
        }

        self.counter += 1;
        let id = (self.actor.clone(), self.counter);
        self.version
            .entry(self.actor.clone())
            .and_modify(|c| *c = (*c).max(self.counter))
            .or_insert(self.counter);

        let node = RgaNode {
            id,
            value,
            deleted: false,
        };

        let raw_index = self.raw_index_for_insert(index);
        self.elements.insert(raw_index, node);
        self.visible_len += 1;
        Ok(())
    }

    /// Remove the element at the given index from the visible sequence.
    ///
    /// Returns the removed value, or `None` if the index is out of bounds.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.visible_len {
            return None;
        }
        let raw = self.visible_to_raw(index);
        self.elements[raw].deleted = true;
        self.visible_len -= 1;
        Some(self.elements[raw].value.clone())
    }

    /// Get a reference to the element at the given index in the visible sequence.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.visible_len {
            return None;
        }
        let raw = self.visible_to_raw(index);
        Some(&self.elements[raw].value)
    }

    /// Get the number of visible (non-tombstoned) elements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.visible_len
    }

    /// Check if the visible sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.visible_len == 0
    }

    /// Iterate over the visible elements in order.
    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.elements
            .iter()
            .filter(|n| !n.deleted)
            .map(|n| &n.value)
    }

    /// Get this replica's actor ID.
    #[must_use]
    pub fn actor(&self) -> &str {
        &self.actor
    }

    /// Collect visible elements into a `Vec`.
    #[must_use]
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }

    // ---- internal helpers ----

    /// Convert a visible index to a raw index in `self.elements`.
    fn visible_to_raw(&self, visible: usize) -> usize {
        let mut seen = 0;
        for (raw, node) in self.elements.iter().enumerate() {
            if !node.deleted {
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
    fn raw_index_for_insert(&self, visible_index: usize) -> usize {
        if visible_index == 0 {
            return 0;
        }
        if visible_index >= self.visible_len {
            return self.elements.len();
        }
        self.visible_to_raw(visible_index)
    }

    /// Determine where an element from a remote replica should be inserted
    /// based on RGA ordering.
    ///
    /// Among consecutive elements with higher or equal `(counter, actor)` we
    /// skip forward; the new element is placed before the first element that
    /// is strictly less.
    fn find_insert_position(&self, node: &RgaNode<T>, after_raw: Option<usize>) -> usize {
        let start = match after_raw {
            Some(idx) => idx + 1,
            None => 0,
        };

        let new_key = (node.id.1, &node.id.0); // (counter, actor)

        for i in start..self.elements.len() {
            let existing = &self.elements[i];
            let existing_key = (existing.id.1, &existing.id.0);
            if existing_key < new_key {
                return i;
            }
        }

        self.elements.len()
    }
}

/// Delta for [`Rga`]: elements and tombstones that the other replica is missing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RgaDelta<T: Clone + Ord> {
    /// Elements that the other replica doesn't have yet.
    pub new_elements: Vec<RgaNode<T>>,
    /// IDs of elements that are deleted in source but not in other.
    pub tombstoned_ids: Vec<(String, u64)>,
    /// Version vector of the source.
    pub version: BTreeMap<String, u64>,
}

impl<T: Clone + Ord> DeltaCrdt for Rga<T> {
    type Delta = RgaDelta<T>;

    fn delta(&self, other: &Self) -> RgaDelta<T> {
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
                    let actor_max = other.version.get(&e.id.0).copied().unwrap_or(0);
                    e.id.1 <= actor_max
                }
            })
            .map(|e| e.id.clone())
            .collect();

        RgaDelta {
            new_elements,
            tombstoned_ids,
            version: self.version.clone(),
        }
    }

    fn apply_delta(&mut self, delta: &RgaDelta<T>) {
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

impl<T: Clone + Ord> Crdt for Rga<T> {
    fn merge(&mut self, other: &Self) {
        // Build an index for O(log n) ID→position lookups.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rga_is_empty() {
        let rga = Rga::<String>::new("a");
        assert!(rga.is_empty());
        assert_eq!(rga.len(), 0);
        assert_eq!(rga.get(0), None);
    }

    #[test]
    fn insert_at_head() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 'H').unwrap();
        rga.insert_at(1, 'i').unwrap();
        assert_eq!(rga.len(), 2);
        assert_eq!(rga.get(0), Some(&'H'));
        assert_eq!(rga.get(1), Some(&'i'));
    }

    #[test]
    fn insert_at_middle() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 'a').unwrap();
        rga.insert_at(1, 'c').unwrap();
        rga.insert_at(1, 'b').unwrap();
        assert_eq!(rga.to_vec(), vec!['a', 'b', 'c']);
    }

    #[test]
    fn insert_at_end() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 1).unwrap();
        rga.insert_at(1, 2).unwrap();
        rga.insert_at(2, 3).unwrap();
        assert_eq!(rga.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn insert_out_of_bounds_returns_error() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 'x').unwrap();
        let err = rga.insert_at(5, 'y');
        assert_eq!(
            err,
            Err(RgaError::IndexOutOfBounds { index: 5, len: 1 })
        );
    }

    #[test]
    fn remove_element() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 'a').unwrap();
        rga.insert_at(1, 'b').unwrap();
        rga.insert_at(2, 'c').unwrap();

        let removed = rga.remove(1);
        assert_eq!(removed, Some('b'));
        assert_eq!(rga.len(), 2);
        assert_eq!(rga.to_vec(), vec!['a', 'c']);
    }

    #[test]
    fn remove_out_of_bounds_returns_none() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 'a').unwrap();
        assert_eq!(rga.remove(5), None);
        assert_eq!(rga.len(), 1);
    }

    #[test]
    fn remove_first_and_last() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 'a').unwrap();
        rga.insert_at(1, 'b').unwrap();
        rga.insert_at(2, 'c').unwrap();

        rga.remove(0);
        assert_eq!(rga.to_vec(), vec!['b', 'c']);

        rga.remove(1);
        assert_eq!(rga.to_vec(), vec!['b']);

        rga.remove(0);
        assert!(rga.is_empty());
    }

    #[test]
    fn get_returns_correct_values() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, "hello").unwrap();
        rga.insert_at(1, "world").unwrap();
        assert_eq!(rga.get(0), Some(&"hello"));
        assert_eq!(rga.get(1), Some(&"world"));
        assert_eq!(rga.get(2), None);
    }

    #[test]
    fn iterate_elements() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 10).unwrap();
        rga.insert_at(1, 20).unwrap();
        rga.insert_at(2, 30).unwrap();
        rga.remove(1);

        let elems: Vec<&i32> = rga.iter().collect();
        assert_eq!(elems, vec![&10, &30]);
    }

    #[test]
    fn actor_returns_id() {
        let rga = Rga::<i32>::new("node-42");
        assert_eq!(rga.actor(), "node-42");
    }

    // --- Merge tests ---

    #[test]
    fn merge_disjoint_inserts() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'x').unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 'y').unwrap();

        r1.merge(&r2);
        assert_eq!(r1.len(), 2);
        // Both elements present
        let v = r1.to_vec();
        assert!(v.contains(&'x'));
        assert!(v.contains(&'y'));
    }

    #[test]
    fn merge_concurrent_inserts_at_same_position() {
        // Both replicas start empty and insert at position 0.
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'A').unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 'B').unwrap();

        let mut r1_copy = r1.clone();
        let mut r2_copy = r2.clone();

        r1_copy.merge(&r2);
        r2_copy.merge(&r1);

        // Both replicas must converge to the same order.
        assert_eq!(r1_copy.to_vec(), r2_copy.to_vec());
        assert_eq!(r1_copy.len(), 2);
    }

    #[test]
    fn merge_concurrent_inserts_after_shared_prefix() {
        // Both replicas share a prefix and then insert at the same position.
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'H').unwrap();
        r1.insert_at(1, 'e').unwrap();

        let mut r2 = r1.fork("b");

        // r1 inserts 'X' after 'e'
        r1.insert_at(2, 'X').unwrap();
        // r2 inserts 'Y' after 'e'
        r2.insert_at(2, 'Y').unwrap();

        let mut r1_merged = r1.clone();
        r1_merged.merge(&r2);

        let mut r2_merged = r2.clone();
        r2_merged.merge(&r1);

        assert_eq!(r1_merged.to_vec(), r2_merged.to_vec());
        assert_eq!(r1_merged.len(), 4);

        // Prefix is preserved.
        assert_eq!(r1_merged.get(0), Some(&'H'));
        assert_eq!(r1_merged.get(1), Some(&'e'));
    }

    #[test]
    fn merge_with_deletions() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'a').unwrap();
        r1.insert_at(1, 'b').unwrap();
        r1.insert_at(2, 'c').unwrap();

        let mut r2 = r1.fork("b");

        // r1 removes 'b'
        r1.remove(1);
        // r2 inserts 'd' at end
        r2.insert_at(3, 'd').unwrap();

        r1.merge(&r2);
        // 'b' should be tombstoned, 'd' should be added
        assert!(!r1.to_vec().contains(&'b'));
        assert!(r1.to_vec().contains(&'d'));
        assert_eq!(r1.len(), 3); // a, c, d
    }

    #[test]
    fn merge_is_commutative() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 1).unwrap();
        r1.insert_at(1, 2).unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 3).unwrap();
        r2.insert_at(1, 4).unwrap();

        let mut left = r1.clone();
        left.merge(&r2);

        let mut right = r2.clone();
        right.merge(&r1);

        assert_eq!(left.to_vec(), right.to_vec());
    }

    #[test]
    fn merge_commutativity_with_deletions() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'x').unwrap();
        r1.insert_at(1, 'y').unwrap();

        let mut r2 = r1.fork("b");

        r1.remove(0); // remove 'x'
        r2.insert_at(2, 'z').unwrap();

        let mut left = r1.clone();
        left.merge(&r2);

        let mut right = r2.clone();
        right.merge(&r1);

        assert_eq!(left.to_vec(), right.to_vec());
    }

    #[test]
    fn merge_is_associative() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'A').unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 'B').unwrap();

        let mut r3 = Rga::new("c");
        r3.insert_at(0, 'C').unwrap();

        // (r1 merge r2) merge r3
        let mut left = r1.clone();
        left.merge(&r2);
        left.merge(&r3);

        // r1 merge (r2 merge r3)
        let mut r2_r3 = r2.clone();
        r2_r3.merge(&r3);
        let mut right = r1.clone();
        right.merge(&r2_r3);

        assert_eq!(left.to_vec(), right.to_vec());
    }

    #[test]
    fn merge_is_idempotent() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'x').unwrap();
        r1.insert_at(1, 'y').unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 'z').unwrap();

        r1.merge(&r2);
        let after_first = r1.clone();

        r1.merge(&r2);
        assert_eq!(r1.to_vec(), after_first.to_vec());
        assert_eq!(r1, after_first);
    }

    #[test]
    fn merge_self_is_idempotent() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 1).unwrap();
        rga.insert_at(1, 2).unwrap();
        rga.remove(0);

        let snapshot = rga.clone();
        rga.merge(&snapshot);

        assert_eq!(rga, snapshot);
    }

    #[test]
    fn causal_ordering_preserved() {
        // Build a sequence on one replica, then merge into another.
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'H').unwrap();
        r1.insert_at(1, 'e').unwrap();
        r1.insert_at(2, 'l').unwrap();
        r1.insert_at(3, 'l').unwrap();
        r1.insert_at(4, 'o').unwrap();

        let mut r2 = Rga::new("b");
        r2.merge(&r1);

        assert_eq!(r2.to_vec(), vec!['H', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn causal_ordering_insert_between() {
        let mut rga = Rga::new("a");
        rga.insert_at(0, 1).unwrap();
        rga.insert_at(1, 3).unwrap();
        rga.insert_at(1, 2).unwrap(); // insert 2 between 1 and 3

        assert_eq!(rga.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn three_way_merge_convergence() {
        // Three replicas each insert at position 0 concurrently.
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'A').unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 'B').unwrap();

        let mut r3 = Rga::new("c");
        r3.insert_at(0, 'C').unwrap();

        let mut m1 = r1.clone();
        m1.merge(&r2);
        m1.merge(&r3);

        let mut m2 = r2.clone();
        m2.merge(&r1);
        m2.merge(&r3);

        let mut m3 = r3.clone();
        m3.merge(&r1);
        m3.merge(&r2);

        assert_eq!(m1.to_vec(), m2.to_vec());
        assert_eq!(m2.to_vec(), m3.to_vec());
        assert_eq!(m1.len(), 3);
    }

    #[test]
    fn concurrent_delete_same_element() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'x').unwrap();

        let mut r2 = r1.fork("b");

        // Both replicas delete the same element.
        r1.remove(0);
        r2.remove(0);

        r1.merge(&r2);
        assert!(r1.is_empty());
    }

    #[test]
    fn merge_preserves_existing_order() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 1).unwrap();
        r1.insert_at(1, 2).unwrap();
        r1.insert_at(2, 3).unwrap();
        r1.insert_at(3, 4).unwrap();

        let snapshot = r1.to_vec();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, 10).unwrap();

        r1.merge(&r2);

        // Original elements should still appear in their original relative order.
        let merged = r1.to_vec();
        let original_positions: Vec<usize> = snapshot
            .iter()
            .map(|v| merged.iter().position(|x| x == v).unwrap())
            .collect();

        // The original order should be strictly increasing.
        for w in original_positions.windows(2) {
            assert!(w[0] < w[1]);
        }
    }

    #[test]
    fn empty_merge_empty() {
        let mut r1 = Rga::<i32>::new("a");
        let r2 = Rga::<i32>::new("b");
        r1.merge(&r2);
        assert!(r1.is_empty());
    }

    #[test]
    fn merge_into_empty() {
        let mut r1 = Rga::<char>::new("a");
        let mut r2 = Rga::new("b");
        r2.insert_at(0, 'z').unwrap();

        r1.merge(&r2);
        assert_eq!(r1.to_vec(), vec!['z']);
    }

    #[test]
    fn repeated_insert_remove_cycles() {
        let mut rga = Rga::new("a");
        for i in 0..5 {
            rga.insert_at(0, i).unwrap();
        }
        // rga is [4, 3, 2, 1, 0]
        assert_eq!(rga.len(), 5);

        // Remove all
        while !rga.is_empty() {
            rga.remove(0);
        }
        assert!(rga.is_empty());

        // Re-insert
        rga.insert_at(0, 99).unwrap();
        assert_eq!(rga.to_vec(), vec![99]);
    }

    #[test]
    fn delta_apply_equivalent_to_merge() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'H').unwrap();
        r1.insert_at(1, 'i').unwrap();

        let mut r2 = Rga::new("b");
        r2.insert_at(0, '!').unwrap();

        let mut via_merge = r2.clone();
        via_merge.merge(&r1);

        let mut via_delta = r2.clone();
        let d = r1.delta(&r2);
        via_delta.apply_delta(&d);

        assert_eq!(via_merge.to_vec(), via_delta.to_vec());
    }

    #[test]
    fn delta_with_tombstones() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'a').unwrap();
        r1.insert_at(1, 'b').unwrap();
        r1.insert_at(2, 'c').unwrap();
        r1.remove(1); // remove 'b'

        let r2 = Rga::new("b");

        let mut via_merge = r2.clone();
        via_merge.merge(&r1);

        let mut via_delta = r2.clone();
        let d = r1.delta(&r2);
        via_delta.apply_delta(&d);

        assert_eq!(via_merge.to_vec(), via_delta.to_vec());
        assert_eq!(via_delta.to_vec(), vec!['a', 'c']);
    }

    #[test]
    fn fork_creates_independent_replica() {
        let mut r1 = Rga::new("a");
        r1.insert_at(0, 'x').unwrap();
        r1.insert_at(1, 'y').unwrap();

        let mut r2 = r1.fork("b");
        r2.insert_at(2, 'z').unwrap();

        // r1 should not be affected
        assert_eq!(r1.len(), 2);
        assert_eq!(r2.len(), 3);

        // Should merge cleanly
        r1.merge(&r2);
        assert_eq!(r1.to_vec(), vec!['x', 'y', 'z']);
    }
}
