# Minimal Zk authorize in a zkVM

## About ZK authorize

This is a minimal implementation for an asset transfer, utilizing Risc Zero's zkVM. This has the following features:

- private balances
- zero security
- plenty of logical inconsistensies
- missing tests
- other oddities

The point is to demonstrate how transaction policies could be attached to transactions, utilizing Risc Zero. The point is not to write useful code or usable solution.

## Installation

First, [install Rust] if you don't already have it.

Next, install the `cargo-risczero` tool and install the toolchain with:

```bash
cargo install cargo-risczero
cargo risczero install
```

[install Rust]: https://doc.rust-lang.org/cargo/getting-started/installation.html

Also get the Risc0 repository to a separate folder. This project references the folder with a relative path.

## Run minimal test

You can run the one minimal test to see how the functionality works.

It's recommended to run it in developer mode, which disables real proving and only generates hand-wavy proofs. This way the performance is way better for development.

Run the test with: `RISC0_DEV_MODE=1 cargo test`
