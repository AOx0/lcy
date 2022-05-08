use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
/* use lcy::test;

/// Test with single value
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Test", |b| b.iter(|| test(black_box(0.0))));
}

/// Test with multiple values
pub fn criterion_benchmark2(c: &mut Criterion) {
    let mut group = c.benchmark_group("related");
    for time in [0.1, 0.5, 1.0, 1.5].iter() {
        // group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(time), time, |b, &time| {
            b.iter(|| test(black_box(time)));
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark2);
criterion_main!(benches);
*/
