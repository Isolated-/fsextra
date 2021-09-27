/// legacy support for old functionality (tested + supported)
pub mod legacy;

/// export for backward compatibility with v0.2.0 (will be removed in v1.0.0)
pub use legacy::Sha2;

/// export for backward compatibility with v0.2.0 (will be removed in v1.0.0)
pub use legacy::Sha5;

use ring::digest::{
    Context, SHA256, SHA256_OUTPUT_LEN, SHA384, SHA384_OUTPUT_LEN, SHA512, SHA512_OUTPUT_LEN,
};
use std::io::{Result, Write};

pub const OUTPUT_LEN_SHA256: usize = SHA256_OUTPUT_LEN;
pub const OUTPUT_LEN_SHA384: usize = SHA384_OUTPUT_LEN;
pub const OUTPUT_LEN_SHA512: usize = SHA512_OUTPUT_LEN;

/// Supports `sha256`, `sha384` and `sha512` algorithms (`v0.3.0`)
///
/// ## Compatibility
///
/// `Sha3` is **only** supported using the new `Digest` interface.
pub enum DigestAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    /// short version of Sha256
    Sha2,
    /// short version of Sha384
    Sha3,
    /// short version of Sha512
    Sha5,
}

pub const SHA_256: DigestAlgorithm = DigestAlgorithm::Sha256;
pub const SHA_384: DigestAlgorithm = DigestAlgorithm::Sha384;
pub const SHA_512: DigestAlgorithm = DigestAlgorithm::Sha512;

pub trait Algorithm {}

impl Algorithm for DigestAlgorithm {}

/// *Added in **v0.3.0*** - new interface for calculating digests using SHA* algorithms
pub struct Digest {
    ctx: Context,
}

impl Digest {
    /// *Added in **v0.3.0*** - create a new Digest instance
    ///
    /// Uses pattern matching to determine which `ring` Context to use.
    ///
    /// `Sha2`, `Sha3` and `Sha5` are used as short versions of `Sha256`, `Sha384` and `Sha512` respectively.
    ///
    /// ## Arguments
    ///
    /// - `algo` - DigestAlgorithm value `DigestAlgorithm::Sha256`
    ///
    /// ## Returns
    ///
    /// - `Self` - a new Digest instance
    ///
    /// ## Errors
    ///
    /// - None
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fsextra::crypto::digest::{Digest, DigestAlgorithm};
    /// use std::io::{Write, Result};
    ///
    /// fn main() -> Result<()> {
    ///     let algo = DigestAlgorithm::Sha256;
    ///     let mut digest = Digest::new(algo);
    ///     
    ///     digest.write(b"")?;
    ///
    ///     assert_eq!(hex::encode(digest.finish()), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new(algo: DigestAlgorithm) -> Self {
        let ctx: Context;

        match algo {
            DigestAlgorithm::Sha256 | DigestAlgorithm::Sha2 => ctx = Context::new(&SHA256),
            DigestAlgorithm::Sha384 | DigestAlgorithm::Sha3 => ctx = Context::new(&SHA384),
            DigestAlgorithm::Sha512 | DigestAlgorithm::Sha5 => ctx = Context::new(&SHA512),
        }

        Digest { ctx }
    }

    /// *Added in **v0.3.0*** - completes the digest operation returning digest
    ///
    /// ## Arguments
    ///
    /// - None
    ///
    /// ## Returns
    ///
    /// - `Vec<u8>` - A vector containing bytes
    ///
    /// ## Errors
    ///
    /// - None
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fsextra::crypto::digest::{Digest, DigestAlgorithm};
    /// use std::io::{Write, Result};
    ///
    /// fn main() -> Result<()> {
    ///     // use SHA384
    ///     let algo = DigestAlgorithm::Sha384;
    ///
    ///     let mut digest = Digest::new(algo);
    ///     
    ///     // write some bytes to update digest    
    ///     digest.write(b"hello world!")?;
    ///     
    ///     // finalise and obtain digest
    ///     let finalised = digest.finish();
    ///
    ///     assert_eq!(hex::encode(finalised), "d33d40f7010ce34aa86efd353630309ed5c3d7ffac66d988825cf699f4803ccdf3f033230612f0945332fb580d8af805");
    ///     Ok(())
    /// }
    /// ```
    pub fn finish(self) -> Vec<u8> {
        self.ctx.finish().as_ref().to_vec()
    }
}

impl Write for Digest {
    /// *Added in **v0.3.0*** - write data to digest
    ///
    /// Can be reused until `finish()` is called.
    ///
    /// ## Arguments
    ///
    /// - `data` - slice of bytes to digest
    ///
    /// ## Returns
    ///
    /// - `Result<usize>` - A result containing the size of data written
    ///
    /// ## Errors
    ///
    /// - None
    ///
    /// ## Example
    ///
    /// ```rust
    /// use fsextra::crypto::digest::{Digest, DigestAlgorithm};
    /// use std::io::{Write, Result};
    ///
    /// fn main() -> Result<()> {
    ///     // use SHA384
    ///     let algo = DigestAlgorithm::Sha384;
    ///
    ///     let mut digest = Digest::new(algo);
    ///     
    ///     // write some bytes to update digest    
    ///     digest.write(b"hello world!")?;
    ///     
    ///     // finalise and obtain digest
    ///     let finalised = digest.finish();
    ///
    ///     assert_eq!(hex::encode(finalised), "d33d40f7010ce34aa86efd353630309ed5c3d7ffac66d988825cf699f4803ccdf3f033230612f0945332fb580d8af805");
    ///     Ok(())
    /// }
    /// ```
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.ctx.update(data);
        Ok(data.len())
    }

    /// not implemented
    fn flush(&mut self) -> Result<()> {
        unimplemented!();
    }

    /// not implemented
    fn write_all(&mut self, _: &[u8]) -> Result<()> {
        unimplemented!();
    }
}
