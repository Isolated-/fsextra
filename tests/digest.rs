mod common;

#[cfg(feature = "crypto")]
use fsextra::crypto::digest::{Digest, DigestAlgorithm, DigestExt};

#[cfg(feature = "crypto")]
use std::io::Write;

#[test]
#[cfg(feature = "crypto")]
fn v1_digest_returns_expected_outputs_from_inputs() {
    let tests = common::consume_digest_tests();

    let mut algo: DigestAlgorithm;
    for test in tests {
        let algorithm: &str = &test.algorithm;

        match algorithm {
            "SHA256" => algo = DigestAlgorithm::Sha256,
            "SHA384" => algo = DigestAlgorithm::Sha384,
            "SHA512" => algo = DigestAlgorithm::Sha512,
            _ => {
                unreachable!();
            }
        }

        let mut digest = Digest::new(algo);
        digest.write(test.plaintext.as_bytes()).unwrap();
        let result = digest.finish();

        assert_eq!(hex::encode(result), test.digest);
    }
}
