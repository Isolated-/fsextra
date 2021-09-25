use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

pub trait MetadataExtended {
    fn is_executable(&self) -> bool;
}

impl MetadataExtended for Metadata {
    /// *Added in **v0.1.0***
    ///
    /// Determines if file is executable using `mode()` and bitwise operator.
    ///
    /// ## Arguments
    ///
    /// - None
    ///
    /// ## Returns
    ///
    /// - `bool`
    fn is_executable(&self) -> bool {
        return self.is_file() && self.mode() & 0o111 != 0;
    }
}
