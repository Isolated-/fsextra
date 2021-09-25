use crate::generics::bound_by::BoundBy;
use std::io::Result;

impl BoundBy for Vec<u8> {}

/// only supports `sha256` and `sha512` (`v0.2.0`)
pub enum DigestAlgorithm {
    Sha2,
    Sha5,
}

/// digest trait for supporting different algorithms.
pub trait DigestExt<B: BoundBy>
where
    B: BoundBy,
{
    /// *Added in **v0.2.0*** - create a new `DigestExt` instance - requires `crypto` feature
    ///
    /// ## Arguments
    ///
    /// - `bytes`- byte slice containing bytes of data to digest
    ///
    /// ## Returns
    ///
    /// - `Self` - a `DigestExt` instance.
    fn new(bytes: &[u8]) -> Self;

    /// *Added in **v0.2.0*** - calculates digest using `sha256`/`sha512` - requires `crypto`
    ///
    /// ## Arguments
    ///
    /// - None
    ///
    /// ## Returns
    ///
    /// - `Result<Vec<u8>>` - a vector of bytes containing `sha*` digest
    ///
    /// ## Errors
    ///
    /// - Errors are pushed up chain.
    fn digest(&self) -> Result<Vec<u8>>;
}

/// *Added in **v0.2.0*** - provides support for calculating `sha256` digests - requires `crypto` feature
pub mod sha2;
pub use sha2::Sha2;

/// *Added in **v0.2.0*** - provides support for calculating `sha512` digests - requires `crypto` feature
pub mod sha5;
pub use sha5::Sha5;
