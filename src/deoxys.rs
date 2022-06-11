use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;
use deoxys::aead::{Aead, NewAead};
use deoxys::{DeoxysI128, DeoxysI256, Key, Nonce};
use rand::rngs::OsRng;
use rand::RngCore;
use rust_crypto_aes_benchmarks::KB;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("deoxys");
    let mut rng = OsRng;

    let cipher128 = DeoxysI128::new(&Default::default());
    let cipher256 = DeoxysI256::new(&Default::default());
    let nonce = &Default::default();

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![0; *size / 16];
        rng.fill_bytes(&mut buf);

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("encrypt-128", size), |b| {
            b.iter(|| cipher128.encrypt(nonce, buf.as_ref()));
        });

        group.bench_function(BenchmarkId::new("encrypt-256", size), |b| {
            b.iter(|| cipher256.encrypt(nonce, buf.as_ref()));
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
