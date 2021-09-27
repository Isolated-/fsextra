mod common;

#[cfg(windows)]
#[cfg(feature = "os")]
use fsextra::os::win::is_executable;

#[cfg(windows)]
#[cfg(feature = "os")]
#[test]
fn is_executable_returns_expected_output_from_inputs() {
    let tests = common::consume_os_win_exec_tests();

    for test in tests {
        assert_eq!(is_executable(&test.path), test.expected);
    }
}
