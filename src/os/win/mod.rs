use std::path::Path;

#[cfg(windows)]
/// *Added in **v0.3.0*** - work around to implement is_executable() on windows
///
/// Note: uses file extension to determine if file is executable (.exe).
///
/// ## Arguments
///
/// - `path` - path to file (doesn't need to exist) - C:\Users\You\executable.exe
///
/// ## Returns
///
/// - `bool` - true if path extension is Some and is "exe"
///
/// ## Usage
///
/// ```rust
/// use fsextra::os::win::{is_executable};
///
/// fn main() {
///     assert_eq!(is_executable("text.txt", false));
///     assert_eq!(is_executable("program.exe", true));
/// }
///
/// ```
pub fn is_executable(path: &str) -> bool {
    match Path::new(path).extension() {
        Some(ext) => ext == "exe",
        None => false,
    }
}
