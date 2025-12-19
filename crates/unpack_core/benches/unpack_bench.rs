use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustc_hash::FxHashMap;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn hashmap_operations(size: usize) {
    let mut map = FxHashMap::default();
    for i in 0..size {
        map.insert(i, i * 2);
    }
    for i in 0..size {
        let _ = map.get(&i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| b.iter(|| fibonacci(black_box(20))));

    c.bench_function("hashmap operations 1000", |b| {
        b.iter(|| hashmap_operations(black_box(1000)))
    });

    c.bench_function("hashmap operations 10000", |b| {
        b.iter(|| hashmap_operations(black_box(10000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
