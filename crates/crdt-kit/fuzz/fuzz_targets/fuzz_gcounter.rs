#![no_main]

use arbitrary::Arbitrary;
use crdt_kit::{Crdt, DeltaCrdt, GCounter};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Op {
    Increment(u64),
    IncrementBy(u64),
    Merge,
    DeltaApply,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut a = GCounter::new(1);
    let mut b = GCounter::new(2);

    for op in ops {
        match op {
            Op::Increment(_) => a.increment(),
            Op::IncrementBy(n) => a.increment_by(n % 1_000_000),
            Op::Merge => {
                let mut ab = a.clone();
                ab.merge(&b);
                let mut ba = b.clone();
                ba.merge(&a);
                // Commutativity
                assert_eq!(ab.value(), ba.value());
            }
            Op::DeltaApply => {
                let mut via_merge = b.clone();
                via_merge.merge(&a);

                let mut via_delta = b.clone();
                let d = a.delta(&b);
                via_delta.apply_delta(&d);

                assert_eq!(via_merge.value(), via_delta.value());
            }
        }
    }
});
