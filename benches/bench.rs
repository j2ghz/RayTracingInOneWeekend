use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn power_impls(c: &mut Criterion) {
    let mut group = c.benchmark_group("power_impls");
    for i in [1, 2, 3, 4, 8, 32, 128, 46340].iter() {
        let f = *i as f64;
        group.bench_with_input(BenchmarkId::new("powi", i), &(f, i), |b, (f, i)| {
            let i = **i;
            b.iter(|| black_box(f.powi(i)))
        });
        group.bench_with_input(BenchmarkId::new("powf", i), &(f, i), |b, (f, _i)| {
            let f = *f;
            b.iter(|| black_box(f.powf(f)))
        });
        group.bench_with_input(BenchmarkId::new("mul", i), &(f, i), |b, (f, _i)| {
            b.iter(|| black_box(f * f))
        });
    }
}

criterion_group!(benches, power_impls);
criterion_main!(benches);
