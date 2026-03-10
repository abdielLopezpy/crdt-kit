#![no_main]

use arbitrary::Arbitrary;
use crdt_kit::{Crdt, DeltaCrdt, ORSet};
use libfuzzer_sys::fuzz_target;
use std::collections::BTreeSet;

#[derive(Arbitrary, Debug)]
enum Op {
    InsertA(u8),
    InsertB(u8),
    RemoveA(u8),
    Merge,
    DeltaApply,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut a = ORSet::new(1);
    let mut b = ORSet::new(2);

    for op in ops {
        match op {
            Op::InsertA(v) => { a.insert(v); }
            Op::InsertB(v) => { b.insert(v); }
            Op::RemoveA(v) => { a.remove(&v); }
            Op::Merge => {
                let mut ab = a.clone();
                ab.merge(&b);
                let mut ba = b.clone();
                ba.merge(&a);

                let ab_elems: BTreeSet<_> = ab.iter().collect();
                let ba_elems: BTreeSet<_> = ba.iter().collect();
                assert_eq!(ab_elems, ba_elems, "Commutativity violated");

                // Idempotency
                let snapshot = ab.clone();
                ab.merge(&b);
                let ab_after: BTreeSet<_> = ab.iter().collect();
                let snap_elems: BTreeSet<_> = snapshot.iter().collect();
                assert_eq!(ab_after, snap_elems, "Idempotency violated");
            }
            Op::DeltaApply => {
                let mut via_merge = b.clone();
                via_merge.merge(&a);

                let mut via_delta = b.clone();
                let d = a.delta(&b);
                via_delta.apply_delta(&d);

                let merge_elems: BTreeSet<_> = via_merge.iter().collect();
                let delta_elems: BTreeSet<_> = via_delta.iter().collect();
                assert_eq!(merge_elems, delta_elems, "Delta != merge");
            }
        }
    }
});
