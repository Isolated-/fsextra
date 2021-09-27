mod common;

#[cfg(feature = "crypto")]
use fsextra::crypto::digest::legacy::{DigestExt, Sha2, Sha5};

#[test]
#[cfg(feature = "crypto")]
fn legacy_digest_returns_expected_outputs_from_inputs() {
    let tests = common::consume_digest_tests();

    let mut digest: Vec<u8>;

    for test in tests {
        let algorithm: &str = &test.algorithm;
        match algorithm {
            "SHA256" => {
                let sha2 = Sha2::new(test.plaintext.as_bytes());
                digest = sha2.digest().unwrap();
            }
            "SHA512" => {
                let sha5 = Sha5::new(test.plaintext.as_bytes());
                digest = sha5.digest().unwrap();
            }
            "SHA384" => {
                // purposely excluded
                continue;
            }
            _ => {
                continue;
            }
        }

        assert_eq!(hex::encode(digest), test.digest);
    }
}
