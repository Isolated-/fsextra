#[cfg(feature = "crypto")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "crypto")]
use std::io::BufReader;

#[cfg(feature = "crypto")]
#[derive(Serialize, Deserialize)]
pub struct DigestTestFile {
    pub plaintext: String,
    pub digest: String,
    pub algorithm: String,
}

#[cfg(feature = "crypto")]
pub fn consume_digest_tests() -> Vec<DigestTestFile> {
    let file = std::fs::File::open("test_data/digest_tests.json").unwrap();
    let mut reader = BufReader::new(file);
    return serde_json::from_reader(&mut reader).unwrap();
}
