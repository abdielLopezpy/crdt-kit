//! WebAssembly bindings for crdt-kit.
//!
//! Enable with the `wasm` feature:
//!
//! ```toml
//! [dependencies]
//! crdt-kit = { version = "0.5", features = ["wasm"] }
//! ```
//!
//! All types are exposed as JavaScript classes with ergonomic APIs.

use wasm_bindgen::prelude::*;

use crate::Crdt;

// ── GCounter ────────────────────────────────────────────────────────

/// A grow-only counter for use from JavaScript.
#[wasm_bindgen(js_name = GCounter)]
pub struct WasmGCounter {
    inner: crate::GCounter,
}

#[wasm_bindgen(js_class = GCounter)]
impl WasmGCounter {
    /// Create a new G-Counter with the given node ID.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::GCounter::new(actor),
        }
    }

    /// Increment this replica's count by 1.
    pub fn increment(&mut self) {
        self.inner.increment();
    }

    /// Increment this replica's count by `n`.
    #[wasm_bindgen(js_name = incrementBy)]
    pub fn increment_by(&mut self, n: u64) {
        self.inner.increment_by(n);
    }

    /// Get the total counter value across all replicas.
    pub fn value(&self) -> u64 {
        self.inner.value()
    }

    /// Merge another G-Counter's state into this one.
    pub fn merge(&mut self, other: &WasmGCounter) {
        self.inner.merge(&other.inner);
    }
}

// ── PNCounter ───────────────────────────────────────────────────────

/// A positive-negative counter for use from JavaScript.
#[wasm_bindgen(js_name = PNCounter)]
pub struct WasmPNCounter {
    inner: crate::PNCounter,
}

#[wasm_bindgen(js_class = PNCounter)]
impl WasmPNCounter {
    /// Create a new PN-Counter with the given node ID.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::PNCounter::new(actor),
        }
    }

    /// Increment the counter by 1.
    pub fn increment(&mut self) {
        self.inner.increment();
    }

    /// Decrement the counter by 1.
    pub fn decrement(&mut self) {
        self.inner.decrement();
    }

    /// Get the current counter value (increments - decrements).
    pub fn value(&self) -> i64 {
        self.inner.value()
    }

    /// Merge another PN-Counter's state into this one.
    pub fn merge(&mut self, other: &WasmPNCounter) {
        self.inner.merge(&other.inner);
    }
}

// ── LWWRegister ─────────────────────────────────────────────────────

/// A last-writer-wins register for string values, for use from JavaScript.
#[wasm_bindgen(js_name = LWWRegister)]
pub struct WasmLWWRegister {
    inner: crate::LWWRegister<String>,
    clock: crate::clock::HybridClock,
}

#[wasm_bindgen(js_class = LWWRegister)]
impl WasmLWWRegister {
    /// Create a new LWW-Register.
    #[wasm_bindgen(constructor)]
    pub fn new(node_id: u16, value: &str) -> Self {
        let mut clock = crate::clock::HybridClock::new(node_id as u64);
        let inner = crate::LWWRegister::new(value.to_string(), &mut clock);
        Self { inner, clock }
    }

    /// Update the register's value.
    pub fn set(&mut self, value: &str) {
        self.inner.set(value.to_string(), &mut self.clock);
    }

    /// Get the current value.
    pub fn value(&self) -> String {
        self.inner.value().clone()
    }

    /// Merge another LWW-Register's state into this one.
    pub fn merge(&mut self, other: &WasmLWWRegister) {
        self.inner.merge(&other.inner);
    }
}

// ── GSet ────────────────────────────────────────────────────────────

/// A grow-only set of strings for use from JavaScript.
#[wasm_bindgen(js_name = GSet)]
pub struct WasmGSet {
    inner: crate::GSet<String>,
}

impl Default for WasmGSet {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_class = GSet)]
impl WasmGSet {
    /// Create a new empty G-Set.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: crate::GSet::new(),
        }
    }

    /// Insert an element into the set.
    pub fn insert(&mut self, value: &str) -> bool {
        self.inner.insert(value.to_string())
    }

    /// Check if the set contains an element.
    pub fn contains(&self, value: &str) -> bool {
        self.inner.contains(&value.to_string())
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the set is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merge another G-Set's state into this one.
    pub fn merge(&mut self, other: &WasmGSet) {
        self.inner.merge(&other.inner);
    }

    /// Get all elements as a JavaScript array.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Box<[JsValue]> {
        self.inner
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

// ── ORSet ───────────────────────────────────────────────────────────

/// An observed-remove set of strings for use from JavaScript.
#[wasm_bindgen(js_name = ORSet)]
pub struct WasmORSet {
    inner: crate::ORSet<String>,
}

#[wasm_bindgen(js_class = ORSet)]
impl WasmORSet {
    /// Create a new empty OR-Set for the given node.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::ORSet::new(actor),
        }
    }

    /// Insert an element into the set.
    pub fn insert(&mut self, value: &str) {
        self.inner.insert(value.to_string());
    }

    /// Remove an element from the set.
    pub fn remove(&mut self, value: &str) -> bool {
        self.inner.remove(&value.to_string())
    }

    /// Check if the set contains an element.
    pub fn contains(&self, value: &str) -> bool {
        self.inner.contains(&value.to_string())
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the set is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merge another OR-Set's state into this one.
    pub fn merge(&mut self, other: &WasmORSet) {
        self.inner.merge(&other.inner);
    }

    /// Get all elements as a JavaScript array.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Box<[JsValue]> {
        self.inner
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

// ── TwoPSet ─────────────────────────────────────────────────────────

/// A two-phase set of strings for use from JavaScript.
/// Elements can be added and permanently removed.
#[wasm_bindgen(js_name = TwoPSet)]
pub struct WasmTwoPSet {
    inner: crate::TwoPSet<String>,
}

impl Default for WasmTwoPSet {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_class = TwoPSet)]
impl WasmTwoPSet {
    /// Create a new empty Two-Phase Set.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: crate::TwoPSet::new(),
        }
    }

    /// Insert an element into the set.
    pub fn insert(&mut self, value: &str) -> bool {
        self.inner.insert(value.to_string())
    }

    /// Remove an element permanently.
    pub fn remove(&mut self, value: &str) -> bool {
        self.inner.remove(&value.to_string())
    }

    /// Check if the set contains an element.
    pub fn contains(&self, value: &str) -> bool {
        self.inner.contains(&value.to_string())
    }

    /// Get the number of elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the set is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merge another Two-Phase Set's state into this one.
    pub fn merge(&mut self, other: &WasmTwoPSet) {
        self.inner.merge(&other.inner);
    }
}

// ── MVRegister ──────────────────────────────────────────────────────

/// A multi-value register for string values, for use from JavaScript.
/// Preserves all concurrent writes — use `values()` to see conflicts.
#[wasm_bindgen(js_name = MVRegister)]
pub struct WasmMVRegister {
    inner: crate::MVRegister<String>,
}

#[wasm_bindgen(js_class = MVRegister)]
impl WasmMVRegister {
    /// Create a new MV-Register for the given node.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::MVRegister::new(actor),
        }
    }

    /// Set the register's value.
    pub fn set(&mut self, value: &str) {
        self.inner.set(value.to_string());
    }

    /// Get all current values as a JavaScript array.
    /// Multiple values indicate a conflict from concurrent writes.
    pub fn values(&self) -> Box<[JsValue]> {
        self.inner
            .values()
            .into_iter()
            .map(|s| JsValue::from_str(s))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Check if there are conflicting concurrent values.
    #[wasm_bindgen(js_name = isConflicted)]
    pub fn is_conflicted(&self) -> bool {
        self.inner.is_conflicted()
    }

    /// Merge another MV-Register's state into this one.
    pub fn merge(&mut self, other: &WasmMVRegister) {
        self.inner.merge(&other.inner);
    }
}

// ── LWWMap ──────────────────────────────────────────────────────────

/// A last-writer-wins map with string keys and values, for use from JavaScript.
#[wasm_bindgen(js_name = LWWMap)]
pub struct WasmLWWMap {
    inner: crate::LWWMap<String, String>,
    clock: crate::clock::HybridClock,
}

#[wasm_bindgen(js_class = LWWMap)]
impl WasmLWWMap {
    /// Create a new empty LWW-Map.
    #[wasm_bindgen(constructor)]
    pub fn new(node_id: u64) -> Self {
        Self {
            inner: crate::LWWMap::new(),
            clock: crate::clock::HybridClock::new(node_id),
        }
    }

    /// Insert or update a key-value pair.
    pub fn insert(&mut self, key: &str, value: &str) {
        let ts = self.clock.now();
        self.inner.insert(key.to_string(), value.to_string(), ts);
    }

    /// Remove a key.
    pub fn remove(&mut self, key: &str) -> bool {
        let ts = self.clock.now();
        self.inner.remove(&key.to_string(), ts)
    }

    /// Get the value for a key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.get(&key.to_string()).cloned()
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the map is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merge another LWW-Map's state into this one.
    pub fn merge(&mut self, other: &WasmLWWMap) {
        self.inner.merge(&other.inner);
    }
}

// ── AWMap ───────────────────────────────────────────────────────────

/// An add-wins map with string keys and values, for use from JavaScript.
/// Concurrent add beats remove.
#[wasm_bindgen(js_name = AWMap)]
pub struct WasmAWMap {
    inner: crate::AWMap<String, String>,
}

#[wasm_bindgen(js_class = AWMap)]
impl WasmAWMap {
    /// Create a new empty AW-Map for the given node.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::AWMap::new(actor),
        }
    }

    /// Insert or update a key-value pair.
    pub fn insert(&mut self, key: &str, value: &str) {
        self.inner.insert(key.to_string(), value.to_string());
    }

    /// Remove a key.
    pub fn remove(&mut self, key: &str) -> bool {
        self.inner.remove(&key.to_string())
    }

    /// Get the value for a key.
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.get(&key.to_string()).cloned()
    }

    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the map is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merge another AW-Map's state into this one.
    pub fn merge(&mut self, other: &WasmAWMap) {
        self.inner.merge(&other.inner);
    }
}

// ── Rga ─────────────────────────────────────────────────────────────

/// A Replicated Growable Array of strings for use from JavaScript.
#[wasm_bindgen(js_name = Rga)]
pub struct WasmRga {
    inner: crate::Rga<String>,
}

#[wasm_bindgen(js_class = Rga)]
impl WasmRga {
    /// Create a new empty RGA for the given node.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::Rga::new(actor),
        }
    }

    /// Insert an element at the given visible index.
    #[wasm_bindgen(js_name = insertAt)]
    pub fn insert_at(&mut self, index: usize, value: &str) -> bool {
        self.inner.insert_at(index, value.to_string()).is_ok()
    }

    /// Remove the element at the given visible index.
    pub fn remove(&mut self, index: usize) -> Option<String> {
        self.inner.remove(index).ok()
    }

    /// Get the element at the given visible index.
    pub fn get(&self, index: usize) -> Option<String> {
        self.inner.get(index).cloned()
    }

    /// Get the number of visible elements.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the list is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get all visible elements as a JavaScript array.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Box<[JsValue]> {
        self.inner
            .iter()
            .map(|s| JsValue::from_str(s))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    /// Merge another RGA's state into this one.
    pub fn merge(&mut self, other: &WasmRga) {
        self.inner.merge(&other.inner);
    }
}

// ── TextCrdt ────────────────────────────────────────────────────────

/// A collaborative text CRDT for use from JavaScript.
#[wasm_bindgen(js_name = TextCrdt)]
pub struct WasmTextCrdt {
    inner: crate::TextCrdt,
}

#[wasm_bindgen(js_class = TextCrdt)]
impl WasmTextCrdt {
    /// Create a new empty text CRDT for the given node.
    #[wasm_bindgen(constructor)]
    pub fn new(actor: u64) -> Self {
        Self {
            inner: crate::TextCrdt::new(actor),
        }
    }

    /// Insert a single character at the given visible position.
    pub fn insert(&mut self, pos: usize, ch: char) {
        let _ = self.inner.insert(pos, ch);
    }

    /// Insert a string at the given visible position.
    #[wasm_bindgen(js_name = insertStr)]
    pub fn insert_str(&mut self, pos: usize, text: &str) {
        let _ = self.inner.insert_str(pos, text);
    }

    /// Remove the character at the given visible position.
    pub fn remove(&mut self, pos: usize) {
        let _ = self.inner.remove(pos);
    }

    /// Get the current visible text.
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string_js(&self) -> String {
        use alloc::string::ToString;
        self.inner.to_string()
    }

    /// Get the number of visible characters.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Check if the text is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Merge another text CRDT's state into this one.
    pub fn merge(&mut self, other: &WasmTextCrdt) {
        self.inner.merge(&other.inner);
    }
}
