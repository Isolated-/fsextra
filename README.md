# fsextra

**Note:** this library has not been externally tested for security and usage is at **your own risk**.

[![Build Status](https://app.travis-ci.com/Isolated-/fsextra.svg?branch=next)](https://app.travis-ci.com/Isolated-/fsextra)

> `fsextra` is a collection of extensions to simplify working with Unix-based filesystems. This library will also support cryptographic operations on files and directories by enabling the `crypto` feature (`> v0.2.0`).

- **Current Version**: `v0.3.0` ([Changelog])

## Installation

Install default features by updating your `Cargo.toml` file to include `fsextra = "*"`.

```toml
# your Cargo.toml
[package]
name = "your-package"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
fsextra = "*"

# optionally enable `crypto` feature
fsextra = { version = "*", features = ["crypto"] }
```

## Known Limitations

- `is_executable()` (`MetadataExtended`) is only supported on Unix-based platforms.

## Dependencies

- This library requires `ring@0.16.20` if `crypto` feature is enabled.

## Compatibility

This library supports Unix-based operating systems and **is not tested** for other operating systems at this time (`v0.3.0`). Since `v0.3.0`, any OS-specific functionality is hidden behind `cfg` attributes. Since `v0.3.0-alpha.1`, updates are tested against Unix (linux) and Windows operating systems.

- `v0.3.0` adds tested support for Windows platforms, adding `is_executable()` workaround.
- `v0.3.0-alpha.1` (and later) tests against Windows operating systems.
- `v0.3.0-alpha.0` (and later) introduces a new interface: `Digest`, requiring `DigestExt`, `Sha2` and `Sha5` to be moved to `legacy`. Code changes may be required when updating to `v0.3.0`.
- `v0.2.0` (and earlier) *may* compile for other operating systems, however, this may lead to undefined results and/or failures.

## Basic Usage

For complete examples, please see our [Documentation].

```rust
use fsextra::extensions::{MetadataExtended, FileExtended};
use fsextra::crypto::digest::{Digest, DigestAlgorithm};
use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let file = File::open("path/to/executable")?;
    let metadata = file.metadata()?;
    
    if !metadata.is_executable() {
        let digest = file.digest(DigestAlgorithm::Sha2);
        assert_eq!(hex::encode(digest), "7509e5bda0c762d2bac7f90d758b5b2263fa01ccbc542ab5e3df163be08e6ca9");
    }

    Ok(())
}
```

## Testing

Cross targets:

- `x86_64-pc-windows-gnu`
- `x86_64-unknown-linux-gnu`
- `arm-unknown-linux-gnueabi`

This library can be tested using Cargo (as usual) with `cargo test`. It's recommended to test with *and* without `--all-features`. When local testing, it's recommended to use [Cross] to achieve testing for the targets listed above.

To use [Cross] to test: `cross test --target {target} --all-features`.

## Learn More

- [Documentation]
- [Ring]
- [Changelog]
- [Cross]

[Documentation]: https://docs.rs/fsextra/0.3.0/fsextra/index.html
[Ring]: https://briansmith.org/rustdoc/ring/
[Changelog]: https://github.com/Isolated-/fsextra/blob/next/CHANGELOG.md
[Cross]: https://github.com/rust-embedded/cross