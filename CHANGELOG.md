# Changelog

- **Note**: Updating from `v0.2.0` may require code changes.


All noteable changes made to the **public API** will be listed here.


## Version: [v0.3.0] (unreleased)

This version focuses on improving multi-platform support, specifically focusing on including windows in the API.

### Added

- (os) - adds windows support (`win`) for `is_executable` using file path and extension to determine if file is executable (is `.exe`).
- (tests) - adds windows `is_executable` test and test vectors to integration tests.

### Removed

- (CI/CD) removes `windows` from list of allowed failures.

## Version: [v0.3.0-alpha.1] - (release) - 2021-09-27

This version focuses on improving compilation on non-supported platforms (specifically Windows).

### Added

- (extensions) adds `unix` platform-specific attribute to imports.
- (CI/CD) adds Windows `os` to Travis CI config.

## Version: [v0.3.0-alpha.0] - (released) - 2021-09-27

This version focuses on bringing a consistent API to Rust developers implementing *some* of the guidelines [found here](https://rust-lang.github.io/api-guidelines/about.html). `v0.3.0-alpha.0` also adds integration testing of the Public API to prevent future updates breaking backward compatibility.

### Added

- (crypto) `Digest` exposes a new interface for working with SHA* algorithms, replacing `Sha2` and `Sha5` (now legacy), implementing `Write` from `std::io`.
- (crypto) `digest` now supports `SHA-384` algorithm using new `Digest` interface (not supported on legacy)

### Changed

- (crypto) `Sha2` and `Sha5` moved to legacy, public exports from `digest` will be removed in `v1.0.0`.
- (crypto/extensions) `FileExtended` now uses new `Digest` interface (no signature change).
- (extensions) adds `unix` OS-specific attribute to `is_executable()`.


## Version: [v0.2.0] - (released) - 2021-09-26

Version `v0.2.0` focuses on bringing support for calculating digests using `SHA-256` and `SHA-512` algorithms.


### Added

1. (extensions) `FileExtended` - added `digest(algorithm)` to calculate `sha256` and `sha512` digests.
2. (crypto) `Sha2` and `Sha5` implementations of `DigestExt` for calculating `sha256` and `sha512` from `Vec<u8>`.

## Version: [v0.1.0] - (released) - 2021-09-25

### Added

1. `MetadataExtended` - added `.is_executable()`.

[v0.3.0]: https://docs.rs/fsextra/0.3.0/fsextra/
[v0.3.0-alpha.1]: https://docs.rs/fsextra/0.3.0-alpha.0/fsextra/
[v0.3.0-alpha.0]: https://docs.rs/fsextra/0.3.0-alpha.0/fsextra/
[v0.2.0]: https://docs.rs/fsextra/0.2.0/fsextra/
[v0.1.0]: https://docs.rs/fsextra/0.1.0/fsextra/