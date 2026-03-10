#![no_main]

use arbitrary::Arbitrary;
use crdt_kit::{Crdt, DeltaCrdt, TextCrdt};
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
enum Op {
    InsertA(u8, char),
    InsertB(u8, char),
    RemoveA(u8),
    InsertStrA(u8),
    Merge,
    DeltaApply,
}

fuzz_target!(|ops: Vec<Op>| {
    let mut a = TextCrdt::new(1);
    let mut b = TextCrdt::new(2);

    for op in ops {
        match op {
            Op::InsertA(pos, ch) => {
                if ch.is_ascii_alphanumeric() {
                    let idx = if a.len() == 0 { 0 } else { (pos as usize) % (a.len() + 1) };
                    let _ = a.insert(idx, ch);
                }
            }
            Op::InsertB(pos, ch) => {
                if ch.is_ascii_alphanumeric() {
                    let idx = if b.len() == 0 { 0 } else { (pos as usize) % (b.len() + 1) };
                    let _ = b.insert(idx, ch);
                }
            }
            Op::RemoveA(pos) => {
                if a.len() > 0 {
                    let idx = (pos as usize) % a.len();
                    let _ = a.remove(idx);
                }
            }
            Op::InsertStrA(pos) => {
                let idx = if a.len() == 0 { 0 } else { (pos as usize) % (a.len() + 1) };
                let _ = a.insert_str(idx, "fuzz");
            }
            Op::Merge => {
                let mut ab = a.clone();
                ab.merge(&b);
                let mut ba = b.clone();
                ba.merge(&a);

                let ab_text = format!("{ab}");
                let ba_text = format!("{ba}");
                assert_eq!(ab_text, ba_text, "Commutativity violated");

                // Idempotency
                let snapshot = format!("{ab}");
                ab.merge(&b);
                let after = format!("{ab}");
                assert_eq!(snapshot, after, "Idempotency violated");
            }
            Op::DeltaApply => {
                let mut via_merge = b.clone();
                via_merge.merge(&a);

                let mut via_delta = b.clone();
                let d = a.delta(&b);
                via_delta.apply_delta(&d);

                let merge_text = format!("{via_merge}");
                let delta_text = format!("{via_delta}");
                assert_eq!(merge_text, delta_text, "Delta != merge");
            }
        }
    }
});
