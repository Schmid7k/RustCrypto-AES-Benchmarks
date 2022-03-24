use aes::cipher::{BlockEncryptMut, KeyIvInit};
use aes::{Aes128, Aes192, Aes256};
use benches::KB;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use rust_crypto_aes_benchmarks as benches;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes192CbcEnc = cbc::Encryptor<Aes192>;
type Aes256CbcEnc = cbc::Encryptor<Aes256>;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("aes-cbc");

    let mut cipher128 = Aes128CbcEnc::new(&Default::default(), &Default::default());
    let mut cipher192 = Aes192CbcEnc::new(&Default::default(), &Default::default());
    let mut cipher256 = Aes256CbcEnc::new(&Default::default(), &Default::default());
    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![Default::default(); *size / 16];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher128.encrypt_blocks_mut(&mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-192", size), |b| {
            b.iter(|| cipher192.encrypt_blocks_mut(&mut buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-256", size), |b| {
            b.iter(|| cipher256.encrypt_blocks_mut(&mut buf));
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
