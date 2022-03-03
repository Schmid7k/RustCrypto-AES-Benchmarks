# RustCrypto-AES-Benchmarks

This repository holds benchmarks for RustCrypto's AES module and its different block modes. The benchmark criteria are _cycles-per-byte (cpb)_ and the performance is measured using [criterion.rs](https://github.com/bheisler/criterion.rs) with the `criterion-cycles-per-byte` plugin.

To execute the benchmarks on your own machine follow these steps:

1. Clone the repository `git clone https://github.com/Schmid7k/RustCrypto-AES-Benchmarks.git`
2. Change directory into the cloned repository
3. Execute `cargo bench` on the command line to run all benchmarks or execute `cargo bench aes-<Block mode>` e.g. `cargo bench aes-ecb` to only run benchmarks for some block mode.

While running the benchmarks will generate some insightful output on the command line. However, after the benchmarks are finished you can take a closer look at the statistics generated by `criterion` by going to `./target/criterion`.

---

Currently implemented benchmarks:

- `aes-ecb`
- `aes-cbc`
- `aes-ctr`
- `aes-gcm`
