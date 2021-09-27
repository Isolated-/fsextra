mod common;

use fsextra::extensions::MetadataExtended;
use std::fs::File;
use std::io::ErrorKind;

#[cfg(unix)]
#[test]
/// this also tests directories (previously in metadata_tests, now removed)
fn is_executable_returns_expected_output_from_inputs() {
    let tests = common::consume_unix_exec_tests();

    for test in tests {
        match File::open(&test.path) {
            Ok(f) => {
                if let Ok(metadata) = f.metadata() {
                    assert_eq!(metadata.is_executable(), test.expected);
                } else {
                    unreachable!();
                }
            }
            Err(e) => {
                // test if a file doesn't exist
                assert_eq!(e.kind(), ErrorKind::NotFound);
            }
        }
    }
}

#[test]
#[cfg(unix)]
fn metadata_is_executable_does_not_change_file() {
    let tests = common::consume_unix_exec_tests();

    for test in tests {
        let file = File::open(&test.path);

        if file.is_err() {
            continue;
        }

        let file = file.unwrap();
        let metadata = file.metadata().unwrap();

        assert_eq!(metadata.is_executable(), test.expected);

        let comparison = File::open(&test.path).unwrap().metadata().unwrap();

        assert_eq!(metadata.len(), comparison.len());
        assert_eq!(metadata.modified().unwrap(), comparison.modified().unwrap());
        assert_eq!(metadata.accessed().unwrap(), comparison.accessed().unwrap());
    }
}
