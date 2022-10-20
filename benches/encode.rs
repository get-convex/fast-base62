use criterion::{
    black_box, criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

pub fn random_bytes<R: Rng>(rng: &mut R, n: usize) -> Vec<u8> {
    let mut out = vec![0; n];
    rng.fill(&mut out[..]);
    out
}

// Based on benchmarks in https://github.com/Nugine/simd
pub fn bench_encode(c: &mut Criterion) {
    let mut rng = StdRng::from_seed([0; 32]);

    let mut group = c.benchmark_group("encode");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let lengths = [8, 16, 32, 64, 128, 256, 1024];
    let inputs: Vec<_> = lengths.iter().map(|&i| random_bytes(&mut rng, i)).collect();

    let functions: &[(&str, fn(&[u8]))] = &[
        ("fast_base62", |src| {
            let _ = fast_base62::encode(src);
        }),
        ("base_62", |src| {
            let _ = base_62::encode(src);
        }),
    ];
    for &(name, f) in functions {
        for src in &inputs {
            group.throughput(Throughput::Bytes(src.len() as u64));
            let id = BenchmarkId::new(name, src.len());
            group.bench_with_input(id, &src[..], |b, src| b.iter(|| f(black_box(src))));
        }
    }
    group.finish();
}

pub fn bench_decode(c: &mut Criterion) {
    let mut rng = StdRng::from_seed([0; 32]);

    let mut group = c.benchmark_group("decode");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    let lengths = [8, 16, 32, 64, 128, 256, 1024];
    let inputs: Vec<_> = lengths.iter().map(|&i| random_bytes(&mut rng, i)).collect();

    let functions: &[(&str, fn(&[u8]) -> String, fn(&str))] = &[
        ("fast_base62", fast_base62::encode, |src| {
            let _ = fast_base62::decode(src).unwrap();
        }),
        ("base_62", base_62::encode, |src| {
            let _ = base_62::decode(src).unwrap();
        }),
    ];
    for &(name, encode, decode) in functions {
        for src in &inputs {
            let encoded = encode(&src);
            group.throughput(Throughput::Bytes(encoded.len() as u64));
            let id = BenchmarkId::new(name, encoded.len());
            group.bench_with_input(id, &encoded, |b, encoded| {
                b.iter(|| decode(black_box(encoded)))
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
