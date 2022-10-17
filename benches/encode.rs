use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_base62::encode;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("[0; 17]", |b| b.iter(|| encode(black_box(&[0; 17]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
