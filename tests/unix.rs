mod common;

use fsextra::extensions::MetadataExtended;
use std::fs::File;

#[cfg(unix)]
#[test]
fn is_executable_returns_expected_output_from_inputs() {
    let tests = common::consume_unix_exec_tests();

    for test in tests {
        let file = File::open(&test.path).unwrap();
        let metadata = file.metadata().unwrap();

        assert_eq!(metadata.is_executable(), test.expected);
    }
}
