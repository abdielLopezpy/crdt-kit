use crdt_kit::clock::HybridTimestamp;
use crdt_kit::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_gcounter_increment(c: &mut Criterion) {
    c.bench_function("GCounter::increment x1000", |b| {
        b.iter(|| {
            let mut counter = GCounter::new(1);
            for _ in 0..1000 {
                counter.increment();
            }
            black_box(counter.value())
        })
    });
}

fn bench_gcounter_merge(c: &mut Criterion) {
    let counters: Vec<GCounter> = (0..10)
        .map(|i| {
            let mut c = GCounter::new(i);
            c.increment_by(100);
            c
        })
        .collect();

    c.bench_function("GCounter::merge 10 replicas", |b| {
        b.iter(|| {
            let mut merged = counters[0].clone();
            for other in &counters[1..] {
                merged.merge(other);
            }
            black_box(merged.value())
        })
    });

    let many_counters: Vec<GCounter> = (0..100)
        .map(|i| {
            let mut c = GCounter::new(i);
            c.increment_by(100);
            c
        })
        .collect();

    c.bench_function("GCounter::merge 100 replicas", |b| {
        b.iter(|| {
            let mut merged = many_counters[0].clone();
            for other in &many_counters[1..] {
                merged.merge(other);
            }
            black_box(merged.value())
        })
    });
}

fn bench_pncounter(c: &mut Criterion) {
    c.bench_function("PNCounter::inc+dec x1000", |b| {
        b.iter(|| {
            let mut counter = PNCounter::new(1);
            for _ in 0..500 {
                counter.increment();
                counter.decrement();
            }
            black_box(counter.value())
        })
    });
}

fn bench_orset_insert(c: &mut Criterion) {
    c.bench_function("ORSet::insert x1000", |b| {
        b.iter(|| {
            let mut set = ORSet::new(1);
            for i in 0..1000u32 {
                set.insert(i);
            }
            black_box(set.len())
        })
    });
}

fn bench_orset_merge(c: &mut Criterion) {
    let mut s1 = ORSet::new(1);
    let mut s2 = ORSet::new(2);

    for i in 0..500u32 {
        s1.insert(i);
        s2.insert(i + 250);
    }

    c.bench_function("ORSet::merge 500+500 elements", |b| {
        b.iter(|| {
            let mut merged = s1.clone();
            merged.merge(&s2);
            black_box(merged.len())
        })
    });
}

fn bench_gset_merge(c: &mut Criterion) {
    let mut s1 = GSet::new();
    let mut s2 = GSet::new();

    for i in 0..1000u32 {
        s1.insert(i);
        s2.insert(i + 500);
    }

    c.bench_function("GSet::merge 1000+1000 elements", |b| {
        b.iter(|| {
            let mut merged = s1.clone();
            merged.merge(&s2);
            black_box(merged.len())
        })
    });
}

fn bench_lww_register_merge(c: &mut Criterion) {
    let registers: Vec<LWWRegister<String>> = (0..100)
        .map(|i| {
            let ts = HybridTimestamp {
                physical: i as u64,
                logical: 0,
                node_id: i as u16,
            };
            LWWRegister::with_timestamp(format!("value-{i}"), ts)
        })
        .collect();

    c.bench_function("LWWRegister::merge 100 replicas", |b| {
        b.iter(|| {
            let mut merged = registers[0].clone();
            for other in &registers[1..] {
                merged.merge(other);
            }
            black_box(merged.value().clone())
        })
    });
}

fn bench_rga_insert(c: &mut Criterion) {
    c.bench_function("Rga::insert_at x1000 sequential", |b| {
        b.iter(|| {
            let mut rga = Rga::new(1);
            for i in 0..1000u32 {
                rga.insert_at(i as usize, i).unwrap();
            }
            black_box(rga.len())
        })
    });
}

fn bench_rga_remove(c: &mut Criterion) {
    let mut rga = Rga::new(1);
    for i in 0..1000u32 {
        rga.insert_at(i as usize, i).unwrap();
    }

    c.bench_function("Rga::remove x500", |b| {
        b.iter(|| {
            let mut r = rga.clone();
            for _ in 0..500 {
                let _ = r.remove(0);
            }
            black_box(r.len())
        })
    });
}

fn bench_rga_merge(c: &mut Criterion) {
    let mut r1 = Rga::new(1);
    let mut r2 = Rga::new(2);
    for i in 0..500u32 {
        r1.insert_at(i as usize, i).unwrap();
        r2.insert_at(i as usize, i + 1000).unwrap();
    }

    c.bench_function("Rga::merge 500+500 elements", |b| {
        b.iter(|| {
            let mut merged = r1.clone();
            merged.merge(&r2);
            black_box(merged.len())
        })
    });
}

fn bench_lww_map_insert(c: &mut Criterion) {
    c.bench_function("LWWMap::insert x1000", |b| {
        b.iter(|| {
            let mut map = LWWMap::new();
            for i in 0..1000u32 {
                let ts = HybridTimestamp {
                    physical: i as u64,
                    logical: 0,
                    node_id: 1,
                };
                map.insert(i, i * 2, ts);
            }
            black_box(map.len())
        })
    });
}

fn bench_lww_map_merge(c: &mut Criterion) {
    let mut m1 = LWWMap::new();
    let mut m2 = LWWMap::new();
    for i in 0..500u32 {
        let ts1 = HybridTimestamp { physical: i as u64, logical: 0, node_id: 1 };
        let ts2 = HybridTimestamp { physical: (i + 250) as u64, logical: 0, node_id: 2 };
        m1.insert(i, i, ts1);
        m2.insert(i + 250, i + 250, ts2);
    }

    c.bench_function("LWWMap::merge 500+500 entries", |b| {
        b.iter(|| {
            let mut merged = m1.clone();
            merged.merge(&m2);
            black_box(merged.len())
        })
    });
}

fn bench_aw_map_insert(c: &mut Criterion) {
    c.bench_function("AWMap::insert x1000", |b| {
        b.iter(|| {
            let mut map = AWMap::new(1);
            for i in 0..1000u32 {
                map.insert(i, i * 2);
            }
            black_box(map.len())
        })
    });
}

fn bench_aw_map_merge(c: &mut Criterion) {
    let mut m1 = AWMap::new(1);
    let mut m2 = AWMap::new(2);
    for i in 0..500u32 {
        m1.insert(i, i);
        m2.insert(i + 250, i + 250);
    }

    c.bench_function("AWMap::merge 500+500 entries", |b| {
        b.iter(|| {
            let mut merged = m1.clone();
            merged.merge(&m2);
            black_box(merged.len())
        })
    });
}

fn bench_text_insert(c: &mut Criterion) {
    c.bench_function("TextCrdt::insert_str 1000 chars", |b| {
        b.iter(|| {
            let mut t = TextCrdt::new(1);
            t.insert_str(0, &"a".repeat(1000)).unwrap();
            black_box(t.len())
        })
    });
}

fn bench_text_merge(c: &mut Criterion) {
    let mut t1 = TextCrdt::new(1);
    t1.insert_str(0, &"a".repeat(500)).unwrap();

    let mut t2 = TextCrdt::new(2);
    t2.insert_str(0, &"b".repeat(500)).unwrap();

    c.bench_function("TextCrdt::merge 500+500 chars", |b| {
        b.iter(|| {
            let mut merged = t1.clone();
            merged.merge(&t2);
            black_box(merged.len())
        })
    });
}

fn bench_text_merge_overlapping(c: &mut Criterion) {
    let mut t1 = TextCrdt::new(1);
    t1.insert_str(0, &"a".repeat(500)).unwrap();
    let mut t2 = t1.fork(2);
    t2.insert_str(500, &"b".repeat(100)).unwrap();

    c.bench_function("TextCrdt::merge 500+100 overlapping", |b| {
        b.iter(|| {
            let mut merged = t1.clone();
            merged.merge(&t2);
            black_box(merged.len())
        })
    });
}

fn bench_delta_sync(c: &mut Criterion) {
    let mut t1 = TextCrdt::new(1);
    t1.insert_str(0, &"a".repeat(500)).unwrap();
    let mut t2 = t1.fork(2);
    t2.insert_str(500, &"b".repeat(100)).unwrap();

    c.bench_function("TextCrdt::delta+apply 500+100", |b| {
        b.iter(|| {
            let delta = t2.delta(&t1);
            let mut target = t1.clone();
            target.apply_delta(&delta);
            black_box(target.len())
        })
    });
}

fn bench_memory_footprint(c: &mut Criterion) {
    use std::mem::size_of_val;

    c.bench_function("memory: GCounter 10 nodes", |b| {
        b.iter(|| {
            let mut gc = GCounter::new(0);
            for i in 0..10u64 {
                let mut other = GCounter::new(i);
                other.increment_by(100);
                gc.merge(&other);
            }
            black_box(size_of_val(&gc))
        })
    });

    c.bench_function("memory: ORSet 1000 elements", |b| {
        b.iter(|| {
            let mut set = ORSet::new(1);
            for i in 0..1000u32 {
                set.insert(i);
            }
            black_box(set.len())
        })
    });

    c.bench_function("memory: Rga 1000 elements", |b| {
        b.iter(|| {
            let mut rga = Rga::new(1);
            for i in 0..1000u32 {
                rga.insert_at(i as usize, i).unwrap();
            }
            black_box(rga.len())
        })
    });

    c.bench_function("memory: LWWMap 1000 entries", |b| {
        b.iter(|| {
            let mut map = LWWMap::new();
            for i in 0..1000u32 {
                let ts = HybridTimestamp {
                    physical: i as u64,
                    logical: 0,
                    node_id: 1,
                };
                map.insert(i, i * 2, ts);
            }
            black_box(map.len())
        })
    });

    c.bench_function("memory: AWMap 1000 entries", |b| {
        b.iter(|| {
            let mut map = AWMap::new(1);
            for i in 0..1000u32 {
                map.insert(i, i * 2);
            }
            black_box(map.len())
        })
    });

    c.bench_function("memory: TextCrdt 1000 chars", |b| {
        b.iter(|| {
            let mut t = TextCrdt::new(1);
            t.insert_str(0, &"x".repeat(1000)).unwrap();
            black_box(t.len())
        })
    });
}

criterion_group!(
    benches,
    bench_gcounter_increment,
    bench_gcounter_merge,
    bench_pncounter,
    bench_orset_insert,
    bench_orset_merge,
    bench_gset_merge,
    bench_lww_register_merge,
    bench_lww_map_insert,
    bench_lww_map_merge,
    bench_aw_map_insert,
    bench_aw_map_merge,
    bench_rga_insert,
    bench_rga_remove,
    bench_rga_merge,
    bench_text_insert,
    bench_text_merge,
    bench_text_merge_overlapping,
    bench_delta_sync,
    bench_memory_footprint,
);
criterion_main!(benches);
