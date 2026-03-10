#![no_main]

use arbitrary::Arbitrary;
use crdt_kit::{Crdt, DeltaCrdt, Rga};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Op {
    InsertA(u8, u8),
    InsertB(u8, u8),
    RemoveA(u8),
    Merge,
    DeltaApply,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut a = Rga::new(1);
    let mut b = Rga::new(2);

    for op in ops {
        match op {
            Op::InsertA(pos, val) => {
                let idx = if a.len() == 0 { 0 } else { (pos as usize) % (a.len() + 1) };
                let _ = a.insert_at(idx, val);
            }
            Op::InsertB(pos, val) => {
                let idx = if b.len() == 0 { 0 } else { (pos as usize) % (b.len() + 1) };
                let _ = b.insert_at(idx, val);
            }
            Op::RemoveA(pos) => {
                if a.len() > 0 {
                    let idx = (pos as usize) % a.len();
                    let _ = a.remove(idx);
                }
            }
            Op::Merge => {
                let mut ab = a.clone();
                ab.merge(&b);
                let mut ba = b.clone();
                ba.merge(&a);

                let ab_items: Vec<_> = ab.iter().collect();
                let ba_items: Vec<_> = ba.iter().collect();
                assert_eq!(ab_items, ba_items, "Commutativity violated");

                // Idempotency
                let snapshot: Vec<_> = ab.iter().collect();
                ab.merge(&b);
                let after: Vec<_> = ab.iter().collect();
                assert_eq!(snapshot, after, "Idempotency violated");
            }
            Op::DeltaApply => {
                let mut via_merge = b.clone();
                via_merge.merge(&a);

                let mut via_delta = b.clone();
                let d = a.delta(&b);
                via_delta.apply_delta(&d);

                let merge_items: Vec<_> = via_merge.iter().collect();
                let delta_items: Vec<_> = via_delta.iter().collect();
                assert_eq!(merge_items, delta_items, "Delta != merge");
            }
        }
    }
});
