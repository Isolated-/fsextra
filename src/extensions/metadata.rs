use std::fs::Metadata;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub const EXECUTABLE: u32 = 0o111;

/// `MetadataExtended` provides extended behaviour for `std::fs::Metadata`
pub trait MetadataExtended {
    /// *Added in **v0.1.0*** - Determines if file is executable using `mode()` and bitwise operator.
    ///
    /// ## Compatibility
    ///
    /// - This method is only available for Unix-based files systems.
    ///
    /// ## Arguments
    ///
    /// - None
    ///
    /// ## Returns
    ///
    /// - `bool` - returns false if file = directory.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use std::fs::File;
    /// use std::error::Error;
    /// use fsextra::extensions::metadata::MetadataExtended;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let file = File::open("test_data/executable")?;
    ///     let metadata = file.metadata()?;
    ///
    ///     assert_eq!(metadata.is_executable(), true);
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(unix)]
    fn is_executable(&self) -> bool;
}

impl MetadataExtended for Metadata {
    #[cfg(unix)]
    fn is_executable(&self) -> bool {
        self.is_file() && self.mode() & EXECUTABLE != 0
    }
}
