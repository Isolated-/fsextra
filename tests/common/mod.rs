#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::io::BufReader;

#[cfg(feature = "crypto")]
#[derive(Serialize, Deserialize)]
pub struct DigestTestFile {
    pub plaintext: String,
    pub digest: String,
    pub algorithm: String,
}

#[cfg(feature = "crypto")]
#[allow(dead_code)]
pub fn consume_digest_tests() -> Vec<DigestTestFile> {
    let file = std::fs::File::open("test_data/digest_tests.json").unwrap();
    let mut reader = BufReader::new(file);
    return serde_json::from_reader(&mut reader).unwrap();
}

#[cfg(windows)]
#[derive(Serialize, Deserialize)]
pub struct OsWinExecutableTestFile {
    pub path: String,
    pub expected: bool,
}

#[cfg(windows)]
#[allow(dead_code)]
pub fn consume_os_win_exec_tests() -> Vec<OsWinExecutableTestFile> {
    let file = std::fs::File::open("test_data/oswin_executable_tests.json").unwrap();
    let mut reader = BufReader::new(file);
    return serde_json::from_reader(&mut reader).unwrap();
}

#[cfg(unix)]
#[derive(Serialize, Deserialize)]
pub struct UnixExecutableTestFile {
    pub path: String,
    pub expected: bool,
}

#[cfg(unix)]
#[allow(dead_code)]
pub fn consume_unix_exec_tests() -> Vec<UnixExecutableTestFile> {
    let file = std::fs::File::open("test_data/unix_executable_tests.json").unwrap();
    let mut reader = BufReader::new(file);
    return serde_json::from_reader(&mut reader).unwrap();
}
