use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iter_num_tools::lin_space;

fn bench(i: impl Iterator<Item = f64>) -> Vec<f64> {
    black_box(i.map(|x| x * 2.0).collect())
}

fn lin_space_std(start: f64, end: f64, steps: usize) -> impl Iterator<Item = f64> {
    let len = end - start;
    let step = len / steps as f64;
    (0..=steps).map(move |i| start + i as f64 * step)
}

pub fn bench_lin_space(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinSpace");

    group.bench_function("linspace [1.0, 100.0] x100 (iter-num-tools)", |b| {
        b.iter(|| bench(lin_space(1.0..=100.0, 100)))
    });

    group.bench_function("linspace [1.0, 100.0] x100 (std)", |b| {
        b.iter(|| bench(lin_space_std(1.0, 100.0, 100)))
    });

    group.bench_function("linspace [1.0, 100.0] x100 (itertools-num)", |b| {
        b.iter(|| bench(itertools_num::linspace(1.0, 100.0, 100)))
    });

    group.finish();
}

criterion_group!(benches, bench_lin_space);
criterion_main!(benches);
