/// Supports calculating digests using various algorithms (requires `crypto` feature)
///
/// This interface was improved and extended in `v0.3.0`.
///
/// ## Supported Algorithms
///
/// - `SHA-256`
/// - `SHA-384`
/// - `SHA-512`
///
/// ## Note
///
/// Version `v0.2.0` exposed a different interface, it is recommended to use `Digest`.
/// If you require the `v0.2.0` interface, you'll find it in `fsextra::crypto::digest::legacy`.
pub mod digest;
