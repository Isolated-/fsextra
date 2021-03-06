pub use std::fs::File;
pub use std::io::{Read, Result, Write};

#[cfg(feature = "crypto")]
pub use crate::crypto::digest::{Digest, DigestAlgorithm};

/// `FileExtended` provides extended behaviour for `std::fs::File`
pub trait FileExtended {
    #[cfg(feature = "crypto")]
    /// *Added in **v0.2.0*** - calculates digest for file using either `Sha2` or `Sha5` - requires `crypto` feature
    ///
    /// ## Arguments
    ///
    /// - `algorithm` - `DigestAlgorithm` value (`Sha2`/`Sha5`)
    ///
    /// ## Returns
    ///
    /// - `Result<Vec<u8>>` - a vector of bytes containing `sha*` digest
    ///
    /// ## Errors
    ///
    /// - `Read` errors are pushed up the chain.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fsextra::extensions::file::*;
    ///
    /// fn main() -> Result<()> {
    ///     let mut file = File::open("test_data/test_01.txt")?;
    ///     let digest = file.digest(DigestAlgorithm::Sha256)?;
    ///     
    ///     assert_eq!(hex::encode(digest), "7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9");
    ///     
    ///     Ok(())
    /// }
    /// ```
    fn digest(&mut self, algorithm: DigestAlgorithm) -> Result<Vec<u8>>;
}

impl FileExtended for File {
    #[cfg(feature = "crypto")]
    fn digest(&mut self, algorithm: DigestAlgorithm) -> Result<Vec<u8>> {
        let len = self.metadata()?.len();

        let mut bytes = vec![0; len as usize];
        self.read_exact(&mut bytes)?;

        let mut digest = Digest::new(algorithm);
        digest.write(&bytes)?;

        Ok(digest.finish())
    }
}
