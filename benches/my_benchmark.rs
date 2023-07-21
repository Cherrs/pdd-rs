use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    // let runtime = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("build_sign", |b| b.iter(|| {}));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
