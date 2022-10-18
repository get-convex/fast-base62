use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    let test_cases = vec![
        vec![],
        vec![0; 17],
        vec![0; 128],
        vec![0; 512],
        vec![0; 1024],
    ];
    for i in &test_cases {
        group.bench_with_input(BenchmarkId::new("fast_base62", i.len()), i, |b, i| {
            b.iter(|| fast_base62::encode(black_box(&i)))
        });
        group.bench_with_input(BenchmarkId::new("base62", i.len()), i, |b, i| {
            b.iter(|| base_62::encode(black_box(&i)))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
