//! A chunked vector providing O(sqrt(n)) insert/remove for sequence CRDTs.

use alloc::vec::Vec;

const CHUNK_SIZE: usize = 256;

/// A chunked sequence that provides O(sqrt(n)) insert/remove.
/// Each chunk holds up to CHUNK_SIZE elements. When a chunk exceeds
/// 2*CHUNK_SIZE it splits; when adjacent chunks are both below CHUNK_SIZE/2
/// they merge.
pub(crate) struct ChunkedVec<T> {
    chunks: Vec<Vec<T>>,
    total_len: usize,
}

impl<T> ChunkedVec<T> {
    /// Create a new empty `ChunkedVec`.
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            total_len: 0,
        }
    }

    /// Total number of elements.
    pub fn len(&self) -> usize {
        self.total_len
    }

    /// Insert `value` at the given global `index`.
    ///
    /// # Panics
    /// Panics if `index > self.len()`.
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.total_len, "index out of bounds");

        if self.chunks.is_empty() {
            self.chunks.push(Vec::with_capacity(CHUNK_SIZE));
        }

        let (chunk_idx, local_idx) = self.locate_for_insert(index);
        self.chunks[chunk_idx].insert(local_idx, value);
        self.total_len += 1;

        if self.chunks[chunk_idx].len() > 2 * CHUNK_SIZE {
            self.split(chunk_idx);
        }
    }

    /// Remove and return the element at `index`.
    ///
    /// # Panics
    /// Panics if `index >= self.len()`.
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.total_len, "index out of bounds");

        let (chunk_idx, local_idx) = self.locate(index);
        let val = self.chunks[chunk_idx].remove(local_idx);
        self.total_len -= 1;

        // Remove empty chunks
        if self.chunks[chunk_idx].is_empty() {
            self.chunks.remove(chunk_idx);
        } else {
            self.try_merge(chunk_idx);
        }

        val
    }

    /// Get a reference to the element at `index`.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.total_len {
            return None;
        }
        let (chunk_idx, local_idx) = self.locate(index);
        Some(&self.chunks[chunk_idx][local_idx])
    }

    /// Get a mutable reference to the element at `index`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.total_len {
            return None;
        }
        let (chunk_idx, local_idx) = self.locate(index);
        Some(&mut self.chunks[chunk_idx][local_idx])
    }

    /// Iterate over all elements in order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.chunks.iter().flat_map(|c| c.iter())
    }

    /// Append a value to the end.
    pub fn push(&mut self, value: T) {
        let len = self.total_len;
        self.insert(len, value);
    }

    /// Build from a flat `Vec`.
    pub fn from_vec(v: Vec<T>) -> Self {
        let total_len = v.len();
        if total_len == 0 {
            return Self::new();
        }
        let mut chunks = Vec::new();
        let mut iter = v.into_iter();
        loop {
            let chunk: Vec<T> = iter.by_ref().take(CHUNK_SIZE).collect();
            if chunk.is_empty() {
                break;
            }
            chunks.push(chunk);
        }
        Self { chunks, total_len }
    }

    /// Convert into a flat `Vec`.
    pub fn into_vec(self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.total_len);
        for chunk in self.chunks {
            result.extend(chunk);
        }
        result
    }

    /// Find the first element matching the predicate, returning its global index.
    pub fn position<F: Fn(&T) -> bool>(&self, predicate: F) -> Option<usize> {
        let mut global = 0;
        for chunk in &self.chunks {
            for item in chunk {
                if predicate(item) {
                    return Some(global);
                }
                global += 1;
            }
        }
        None
    }

    // --- internal helpers ---

    /// Locate the chunk and local index for a given global index (for access/remove).
    fn locate(&self, index: usize) -> (usize, usize) {
        let mut remaining = index;
        for (ci, chunk) in self.chunks.iter().enumerate() {
            if remaining < chunk.len() {
                return (ci, remaining);
            }
            remaining -= chunk.len();
        }
        panic!("index out of bounds");
    }

    /// Locate for insertion (allows index == chunk.len() for end-of-chunk).
    fn locate_for_insert(&self, index: usize) -> (usize, usize) {
        let mut remaining = index;
        for (ci, chunk) in self.chunks.iter().enumerate() {
            if remaining <= chunk.len() {
                return (ci, remaining);
            }
            remaining -= chunk.len();
        }
        // Append to last chunk
        let last = self.chunks.len() - 1;
        (last, self.chunks[last].len())
    }

    fn split(&mut self, chunk_idx: usize) {
        let mid = self.chunks[chunk_idx].len() / 2;
        let second_half = self.chunks[chunk_idx].split_off(mid);
        self.chunks.insert(chunk_idx + 1, second_half);
    }

    fn try_merge(&mut self, chunk_idx: usize) {
        if self.chunks.len() < 2 {
            return;
        }
        // Try merging with next chunk
        if chunk_idx + 1 < self.chunks.len()
            && self.chunks[chunk_idx].len() + self.chunks[chunk_idx + 1].len() <= CHUNK_SIZE
        {
            let next = self.chunks.remove(chunk_idx + 1);
            self.chunks[chunk_idx].extend(next);
        }
        // Try merging with previous chunk
        else if chunk_idx > 0
            && self.chunks[chunk_idx - 1].len() + self.chunks[chunk_idx].len() <= CHUNK_SIZE
        {
            let current = self.chunks.remove(chunk_idx);
            self.chunks[chunk_idx - 1].extend(current);
        }
    }
}

impl<T: Clone> Clone for ChunkedVec<T> {
    fn clone(&self) -> Self {
        Self {
            chunks: self.chunks.clone(),
            total_len: self.total_len,
        }
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for ChunkedVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ChunkedVec")
            .field("total_len", &self.total_len)
            .field("num_chunks", &self.chunks.len())
            .finish()
    }
}

impl<T: PartialEq> PartialEq for ChunkedVec<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.total_len != other.total_len {
            return false;
        }
        self.iter().zip(other.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Eq> Eq for ChunkedVec<T> {}

impl<T> core::ops::Index<usize> for ChunkedVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> core::ops::IndexMut<usize> for ChunkedVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("index out of bounds")
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for ChunkedVec<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.total_len))?;
        for chunk in &self.chunks {
            for item in chunk {
                seq.serialize_element(item)?;
            }
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for ChunkedVec<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let v: Vec<T> = Vec::deserialize(deserializer)?;
        Ok(Self::from_vec(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_chunked_vec() {
        let cv = ChunkedVec::<i32>::new();
        assert_eq!(cv.len(), 0);
        assert_eq!(cv.get(0), None);
    }

    #[test]
    fn push_and_get() {
        let mut cv = ChunkedVec::new();
        for i in 0..1000 {
            cv.push(i);
        }
        assert_eq!(cv.len(), 1000);
        for i in 0..1000 {
            assert_eq!(cv.get(i), Some(&i));
        }
    }

    #[test]
    fn insert_at_beginning() {
        let mut cv = ChunkedVec::new();
        for i in (0..500).rev() {
            cv.insert(0, i);
        }
        assert_eq!(cv.len(), 500);
        for i in 0..500 {
            assert_eq!(cv.get(i), Some(&i));
        }
    }

    #[test]
    fn remove_elements() {
        let mut cv = ChunkedVec::from_vec((0..100).collect());
        assert_eq!(cv.len(), 100);

        // Remove from the middle
        let val = cv.remove(50);
        assert_eq!(val, 50);
        assert_eq!(cv.len(), 99);

        // Remove from beginning
        let val = cv.remove(0);
        assert_eq!(val, 0);
        assert_eq!(cv.len(), 98);
    }

    #[test]
    fn from_vec_and_into_vec_roundtrip() {
        let original: Vec<i32> = (0..1000).collect();
        let cv = ChunkedVec::from_vec(original.clone());
        assert_eq!(cv.len(), 1000);
        let result = cv.into_vec();
        assert_eq!(result, original);
    }

    #[test]
    fn position_finds_element() {
        let cv = ChunkedVec::from_vec(vec![10, 20, 30, 40, 50]);
        assert_eq!(cv.position(|&x| x == 30), Some(2));
        assert_eq!(cv.position(|&x| x == 99), None);
    }

    #[test]
    fn iter_order() {
        let data: Vec<i32> = (0..600).collect();
        let cv = ChunkedVec::from_vec(data.clone());
        let collected: Vec<&i32> = cv.iter().collect();
        let expected: Vec<&i32> = data.iter().collect();
        assert_eq!(collected, expected);
    }

    #[test]
    fn chunks_split_when_large() {
        let mut cv = ChunkedVec::new();
        for i in 0..600 {
            cv.push(i);
        }
        // Should have split into multiple chunks
        assert!(cv.chunks.len() > 1);
        // Verify correctness
        for i in 0..600 {
            assert_eq!(cv.get(i), Some(&i));
        }
    }
}
