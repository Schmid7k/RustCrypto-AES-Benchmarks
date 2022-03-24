use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use benches::KB;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use rust_crypto_aes_benchmarks as benches;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("aes-gcm");

    let cipher128 = Aes128Gcm::new(&Default::default());
    let cipher256 = Aes256Gcm::new(&Default::default());

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let buf = vec![Default::default(); *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher128.encrypt(&Default::default(), &*buf));
        });

        group.bench_function(BenchmarkId::new("encrypt-256", size), |b| {
            b.iter(|| cipher256.encrypt(&Default::default(), &*buf));
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
