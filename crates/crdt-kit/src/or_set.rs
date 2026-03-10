use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use crate::{Crdt, DeltaCrdt, NodeId};

/// An observed-remove set (OR-Set), also known as an add-wins set.
///
/// Unlike the 2P-Set, elements can be freely added and removed, and
/// re-added after removal. Each add operation generates a unique tag.
/// Remove only removes the tags that the remover has observed, so
/// concurrent adds are preserved.
///
/// # Example
///
/// ```
/// use crdt_kit::prelude::*;
///
/// let mut s1 = ORSet::new(1);
/// s1.insert("apple");
/// s1.insert("banana");
/// s1.remove(&"banana");
///
/// let mut s2 = ORSet::new(2);
/// s2.insert("banana"); // concurrent add
///
/// s1.merge(&s2);
/// // banana is present because s2's add was concurrent with s1's remove
/// assert!(s1.contains(&"banana"));
/// assert!(s1.contains(&"apple"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ORSet<T: Ord + Clone> {
    actor: NodeId,
    counter: u64,
    /// element -> set of unique tags (actor, counter)
    elements: BTreeMap<T, BTreeSet<(NodeId, u64)>>,
    /// Tombstones: tags that have been removed
    tombstones: BTreeSet<(NodeId, u64)>,
}

impl<T: Ord + Clone> ORSet<T> {
    /// Create a new empty OR-Set for the given node.
    pub fn new(actor: NodeId) -> Self {
        Self {
            actor,
            counter: 0,
            elements: BTreeMap::new(),
            tombstones: BTreeSet::new(),
        }
    }

    /// Insert an element into the set.
    ///
    /// Generates a unique tag for this insertion. Even if the element
    /// was previously removed, this new tag allows it to be re-added.
    pub fn insert(&mut self, value: T) {
        self.counter += 1;
        let tag = (self.actor, self.counter);
        self.elements.entry(value).or_default().insert(tag);
    }

    /// Remove an element from the set.
    ///
    /// Only removes the tags that this replica has observed. Concurrent
    /// adds on other replicas will survive the merge.
    ///
    /// Returns `true` if the element was present and removed.
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(tags) = self.elements.remove(value) {
            self.tombstones.extend(tags);
            true
        } else {
            false
        }
    }

    /// Check if the set contains an element.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.elements
            .get(value)
            .is_some_and(|tags| !tags.is_empty())
    }

    /// Get the number of distinct elements in the set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.elements
            .values()
            .filter(|tags| !tags.is_empty())
            .count()
    }

    /// Check if the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over the elements in the set.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements
            .iter()
            .filter(|(_, tags)| !tags.is_empty())
            .map(|(v, _)| v)
    }

    /// Get this replica's node ID.
    #[must_use]
    pub fn actor(&self) -> NodeId {
        self.actor
    }

    /// Get the number of tombstones stored.
    #[must_use]
    pub fn tombstone_count(&self) -> usize {
        self.tombstones.len()
    }

    /// Remove tombstones that are no longer needed.
    ///
    /// A tombstone is only needed while it might suppress a tag that hasn't
    /// been propagated yet. Once no live element carries the same tag, the
    /// tombstone is safe to discard.
    ///
    /// **Note:** this is a local-only GC. It is safe to call at any time, but
    /// for best results call it after all peers have converged.
    ///
    /// Returns the number of tombstones removed.
    pub fn compact_tombstones(&mut self) -> usize {
        let live_tags: BTreeSet<&(NodeId, u64)> = self
            .elements
            .values()
            .flat_map(|tags| tags.iter())
            .collect();

        let before = self.tombstones.len();
        // Keep only tombstones whose tag is still live somewhere (shouldn't
        // normally happen, but guards against partial merges). All others
        // are safely discarded.
        self.tombstones.retain(|tag| live_tags.contains(tag));
        before - self.tombstones.len()
    }

    /// Aggressively remove **all** tombstones.
    ///
    /// This is safe only when every peer has converged to the same state.
    ///
    /// Returns the number of tombstones removed.
    pub fn compact_tombstones_all(&mut self) -> usize {
        let count = self.tombstones.len();
        self.tombstones.clear();
        count
    }
}

impl<T: Ord + Clone> IntoIterator for ORSet<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let items: Vec<T> = self
            .elements
            .into_iter()
            .filter(|(_, tags)| !tags.is_empty())
            .map(|(v, _)| v)
            .collect();
        items.into_iter()
    }
}

impl<T: Ord + Clone> Crdt for ORSet<T> {
    fn merge(&mut self, other: &Self) {
        for (value, other_tags) in &other.elements {
            let self_tags = self.elements.entry(value.clone()).or_default();
            for &tag in other_tags {
                if !self.tombstones.contains(&tag) {
                    self_tags.insert(tag);
                }
            }
        }

        for &tag in &other.tombstones {
            for tags in self.elements.values_mut() {
                tags.remove(&tag);
            }
        }

        self.tombstones.extend(&other.tombstones);

        self.elements.retain(|_, tags| !tags.is_empty());

        self.counter = self.counter.max(other.counter);
    }
}

/// Delta for [`ORSet`]: new element tags and new tombstones since another state.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ORSetDelta<T: Ord + Clone> {
    /// New element-tag pairs that the other replica doesn't have.
    additions: BTreeMap<T, BTreeSet<(NodeId, u64)>>,
    /// New tombstones that the other replica doesn't have.
    tombstones: BTreeSet<(NodeId, u64)>,
}

impl<T: Ord + Clone> DeltaCrdt for ORSet<T> {
    type Delta = ORSetDelta<T>;

    fn delta(&self, other: &Self) -> ORSetDelta<T> {
        let mut additions = BTreeMap::new();
        for (value, self_tags) in &self.elements {
            let other_tags = other.elements.get(value);
            let new_tags: BTreeSet<_> = self_tags
                .iter()
                .filter(|tag| {
                    other_tags.map_or(true, |ot| !ot.contains(*tag))
                        && !other.tombstones.contains(*tag)
                })
                .copied()
                .collect();
            if !new_tags.is_empty() {
                additions.insert(value.clone(), new_tags);
            }
        }

        let tombstones: BTreeSet<_> = self
            .tombstones
            .difference(&other.tombstones)
            .copied()
            .collect();

        ORSetDelta {
            additions,
            tombstones,
        }
    }

    fn apply_delta(&mut self, delta: &ORSetDelta<T>) {
        for (value, tags) in &delta.additions {
            let self_tags = self.elements.entry(value.clone()).or_default();
            for &tag in tags {
                if !self.tombstones.contains(&tag) {
                    self_tags.insert(tag);
                }
            }
        }

        for &tag in &delta.tombstones {
            for tags in self.elements.values_mut() {
                tags.remove(&tag);
            }
        }
        self.tombstones.extend(&delta.tombstones);

        self.elements.retain(|_, tags| !tags.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_set_is_empty() {
        let s = ORSet::<String>::new(1);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn insert_and_contains() {
        let mut s = ORSet::new(1);
        s.insert("x");
        assert!(s.contains(&"x"));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn remove_element() {
        let mut s = ORSet::new(1);
        s.insert("x");
        assert!(s.remove(&"x"));
        assert!(!s.contains(&"x"));
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn can_readd_after_remove() {
        let mut s = ORSet::new(1);
        s.insert("x");
        s.remove(&"x");
        assert!(!s.contains(&"x"));

        s.insert("x");
        assert!(s.contains(&"x"));
    }

    #[test]
    fn concurrent_add_survives_remove() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");
        s1.remove(&"x");

        let mut s2 = ORSet::new(2);
        s2.insert("x");

        s1.merge(&s2);
        assert!(s1.contains(&"x"));
    }

    #[test]
    fn merge_is_commutative() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");
        s1.insert("y");

        let mut s2 = ORSet::new(2);
        s2.insert("y");
        s2.insert("z");

        let mut left = s1.clone();
        left.merge(&s2);

        let mut right = s2.clone();
        right.merge(&s1);

        let left_elems: BTreeSet<_> = left.iter().collect();
        let right_elems: BTreeSet<_> = right.iter().collect();
        assert_eq!(left_elems, right_elems);
    }

    #[test]
    fn merge_is_idempotent() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");

        let mut s2 = ORSet::new(2);
        s2.insert("y");

        s1.merge(&s2);
        let after_first = s1.clone();
        s1.merge(&s2);

        assert_eq!(s1, after_first);
    }

    #[test]
    fn add_wins_semantics() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");
        s1.remove(&"x");

        let mut s2 = ORSet::new(2);
        s2.insert("x");

        s1.merge(&s2);
        assert!(s1.contains(&"x"));
    }

    #[test]
    fn remove_nonexistent_returns_false() {
        let mut s = ORSet::<&str>::new(1);
        assert!(!s.remove(&"x"));
    }

    #[test]
    fn iterate_elements() {
        let mut s = ORSet::new(1);
        s.insert(1);
        s.insert(2);
        s.insert(3);
        s.remove(&2);

        let elems: Vec<&i32> = s.iter().collect();
        assert_eq!(elems, vec![&1, &3]);
    }

    #[test]
    fn delta_apply_equivalent_to_merge() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");
        s1.insert("y");
        s1.remove(&"x");

        let mut s2 = ORSet::new(2);
        s2.insert("y");
        s2.insert("z");

        let mut full = s2.clone();
        full.merge(&s1);

        let mut via_delta = s2.clone();
        let d = s1.delta(&s2);
        via_delta.apply_delta(&d);

        let full_elems: BTreeSet<_> = full.iter().collect();
        let delta_elems: BTreeSet<_> = via_delta.iter().collect();
        assert_eq!(full_elems, delta_elems);
    }

    #[test]
    fn delta_is_empty_when_equal() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");

        let s2 = s1.clone();
        let d = s1.delta(&s2);
        assert!(d.additions.is_empty());
        assert!(d.tombstones.is_empty());
    }

    #[test]
    fn tombstone_count_tracks_removals() {
        let mut s = ORSet::new(1);
        s.insert("x");
        s.insert("y");
        assert_eq!(s.tombstone_count(), 0);

        s.remove(&"x");
        assert_eq!(s.tombstone_count(), 1);

        s.remove(&"y");
        assert_eq!(s.tombstone_count(), 2);
    }

    #[test]
    fn compact_tombstones_removes_dangling() {
        let mut s = ORSet::new(1);
        s.insert("x");
        s.insert("y");
        s.remove(&"x");
        s.remove(&"y");

        assert_eq!(s.tombstone_count(), 2);
        let removed = s.compact_tombstones();
        assert_eq!(removed, 2);
        assert_eq!(s.tombstone_count(), 0);
    }

    #[test]
    fn compact_tombstones_all_clears_everything() {
        let mut s = ORSet::new(1);
        s.insert("x");
        s.remove(&"x");
        s.insert("y");
        s.remove(&"y");

        assert_eq!(s.compact_tombstones_all(), 2);
        assert_eq!(s.tombstone_count(), 0);
    }

    #[test]
    fn delta_carries_tombstones() {
        let mut s1 = ORSet::new(1);
        s1.insert("x");

        let s2 = s1.clone();
        s1.remove(&"x");

        let d = s1.delta(&s2);
        assert!(!d.tombstones.is_empty());

        let mut via_delta = s2.clone();
        via_delta.apply_delta(&d);
        assert!(!via_delta.contains(&"x"));
    }
}
