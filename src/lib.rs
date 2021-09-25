/// a collection of extensions for filesystem operations
pub mod extensions;

/// a collection of generic types/bounds
pub mod generics;

#[cfg(feature = "crypto")]
/// support for cryptographic operations (requires `crypto` features)
pub mod crypto;
