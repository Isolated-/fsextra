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
/// support for cryptographic operations (requires `crypto` features)
pub mod crypto;
