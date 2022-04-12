use aes::cipher::{BlockEncrypt, KeyInit};
use aes::{Aes128Enc, Aes192Enc, Aes256Enc};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use rust_crypto_aes_benchmarks::KB;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("aes-ecb");

    let cipher128 = Aes128Enc::new(&Default::default());
    let cipher192 = Aes192Enc::new(&Default::default());
    let cipher256 = Aes256Enc::new(&Default::default());

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![Default::default(); *size / 16];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher128.encrypt_blocks(&mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-192", size), |b| {
            b.iter(|| cipher192.encrypt_blocks(&mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-256", size), |b| {
            b.iter(|| cipher256.encrypt_blocks(&mut buf));
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
