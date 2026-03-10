#![no_main]

use arbitrary::Arbitrary;
use crdt_kit::clock::HybridTimestamp;
use crdt_kit::{Crdt, DeltaCrdt, LWWMap};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Op {
    InsertA { key: u8, value: u8, physical: u16, node: u8 },
    InsertB { key: u8, value: u8, physical: u16, node: u8 },
    RemoveA { key: u8, physical: u16, node: u8 },
    Merge,
    DeltaApply,
}

fn ts(physical: u16, node: u8) -> HybridTimestamp {
    HybridTimestamp {
        physical: physical as u64,
        logical: 0,
        node_id: node as u16,
    }
}

fuzz_target!(|ops: Vec<Op>| {
    let mut a = LWWMap::<u8, u8>::new();
    let mut b = LWWMap::<u8, u8>::new();

    for op in ops {
        match op {
            Op::InsertA { key, value, physical, node } => {
                a.insert(key, value, ts(physical, node));
            }
            Op::InsertB { key, value, physical, node } => {
                b.insert(key, value, ts(physical, node));
            }
            Op::RemoveA { key, physical, node } => {
                a.remove(&key, ts(physical, node));
            }
            Op::Merge => {
                let mut ab = a.clone();
                ab.merge(&b);
                let mut ba = b.clone();
                ba.merge(&a);
                assert_eq!(ab, ba, "Commutativity violated");

                // Idempotency
                let snapshot = ab.clone();
                ab.merge(&b);
                assert_eq!(ab, snapshot, "Idempotency violated");
            }
            Op::DeltaApply => {
                let mut via_merge = b.clone();
                via_merge.merge(&a);

                let mut via_delta = b.clone();
                let d = a.delta(&b);
                via_delta.apply_delta(&d);

                assert_eq!(via_merge, via_delta, "Delta != merge");
            }
        }
    }
});
