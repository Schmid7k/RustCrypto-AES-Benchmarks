use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{AeadInPlace, Aes128Gcm, Aes256Gcm};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use rust_crypto_aes_benchmarks::KB;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("aes-gcm");

    let cipher128 = Aes128Gcm::new(&Default::default());
    let cipher256 = Aes256Gcm::new(&Default::default());
    let nonce = &Default::default();

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher128.encrypt_in_place_detached(nonce, b"", &mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-256", size), |b| {
            b.iter(|| cipher256.encrypt_in_place_detached(nonce, b"", &mut buf));
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

/*
use aes_gcm::aead::heapless::Vec;
use aes_gcm::aead::{AeadInPlace, NewAead};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use rust_crypto_aes_benchmarks::KB;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("aes-gcm");

    let cipher128 = Aes128Gcm::new(&Default::default());
    let cipher256 = Aes256Gcm::new(&Default::default());
    let nonce = &Default::default();

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf: Vec<u8, 1040> = Vec::new();

        let ext = vec![0u8; *size];

        buf.extend(ext.into_iter());

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher128.encrypt_in_place(&nonce, b"", &mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-256", size), |b| {
            b.iter(|| cipher256.encrypt_in_place(&nonce, b"", &mut buf));
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

*/
