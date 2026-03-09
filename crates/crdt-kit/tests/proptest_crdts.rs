//! Property-based tests for all CRDT types using proptest.
//!
//! Verifies the three fundamental CRDT laws (commutativity, associativity,
//! idempotency) and delta equivalence under random operation sequences.

use crdt_kit::prelude::*;
use proptest::prelude::*;

// ─── Strategies ──────────────────────────────────────────────────────

fn actor_id() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
    ])
}

fn small_u64() -> impl Strategy<Value = u64> {
    0u64..100
}

// ─── GCounter ────────────────────────────────────────────────────────

fn gcounter_with_ops() -> impl Strategy<Value = GCounter> {
    (actor_id(), prop::collection::vec(small_u64(), 0..20)).prop_map(|(actor, ops)| {
        let mut c = GCounter::new(&actor);
        for n in ops {
            c.increment_by(n);
        }
        c
    })
}

proptest! {
    #[test]
    fn gcounter_merge_commutative(a in gcounter_with_ops(), b in gcounter_with_ops()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(ab.value(), ba.value());
    }

    #[test]
    fn gcounter_merge_associative(
        a in gcounter_with_ops(),
        b in gcounter_with_ops(),
        c in gcounter_with_ops(),
    ) {
        let mut ab_c = a.clone();
        ab_c.merge(&b);
        ab_c.merge(&c);

        let mut a_bc = a.clone();
        let mut bc = b.clone();
        bc.merge(&c);
        a_bc.merge(&bc);

        prop_assert_eq!(ab_c.value(), a_bc.value());
    }

    #[test]
    fn gcounter_merge_idempotent(a in gcounter_with_ops(), b in gcounter_with_ops()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.clone();
        merged.merge(&b);
        prop_assert_eq!(merged.value(), first.value());
    }

    #[test]
    fn gcounter_delta_equivalent_to_merge(a in gcounter_with_ops(), b in gcounter_with_ops()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(via_merge.value(), via_delta.value());
    }
}

// ─── PNCounter ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum PNOp {
    Inc,
    Dec,
}

fn pncounter_with_ops() -> impl Strategy<Value = PNCounter> {
    (
        actor_id(),
        prop::collection::vec(prop::sample::select(vec![PNOp::Inc, PNOp::Dec]), 0..20),
    )
        .prop_map(|(actor, ops)| {
            let mut c = PNCounter::new(&actor);
            for op in ops {
                match op {
                    PNOp::Inc => c.increment(),
                    PNOp::Dec => c.decrement(),
                }
            }
            c
        })
}

proptest! {
    #[test]
    fn pncounter_merge_commutative(a in pncounter_with_ops(), b in pncounter_with_ops()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(ab.value(), ba.value());
    }

    #[test]
    fn pncounter_merge_associative(
        a in pncounter_with_ops(),
        b in pncounter_with_ops(),
        c in pncounter_with_ops(),
    ) {
        let mut ab_c = a.clone();
        ab_c.merge(&b);
        ab_c.merge(&c);

        let mut a_bc = a.clone();
        let mut bc = b.clone();
        bc.merge(&c);
        a_bc.merge(&bc);

        prop_assert_eq!(ab_c.value(), a_bc.value());
    }

    #[test]
    fn pncounter_merge_idempotent(a in pncounter_with_ops(), b in pncounter_with_ops()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.clone();
        merged.merge(&b);
        prop_assert_eq!(merged.value(), first.value());
    }

    #[test]
    fn pncounter_delta_equivalent_to_merge(a in pncounter_with_ops(), b in pncounter_with_ops()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(via_merge.value(), via_delta.value());
    }
}

// ─── LWWRegister ─────────────────────────────────────────────────────

fn lww_register() -> impl Strategy<Value = LWWRegister<u32>> {
    (actor_id(), 0u32..1000, 0u64..10000).prop_map(|(actor, val, ts)| {
        LWWRegister::with_timestamp(actor, val, ts)
    })
}

proptest! {
    #[test]
    fn lww_merge_commutative(a in lww_register(), b in lww_register()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(*ab.value(), *ba.value());
    }

    #[test]
    fn lww_merge_idempotent(a in lww_register(), b in lww_register()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.clone();
        merged.merge(&b);
        prop_assert_eq!(*merged.value(), *first.value());
    }

    #[test]
    fn lww_delta_equivalent_to_merge(a in lww_register(), b in lww_register()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(*via_merge.value(), *via_delta.value());
    }
}

// ─── MVRegister ──────────────────────────────────────────────────────

fn mvregister_pair() -> impl Strategy<Value = (MVRegister<u32>, MVRegister<u32>)> {
    (
        prop::collection::vec(0u32..100, 1..5),
        prop::collection::vec(0u32..100, 1..5),
    )
        .prop_map(|(vals_a, vals_b)| {
            let mut a = MVRegister::new("a");
            for v in vals_a {
                a.set(v);
            }
            let mut b = MVRegister::new("b");
            for v in vals_b {
                b.set(v);
            }
            (a, b)
        })
}

proptest! {
    #[test]
    fn mvregister_merge_commutative((a, b) in mvregister_pair()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        let mut ab_vals: Vec<_> = ab.values().into_iter().cloned().collect();
        ab_vals.sort();
        let mut ba_vals: Vec<_> = ba.values().into_iter().cloned().collect();
        ba_vals.sort();
        prop_assert_eq!(ab_vals, ba_vals);
    }

    #[test]
    fn mvregister_merge_idempotent((a, b) in mvregister_pair()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let mut first_vals: Vec<_> = merged.values().into_iter().cloned().collect();
        first_vals.sort();
        merged.merge(&b);
        let mut second_vals: Vec<_> = merged.values().into_iter().cloned().collect();
        second_vals.sort();
        prop_assert_eq!(first_vals, second_vals);
    }

    #[test]
    fn mvregister_delta_equivalent_to_merge((a, b) in mvregister_pair()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        let mut merge_vals: Vec<_> = via_merge.values().into_iter().cloned().collect();
        merge_vals.sort();
        let mut delta_vals: Vec<_> = via_delta.values().into_iter().cloned().collect();
        delta_vals.sort();
        prop_assert_eq!(merge_vals, delta_vals);
    }
}

// ─── GSet ────────────────────────────────────────────────────────────

fn gset_with_ops() -> impl Strategy<Value = GSet<u32>> {
    prop::collection::vec(0u32..50, 0..20).prop_map(|vals| {
        let mut s = GSet::new();
        for v in vals {
            s.insert(v);
        }
        s
    })
}

proptest! {
    #[test]
    fn gset_merge_commutative(a in gset_with_ops(), b in gset_with_ops()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(ab, ba);
    }

    #[test]
    fn gset_merge_associative(
        a in gset_with_ops(),
        b in gset_with_ops(),
        c in gset_with_ops(),
    ) {
        let mut ab_c = a.clone();
        ab_c.merge(&b);
        ab_c.merge(&c);

        let mut a_bc = a.clone();
        let mut bc = b.clone();
        bc.merge(&c);
        a_bc.merge(&bc);

        prop_assert_eq!(ab_c, a_bc);
    }

    #[test]
    fn gset_merge_idempotent(a in gset_with_ops(), b in gset_with_ops()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.clone();
        merged.merge(&b);
        prop_assert_eq!(merged, first);
    }

    #[test]
    fn gset_delta_equivalent_to_merge(a in gset_with_ops(), b in gset_with_ops()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(via_merge, via_delta);
    }
}

// ─── TwoPSet ─────────────────────────────────────────────────────────

fn twopset_with_ops() -> impl Strategy<Value = TwoPSet<u32>> {
    prop::collection::vec((0u32..20, prop::bool::ANY), 0..20).prop_map(|ops| {
        let mut s = TwoPSet::new();
        for (v, remove) in ops {
            if remove {
                s.remove(&v);
            } else {
                s.insert(v);
            }
        }
        s
    })
}

proptest! {
    #[test]
    fn twopset_merge_commutative(a in twopset_with_ops(), b in twopset_with_ops()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(ab, ba);
    }

    #[test]
    fn twopset_merge_associative(
        a in twopset_with_ops(),
        b in twopset_with_ops(),
        c in twopset_with_ops(),
    ) {
        let mut ab_c = a.clone();
        ab_c.merge(&b);
        ab_c.merge(&c);

        let mut a_bc = a.clone();
        let mut bc = b.clone();
        bc.merge(&c);
        a_bc.merge(&bc);

        prop_assert_eq!(ab_c, a_bc);
    }

    #[test]
    fn twopset_merge_idempotent(a in twopset_with_ops(), b in twopset_with_ops()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.clone();
        merged.merge(&b);
        prop_assert_eq!(merged, first);
    }

    #[test]
    fn twopset_delta_equivalent_to_merge(a in twopset_with_ops(), b in twopset_with_ops()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(via_merge, via_delta);
    }
}

// ─── ORSet ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum ORSetOp {
    Insert(u32),
    Remove(u32),
}

fn orset_with_ops() -> impl Strategy<Value = ORSet<u32>> {
    (
        actor_id(),
        prop::collection::vec(
            prop_oneof![
                (0u32..20).prop_map(ORSetOp::Insert),
                (0u32..20).prop_map(ORSetOp::Remove),
            ],
            0..20,
        ),
    )
        .prop_map(|(actor, ops)| {
            let mut s = ORSet::new(&actor);
            for op in ops {
                match op {
                    ORSetOp::Insert(v) => {
                        s.insert(v);
                    }
                    ORSetOp::Remove(v) => {
                        s.remove(&v);
                    }
                }
            }
            s
        })
}

proptest! {
    #[test]
    fn orset_merge_commutative(a in orset_with_ops(), b in orset_with_ops()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);

        let mut ab_items: Vec<_> = ab.iter().cloned().collect();
        ab_items.sort();
        let mut ba_items: Vec<_> = ba.iter().cloned().collect();
        ba_items.sort();
        prop_assert_eq!(ab_items, ba_items);
    }

    #[test]
    fn orset_merge_idempotent(a in orset_with_ops(), b in orset_with_ops()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first: Vec<_> = { let mut v: Vec<_> = merged.iter().cloned().collect(); v.sort(); v };
        merged.merge(&b);
        let second: Vec<_> = { let mut v: Vec<_> = merged.iter().cloned().collect(); v.sort(); v };
        prop_assert_eq!(first, second);
    }

    #[test]
    fn orset_delta_equivalent_to_merge(a in orset_with_ops(), b in orset_with_ops()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        let mut merge_items: Vec<_> = via_merge.iter().cloned().collect();
        merge_items.sort();
        let mut delta_items: Vec<_> = via_delta.iter().cloned().collect();
        delta_items.sort();
        prop_assert_eq!(merge_items, delta_items);
    }

    #[test]
    fn orset_compaction_preserves_semantics(a in orset_with_ops()) {
        let before: Vec<_> = { let mut v: Vec<_> = a.iter().cloned().collect(); v.sort(); v };
        let mut compacted = a.clone();
        compacted.compact_tombstones();
        let after: Vec<_> = { let mut v: Vec<_> = compacted.iter().cloned().collect(); v.sort(); v };
        prop_assert_eq!(before, after);
    }
}

// ─── Rga ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum RgaOp {
    Insert(usize, u32),
    Remove(usize),
}

fn rga_pair() -> impl Strategy<Value = (Rga<u32>, Rga<u32>)> {
    (
        prop::collection::vec(
            prop_oneof![
                (0usize..10, 0u32..100).prop_map(|(i, v)| RgaOp::Insert(i, v)),
                (0usize..10).prop_map(RgaOp::Remove),
            ],
            0..15,
        ),
        prop::collection::vec(
            prop_oneof![
                (0usize..10, 0u32..100).prop_map(|(i, v)| RgaOp::Insert(i, v)),
                (0usize..10).prop_map(RgaOp::Remove),
            ],
            0..15,
        ),
    )
        .prop_map(|(ops_a, ops_b)| {
            let mut a = Rga::new("rga-a");
            for op in ops_a {
                match op {
                    RgaOp::Insert(idx, val) => {
                        let len = a.len();
                        let idx = if len == 0 { 0 } else { idx % (len + 1) };
                        let _ = a.insert_at(idx, val);
                    }
                    RgaOp::Remove(idx) => {
                        let len = a.len();
                        if len > 0 { a.remove(idx % len); }
                    }
                }
            }
            let mut b = Rga::new("rga-b");
            for op in ops_b {
                match op {
                    RgaOp::Insert(idx, val) => {
                        let len = b.len();
                        let idx = if len == 0 { 0 } else { idx % (len + 1) };
                        let _ = b.insert_at(idx, val);
                    }
                    RgaOp::Remove(idx) => {
                        let len = b.len();
                        if len > 0 { b.remove(idx % len); }
                    }
                }
            }
            (a, b)
        })
}

proptest! {
    #[test]
    fn rga_merge_commutative((a, b) in rga_pair()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(ab.to_vec(), ba.to_vec());
    }

    #[test]
    fn rga_merge_idempotent((a, b) in rga_pair()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.to_vec();
        merged.merge(&b);
        prop_assert_eq!(merged.to_vec(), first);
    }

    #[test]
    fn rga_delta_equivalent_to_merge((a, b) in rga_pair()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(via_merge.to_vec(), via_delta.to_vec());
    }
}

// ─── TextCrdt ────────────────────────────────────────────────────────

fn text_pair() -> impl Strategy<Value = (TextCrdt, TextCrdt)> {
    (
        prop::collection::vec(
            prop_oneof![
                (0usize..20, prop::char::range('a', 'z')).prop_map(|(i, c)| (true, i, c)),
                (0usize..20, Just(' ')).prop_map(|(i, _)| (false, i, ' ')),
            ],
            0..10,
        ),
        prop::collection::vec(
            prop_oneof![
                (0usize..20, prop::char::range('a', 'z')).prop_map(|(i, c)| (true, i, c)),
                (0usize..20, Just(' ')).prop_map(|(i, _)| (false, i, ' ')),
            ],
            0..10,
        ),
    )
        .prop_map(|(ops_a, ops_b)| {
            let mut a = TextCrdt::new("text-a");
            for (is_insert, idx, ch) in ops_a {
                let len = a.len();
                if is_insert {
                    let idx = if len == 0 { 0 } else { idx % (len + 1) };
                    let _ = a.insert(idx, ch);
                } else if len > 0 {
                    let _ = a.remove(idx % len);
                }
            }
            let mut b = TextCrdt::new("text-b");
            for (is_insert, idx, ch) in ops_b {
                let len = b.len();
                if is_insert {
                    let idx = if len == 0 { 0 } else { idx % (len + 1) };
                    let _ = b.insert(idx, ch);
                } else if len > 0 {
                    let _ = b.remove(idx % len);
                }
            }
            (a, b)
        })
}

proptest! {
    #[test]
    fn text_merge_commutative((a, b) in text_pair()) {
        let mut ab = a.clone();
        ab.merge(&b);
        let mut ba = b.clone();
        ba.merge(&a);
        prop_assert_eq!(ab.to_string(), ba.to_string());
    }

    #[test]
    fn text_merge_idempotent((a, b) in text_pair()) {
        let mut merged = a.clone();
        merged.merge(&b);
        let first = merged.to_string();
        merged.merge(&b);
        prop_assert_eq!(merged.to_string(), first);
    }

    #[test]
    fn text_delta_equivalent_to_merge((a, b) in text_pair()) {
        let mut via_merge = b.clone();
        via_merge.merge(&a);

        let mut via_delta = b.clone();
        let d = a.delta(&b);
        via_delta.apply_delta(&d);

        prop_assert_eq!(via_merge.to_string(), via_delta.to_string());
    }
}
