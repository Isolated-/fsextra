mod common;

#[cfg(windows)]
use fsextra::os::win::is_executable;

#[cfg(windows)]
#[test]
fn is_executable_returns_expected_output_from_inputs() {
    let tests = common::consume_os_win_exec_tests();

    for test in tests {
        assert_eq!(is_executable(&test.path), test.expected);
    }
}
