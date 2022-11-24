# polkafuzz


Differential Fuzzing Framework for Polkafuzz.
Maintained by Sigma Prime for the Web3 Foundation.

## Overview

This project aims at identifying bugs, vulnerabilities, and non-conformance on various Polkadot Host implementations, by leveraging different fuzzing engines and techniques.


_**A note on terminology:** "client" and "implementation" are used interchangeably here to mean a specific Polkadot implementation._

## Implementations

* [Substrate](https://github.com/paritytech/substrate) (Parity, Rust)
* [Smoldot](https://github.com/paritytech/smoldot) (Parity, Rust)
* [Gossamer](https://github.com/ChainSafe/gossamer) (ChainSafe, Golang)


## Architecture overview

The following diagram describes the current architecture of polkafuzz:

<img src="./architecture.svg">

# ```polkafuzz_v2``` - Coverage Guided Fuzzer

The purpose of this tool is to identify crashes (i.e. panics) in Polkadot implementations. It uses multiple different fuzzing engines (AFL++, HonggFuzz, libFuzzer, etc.).

# ```polkafuzz_v1``` -  Differential Fuzzing with FFI Bindings

A differential fuzzer of Polkadot implementations using `libfuzzer` and `libafl`. It is developed in Rust (for ease of maintainability) and leverages Foreign Function Interfaces (FFI) bindings.

By leveraging the latest updates to the `libfuzzer-sys` and `cargo_fuzz` crates, this tool is able to write fuzz targets that take well-formed instances of custom types by deriving and implementing the `Arbitrary` trait, which allows us to create structured inputs from raw byte buffers.

# ```reproducer``` Replaying Samples Across Implementations

This tool can be used to reproduce crashes to help with debugging and triaging.

## Usage

Please refer to each tool's ```README``` for detailed instructions:

- `polkafuzz_v1` [README](./polkafuzz_v1/README.md)
- `polkafuzz_v2` [README](./polkafuzz_v2/README.md)
- `reproducer` [README](./reproducer/README.md)
