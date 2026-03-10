//! Comparative benchmarks: crdt-kit vs Automerge vs Yrs
//!
//! Run with: cargo bench --bench comparative

use automerge::ReadDoc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yrs::updates::decoder::Decode;
use yrs::{GetString, ReadTxn};

// ---- Counter: crdt-kit GCounter vs Automerge Counter ----

fn bench_counter_increment(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter_increment_x1000");

    group.bench_function("crdt-kit GCounter", |b| {
        use crdt_kit::prelude::*;
        b.iter(|| {
            let mut counter = GCounter::new(1);
            for _ in 0..1000 {
                counter.increment();
            }
            black_box(counter.value())
        })
    });

    group.bench_function("automerge Counter", |b| {
        use automerge::{transaction::Transactable, AutoCommit, ROOT};
        b.iter(|| {
            let mut doc = AutoCommit::new();
            doc.put(ROOT, "counter", automerge::ScalarValue::counter(0))
                .unwrap();
            for _ in 0..1000 {
                doc.increment(ROOT, "counter", 1).unwrap();
            }
            black_box(doc.length(ROOT))
        })
    });

    group.finish();
}

// ---- Text insert: crdt-kit TextCrdt vs Yrs Y.Text vs Automerge Text ----

fn bench_text_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_insert_1000_chars");

    group.bench_function("crdt-kit TextCrdt", |b| {
        use crdt_kit::prelude::*;
        b.iter(|| {
            let mut t = TextCrdt::new(1);
            for i in 0..1000 {
                t.insert(i, 'a').unwrap();
            }
            black_box(t.len())
        })
    });

    group.bench_function("yrs Y.Text", |b| {
        use yrs::{Doc, Text, Transact};
        b.iter(|| {
            let doc = Doc::new();
            let text = doc.get_or_insert_text("text");
            let mut txn = doc.transact_mut();
            for i in 0..1000u32 {
                text.insert(&mut txn, i, "a");
            }
            black_box(text.get_string(&txn).len())
        })
    });

    group.bench_function("automerge Text", |b| {
        use automerge::{transaction::Transactable, AutoCommit, ObjType, ROOT};
        b.iter(|| {
            let mut doc = AutoCommit::new();
            let text = doc.put_object(ROOT, "text", ObjType::Text).unwrap();
            for i in 0..1000u32 {
                doc.insert(&text, i as usize, "a").unwrap();
            }
            black_box(doc.text(&text).unwrap().len())
        })
    });

    group.finish();
}

// ---- Text merge: crdt-kit vs Yrs vs Automerge ----

fn bench_text_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_merge_500+500_chars");

    group.bench_function("crdt-kit TextCrdt", |b| {
        use crdt_kit::prelude::*;
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, &"a".repeat(500)).unwrap();
        let mut t2 = TextCrdt::new(2);
        t2.insert_str(0, &"b".repeat(500)).unwrap();

        b.iter(|| {
            let mut merged = t1.clone();
            merged.merge(&t2);
            black_box(merged.len())
        })
    });

    group.bench_function("yrs Y.Text", |b| {
        use yrs::{Doc, Options, Text, Transact, Update};

        let doc1 = Doc::with_options(Options {
            client_id: 1,
            ..Default::default()
        });
        let text1 = doc1.get_or_insert_text("text");
        {
            let mut txn = doc1.transact_mut();
            text1.insert(&mut txn, 0, &"a".repeat(500));
        }

        let doc2 = Doc::with_options(Options {
            client_id: 2,
            ..Default::default()
        });
        let text2 = doc2.get_or_insert_text("text");
        {
            let mut txn = doc2.transact_mut();
            text2.insert(&mut txn, 0, &"b".repeat(500));
        }

        let update2 = doc2
            .transact()
            .encode_state_as_update_v1(&yrs::StateVector::default());

        b.iter(|| {
            let merged = Doc::with_options(Options {
                client_id: 3,
                ..Default::default()
            });
            let _text = merged.get_or_insert_text("text");
            {
                let mut txn = merged.transact_mut();
                let update1 = doc1
                    .transact()
                    .encode_state_as_update_v1(&yrs::StateVector::default());
                let _ = txn.apply_update(Update::decode_v1(&update1).unwrap());
                let _ = txn.apply_update(Update::decode_v1(&update2).unwrap());
            }
            let txn = merged.transact();
            let t = merged.get_or_insert_text("text");
            black_box(t.get_string(&txn).len())
        })
    });

    group.bench_function("automerge Text", |b| {
        use automerge::{transaction::Transactable, AutoCommit, ObjType, ROOT};
        let mut doc1 = AutoCommit::new();
        let text1 = doc1.put_object(ROOT, "text", ObjType::Text).unwrap();
        for i in 0..500 {
            doc1.insert(&text1, i, "a").unwrap();
        }

        let mut doc2 = AutoCommit::new();
        let text2 = doc2.put_object(ROOT, "text", ObjType::Text).unwrap();
        for i in 0..500 {
            doc2.insert(&text2, i, "b").unwrap();
        }

        let changes1 = doc1.save();
        let changes2 = doc2.save();

        b.iter(|| {
            let mut merged = AutoCommit::new();
            merged.load_incremental(&changes1).unwrap();
            merged.load_incremental(&changes2).unwrap();
            black_box(merged.length(ROOT))
        })
    });

    group.finish();
}

// ---- List/Array: crdt-kit Rga vs Yrs Y.Array vs Automerge List ----

fn bench_list_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("list_insert_1000_elements");

    group.bench_function("crdt-kit Rga", |b| {
        use crdt_kit::prelude::*;
        b.iter(|| {
            let mut rga = Rga::new(1);
            for i in 0..1000u32 {
                rga.insert_at(i as usize, i).unwrap();
            }
            black_box(rga.len())
        })
    });

    group.bench_function("yrs Y.Array", |b| {
        use yrs::{Array, Doc, Transact};
        b.iter(|| {
            let doc = Doc::new();
            let arr = doc.get_or_insert_array("list");
            let mut txn = doc.transact_mut();
            for i in 0..1000u32 {
                arr.insert(&mut txn, i, i as i64);
            }
            black_box(arr.len(&txn))
        })
    });

    group.bench_function("automerge List", |b| {
        use automerge::{transaction::Transactable, AutoCommit, ObjType, ROOT};
        b.iter(|| {
            let mut doc = AutoCommit::new();
            let list = doc.put_object(ROOT, "list", ObjType::List).unwrap();
            for i in 0..1000u32 {
                doc.insert(&list, i as usize, i as i64).unwrap();
            }
            black_box(doc.length(&list))
        })
    });

    group.finish();
}

// ---- Set operations: crdt-kit ORSet vs Automerge Map keys ----

fn bench_set_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_insert_1000");

    group.bench_function("crdt-kit ORSet", |b| {
        use crdt_kit::prelude::*;
        b.iter(|| {
            let mut set = ORSet::new(1);
            for i in 0..1000u32 {
                set.insert(i);
            }
            black_box(set.len())
        })
    });

    group.bench_function("automerge Map (set emulation)", |b| {
        use automerge::{transaction::Transactable, AutoCommit, ObjType, ROOT};
        b.iter(|| {
            let mut doc = AutoCommit::new();
            let map = doc.put_object(ROOT, "set", ObjType::Map).unwrap();
            for i in 0..1000u32 {
                doc.put(&map, format!("{i}"), true).unwrap();
            }
            black_box(doc.length(&map))
        })
    });

    group.finish();
}

// ---- Delta sync: crdt-kit only (unique feature) ----

fn bench_delta_sync(c: &mut Criterion) {
    let mut group = c.benchmark_group("delta_sync");

    group.bench_function("GCounter delta+apply", |b| {
        use crdt_kit::prelude::*;
        let mut c1 = GCounter::new(1);
        c1.increment_by(1000);
        let mut c2 = GCounter::new(2);
        c2.increment_by(500);
        c2.merge(&c1);
        c1.increment_by(100);

        b.iter(|| {
            let delta = c1.delta(&c2);
            let mut target = c2.clone();
            target.apply_delta(&delta);
            black_box(target.value())
        })
    });

    group.bench_function("TextCrdt delta+apply (500 base + 100 new)", |b| {
        use crdt_kit::prelude::*;
        let mut t1 = TextCrdt::new(1);
        t1.insert_str(0, &"a".repeat(500)).unwrap();
        let mut t2 = t1.fork(2);
        t2.insert_str(500, &"b".repeat(100)).unwrap();

        b.iter(|| {
            let delta = t2.delta(&t1);
            let mut target = t1.clone();
            target.apply_delta(&delta);
            black_box(target.len())
        })
    });

    group.finish();
}

criterion_group!(
    comparative,
    bench_counter_increment,
    bench_text_insert,
    bench_text_merge,
    bench_list_insert,
    bench_set_insert,
    bench_delta_sync,
);
criterion_main!(comparative);
