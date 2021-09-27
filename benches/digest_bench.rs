#[allow(unused_imports)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(feature = "crypto")]
use fsextra::crypto::digest::{Digest, DigestAlgorithm};

#[cfg(feature = "crypto")]
use fsextra::extensions::FileExtended;

#[allow(unused_imports)]
use std::io::{Result, Write};

#[allow(unused_imports)]
use std::fs::File;

fn criterion_benchmark(c: &mut Criterion) {
    #[cfg(feature = "crypto")]
    c.bench_function("digest (sha256) - plaintext", |b| {
        b.iter(|| {
            let mut digest = Digest::new(DigestAlgorithm::Sha256);
            let plaintext = black_box("hello world!");

            digest.write(plaintext.as_bytes()).unwrap();
            digest.finish();
        })
    });

    #[cfg(feature = "crypto")]
    c.bench_function("digest (sha256) - FileExtended", |b| {
        b.iter(|| {
            let mut file = File::open("test_data/files/test_01.txt").unwrap();

            let _ = file.digest(DigestAlgorithm::Sha256).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
