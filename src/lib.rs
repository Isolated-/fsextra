/// A collection of extensions for filesystem operations
///
/// ## Compatibility
///
/// - Version `v0.3.0` is tested on Unix-based operating systems.
/// - Version `v0.2.0` is untested to work on any other system (although Rust may not complain).
pub mod extensions;

/// a collection of generic types/bounds
pub mod generics;

#[cfg(feature = "crypto")]
/// A collection of cryptographic operations (enabled by `crypto` feature)
///
/// ## Dependencies
///
/// - This module is a wrapper around [`ring@0.16.20`](https://docs.rs/ring/0.16.20/ring/).
pub mod crypto;

/// A collection of platform-support implementations and utilities
pub mod os;
