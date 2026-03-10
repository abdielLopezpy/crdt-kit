#![no_main]

use arbitrary::Arbitrary;
use crdt_kit::{AWMap, Crdt, DeltaCrdt};
use libfuzzer_sys::fuzz_target;
use std::collections::BTreeSet;

#[derive(Arbitrary, Debug)]
enum Op {
    InsertA(u8, u8),
    InsertB(u8, u8),
    RemoveA(u8),
    RemoveB(u8),
    Merge,
    DeltaApply,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut a = AWMap::new(1);
    let mut b = AWMap::new(2);

    for op in ops {
        match op {
            Op::InsertA(k, v) => { a.insert(k, v); }
            Op::InsertB(k, v) => { b.insert(k, v); }
            Op::RemoveA(k) => { a.remove(&k); }
            Op::RemoveB(k) => { b.remove(&k); }
            Op::Merge => {
                let mut ab = a.clone();
                ab.merge(&b);
                let mut ba = b.clone();
                ba.merge(&a);

                let ab_keys: BTreeSet<_> = ab.keys().collect();
                let ba_keys: BTreeSet<_> = ba.keys().collect();
                assert_eq!(ab_keys, ba_keys, "Commutativity violated (keys)");

                // Idempotency
                let snapshot_keys: BTreeSet<_> = ab.keys().collect();
                ab.merge(&b);
                let after_keys: BTreeSet<_> = ab.keys().collect();
                assert_eq!(snapshot_keys, after_keys, "Idempotency violated");
            }
            Op::DeltaApply => {
                let mut via_merge = b.clone();
                via_merge.merge(&a);

                let mut via_delta = b.clone();
                let d = a.delta(&b);
                via_delta.apply_delta(&d);

                let merge_keys: BTreeSet<_> = via_merge.keys().collect();
                let delta_keys: BTreeSet<_> = via_delta.keys().collect();
                assert_eq!(merge_keys, delta_keys, "Delta != merge (keys)");
            }
        }
    }
});
