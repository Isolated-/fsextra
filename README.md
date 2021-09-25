# fsextra

> `fsextra` is a collection of extensions to simplify working with Unix-based filesystems. This library will also support cryptographic operations on files and directories by enabling the `crypto` feature (`> v0.2.0`).

- Version: **v0.2.0**

[![Build Status](https://app.travis-ci.com/Isolated-/fsextra.svg?branch=master)](https://app.travis-ci.com/Isolated-/fsextra)

## Introduction

`fsextra` is a library for simplifying work with Unix-based filesystems. This library will grow over time and non-essential/niche features will be controlled using feature flags to prevent bloat and unneeded dependencies.

Cryptographic operations (sha256 digest, AES encryption/decryption) can be enabled using the `crypto` feature flag.

**Recommendations?** - Please use the issue tracker to suggest new features/improvements (or implement it yourself and submit a PR).

## Basic Usage

```rust
use fsextra::extensions::MetadataExtended;
use std::fs::File;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("path/to/executable")?;
    let metadata = file.metadata()?;
    assert_eq!(metadata.is_executable(), true);

    Ok(())
}
```

## Installation

This package can be installed by adding `fsextra = "*"` to your `Cargo.toml` file.

An example of your `Cargo.toml` may look something like:

```toml
[package]
name = "your-package"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
fsextra = "*"
```

## SemVer

This library uses Semantic Versioning ([SemVer]) where a MAJOR represents breaking API changes, MINOR represents backward compatiable changes and PATCH represents a patch/hotfix.

## Collections

### Extensions

Added in `v0.1.0`, extensions cannot be disabled and main purpose of this library. Each extension provides additional methods on **existing types**. These types are imported **only** from `std`.

#### Metadata

`MetadataExtended` provides the following methods:

- `is_executable()` - returning true if the file is a file (not directory) and mode() == executable (using bitwise operator).

## Learn More

- [Documentation]
- [SemVer]

[Documentation]: https://docs.rs/fsextra/0.1.0/fsextra/all.html
[SemVer]: https://semver.org/