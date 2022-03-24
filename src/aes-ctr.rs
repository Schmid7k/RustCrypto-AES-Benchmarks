use aes::cipher::{KeyIvInit, StreamCipher};
use aes::{Aes128, Aes192, Aes256};
use benches::KB;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use rust_crypto_aes_benchmarks as benches;

type Aes128Ctr128LE = ctr::Ctr128LE<Aes128>;
type Aes192Ctr128LE = ctr::Ctr128LE<Aes192>;
type Aes256Ctr128LE = ctr::Ctr128LE<Aes256>;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("aes-ctr");

    let mut cipher128_128 = Aes128Ctr128LE::new(&Default::default(), &Default::default());
    let mut cipher192_128 = Aes192Ctr128LE::new(&Default::default(), &Default::default());
    let mut cipher256_128 = Aes256Ctr128LE::new(&Default::default(), &Default::default());

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![Default::default(); *size];

        group.throughput(Throughput::Bytes((*size) as u64));

        group.bench_function(BenchmarkId::new("encrypt-128-128LE", size), |b| {
            b.iter(|| cipher128_128.apply_keystream(&mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-192-128LE", size), |b| {
            b.iter(|| cipher192_128.apply_keystream(&mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-256-128LE", size), |b| {
            b.iter(|| cipher256_128.apply_keystream(&mut buf));
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);

criterion_main!(benches);
