# Changelog

All noteable changes made to the **public API** will be listed here.

## Version: [v0.2.0] - (released) - 2021-09-26

Version `v0.2.0` focuses on bringing support for calculating digests using `SHA-256` and `SHA-512` algorithms.


### Added

1. (extensions) `FileExtended` - added `digest(algorithm)` to calculate `sha256` and `sha512` digests.
2. (crypto) `Sha2` and `Sha5` implementations of `DigestExt` for calculating `sha256` and `sha512` from `Vec<u8>`.

## Version: [v0.1.0] - (released) - 2021-09-25

### Added

1. `MetadataExtended` - added `.is_executable()`.

[v0.1.0]: https://docs.rs/fsextra/0.1.0/fsextra/
[v0.2.0]: https://docs.rs/fsextra/0.2.0/fsextra/