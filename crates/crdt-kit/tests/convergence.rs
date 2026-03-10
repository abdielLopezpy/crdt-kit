//! Integration tests verifying CRDT convergence properties.
//!
//! For any CRDT, merging replicas in any order must produce the same result.

use crdt_kit::clock::HybridTimestamp;
use crdt_kit::prelude::*;
use std::collections::BTreeSet;

#[test]
fn gcounter_three_way_convergence() {
    let mut a = GCounter::new(1);
    let mut b = GCounter::new(2);
    let mut c = GCounter::new(3);

    a.increment_by(10);
    b.increment_by(20);
    c.increment_by(30);

    let mut order1 = a.clone();
    order1.merge(&b);
    order1.merge(&c);

    let mut order2 = c.clone();
    order2.merge(&a);
    order2.merge(&b);

    let mut order3 = b.clone();
    order3.merge(&c);
    order3.merge(&a);

    assert_eq!(order1.value(), 60);
    assert_eq!(order2.value(), 60);
    assert_eq!(order3.value(), 60);
}

#[test]
fn pncounter_convergence_with_concurrent_ops() {
    let mut a = PNCounter::new(1);
    let mut b = PNCounter::new(2);

    a.increment();
    a.increment();
    a.decrement();

    b.decrement();
    b.decrement();
    b.increment();

    let mut ab = a.clone();
    ab.merge(&b);

    let mut ba = b.clone();
    ba.merge(&a);

    assert_eq!(ab.value(), ba.value());
    assert_eq!(ab.value(), 0);
}

#[test]
fn orset_concurrent_add_remove_convergence() {
    let mut shared = ORSet::new(1);
    shared.insert("item");

    let mut alice = shared.clone();
    let mut bob_set = ORSet::new(2);
    bob_set.insert("item");

    alice.remove(&"item");

    alice.merge(&bob_set);
    assert!(
        alice.contains(&"item"),
        "Concurrent add should survive remove in OR-Set"
    );
}

#[test]
fn twopset_remove_wins_over_concurrent_add() {
    let mut a = TwoPSet::new();
    a.insert("x");
    a.remove(&"x");

    let mut b = TwoPSet::new();
    b.insert("x");

    a.merge(&b);
    assert!(!a.contains(&"x"), "2P-Set: remove should be permanent");
}

#[test]
fn mvregister_preserves_concurrent_writes() {
    let mut a = MVRegister::new(1);
    let mut b = MVRegister::new(2);

    a.set(1);
    b.set(2);

    a.merge(&b);

    let values = a.values();
    assert_eq!(
        values.len(),
        2,
        "Both concurrent values should be preserved"
    );
    assert!(values.contains(&&1));
    assert!(values.contains(&&2));
}

#[test]
fn mvregister_causal_write_supersedes() {
    let mut a = MVRegister::new(1);
    a.set("first");

    let mut b = a.clone();
    b.set("second");

    a.merge(&b);
    assert_eq!(a.values(), vec![&"second"]);
    assert!(!a.is_conflicted());
}

#[test]
fn lww_register_deterministic_resolution() {
    let ts = |ms: u64, node: u16| HybridTimestamp {
        physical: ms,
        logical: 0,
        node_id: node,
    };

    let r1 = LWWRegister::with_timestamp("x", ts(100, 1));
    let r2 = LWWRegister::with_timestamp("y", ts(200, 2));

    let mut merged1 = r1.clone();
    merged1.merge(&r2);

    let mut merged2 = r2.clone();
    merged2.merge(&r1);

    assert_eq!(merged1.value(), merged2.value());
    assert_eq!(*merged1.value(), "y");
}

#[test]
fn gset_union_convergence() {
    let sets: Vec<GSet<u32>> = (0..5)
        .map(|i| {
            let mut s = GSet::new();
            for j in (i * 10)..((i + 1) * 10) {
                s.insert(j);
            }
            s
        })
        .collect();

    let mut result = sets[0].clone();
    for s in &sets[1..] {
        result.merge(s);
    }

    assert_eq!(result.len(), 50);
    for i in 0..50 {
        assert!(result.contains(&i), "Missing element {i}");
    }
}

#[test]
fn repeated_merge_is_idempotent() {
    let mut a = ORSet::new(1);
    a.insert(1);
    a.insert(2);

    let mut b = ORSet::new(2);
    b.insert(2);
    b.insert(3);

    a.merge(&b);
    let snapshot = a.clone();

    a.merge(&b);
    assert_eq!(a, snapshot, "Merge should be idempotent");

    a.merge(&b);
    assert_eq!(a, snapshot, "Merge should be idempotent (3rd time)");
}

#[test]
fn lww_map_three_way_convergence() {
    let ts = |ms: u64, node: u16| HybridTimestamp {
        physical: ms,
        logical: 0,
        node_id: node,
    };

    let mut a = LWWMap::new();
    a.insert("price", 100, ts(1, 1));
    a.insert("stock", 50, ts(2, 1));

    let mut b = LWWMap::new();
    b.insert("price", 120, ts(3, 2));
    b.insert("rating", 5, ts(1, 2));

    let mut c = LWWMap::new();
    c.insert("stock", 45, ts(4, 3));

    let mut order1 = a.clone();
    order1.merge(&b);
    order1.merge(&c);

    let mut order2 = c.clone();
    order2.merge(&a);
    order2.merge(&b);

    let mut order3 = b.clone();
    order3.merge(&c);
    order3.merge(&a);

    assert_eq!(order1, order2);
    assert_eq!(order2, order3);
    assert_eq!(order1.get(&"price"), Some(&120));
    assert_eq!(order1.get(&"stock"), Some(&45));
    assert_eq!(order1.get(&"rating"), Some(&5));
}

#[test]
fn lww_map_remove_propagates_across_replicas() {
    let ts = |ms: u64, node: u16| HybridTimestamp {
        physical: ms,
        logical: 0,
        node_id: node,
    };

    let mut a = LWWMap::new();
    a.insert("k", "v", ts(1, 1));

    let mut b = a.clone();
    b.remove(&"k", ts(2, 2));

    // Stale insert on a should NOT resurrect
    a.insert("k", "stale", ts(1, 3));
    a.merge(&b);
    assert!(!a.contains_key(&"k"));
}

#[test]
fn aw_map_concurrent_add_remove_convergence() {
    let mut shared = AWMap::new(1);
    shared.insert("config", "v1");

    let mut alice = shared.clone();
    let mut bob = AWMap::new(2);
    bob.insert("config", "v2");

    alice.remove(&"config");

    alice.merge(&bob);
    assert!(
        alice.contains_key(&"config"),
        "Concurrent add should survive remove in AW-Map"
    );
}

#[test]
fn aw_map_three_way_convergence_keys() {
    let mut a = AWMap::new(1);
    a.insert("x", 1);
    a.insert("y", 2);

    let mut b = AWMap::new(2);
    b.insert("y", 3);
    b.insert("z", 4);

    let mut c = AWMap::new(3);
    c.insert("x", 5);
    c.insert("z", 6);

    let mut order1 = a.clone();
    order1.merge(&b);
    order1.merge(&c);
    let keys1: BTreeSet<_> = order1.keys().collect();

    let mut order2 = c.clone();
    order2.merge(&a);
    order2.merge(&b);
    let keys2: BTreeSet<_> = order2.keys().collect();

    let mut order3 = b.clone();
    order3.merge(&c);
    order3.merge(&a);
    let keys3: BTreeSet<_> = order3.keys().collect();

    assert_eq!(keys1, keys2);
    assert_eq!(keys2, keys3);
    assert_eq!(keys1.len(), 3);
}

// Compile-time assertions: all CRDT types must be Send + Sync.
fn _assert_send<T: Send>() {}
fn _assert_sync<T: Sync>() {}

#[test]
fn all_types_are_send_and_sync() {
    _assert_send::<AWMap<String, String>>();
    _assert_sync::<AWMap<String, String>>();
    _assert_send::<GCounter>();
    _assert_sync::<GCounter>();
    _assert_send::<LWWMap<String, String>>();
    _assert_sync::<LWWMap<String, String>>();
    _assert_send::<PNCounter>();
    _assert_sync::<PNCounter>();
    _assert_send::<LWWRegister<String>>();
    _assert_sync::<LWWRegister<String>>();
    _assert_send::<MVRegister<String>>();
    _assert_sync::<MVRegister<String>>();
    _assert_send::<GSet<String>>();
    _assert_sync::<GSet<String>>();
    _assert_send::<TwoPSet<String>>();
    _assert_sync::<TwoPSet<String>>();
    _assert_send::<ORSet<String>>();
    _assert_sync::<ORSet<String>>();
    _assert_send::<Rga<String>>();
    _assert_sync::<Rga<String>>();
    _assert_send::<TextCrdt>();
    _assert_sync::<TextCrdt>();
}
