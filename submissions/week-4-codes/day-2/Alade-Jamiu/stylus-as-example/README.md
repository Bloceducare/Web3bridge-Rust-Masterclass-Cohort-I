# Stylus - AssemblyScript Example Contract (Minimum Even Prime)

This fork modifies the original [Stylus AssemblyScript Sieve of Eratosthenes example](https://github.com/OffchainLabs/stylus-examples) to demonstrate a minimal Stylus smart contract written in AssemblyScript and compiled to WebAssembly (WASM) for Arbitrum Stylus. Instead of implementing the [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) algorithm to find the maximum prime number, the contract now returns the minimum even prime number (2).

Stylus is an upgrade to Arbitrum, an Ethereum-focused, smart contract blockchain that scales the network. In addition to supporting Ethereum smart contracts written in Solidity, Stylus supports contracts written in WebAssembly. Because AssemblyScript compiles to WASM, it can be used to create smart contracts for Stylus.

## Overview

In order to make your AssemblyScript contract work on Stylus, there are a few things to keep in mind:
- The main entry point for the WASM contract must be a function called `user_entrypoint`. This function must exist, receive an `i32` (the length of the input byte stream), and return an `i32` (0 on success, 1 on error).
- AssemblyScript creates a [start](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start) function by default, which is not supported on Stylus. To prevent this, specify the `--exportStart` option and provide a different name for the start function (e.g., `myStart`). This tells AssemblyScript to export the start function instead of calling it in the compiled WASM file.
- Input data is read from memory using the Stylus function `read_args`.
- Output data is written to memory using the Stylus function `write_result`.
- The `bulk-memory` feature must be disabled, as AssemblyScript’s use of the DataCountSection in WASM is not yet supported by Stylus.
- The runtime variant must be `minimal` or `stub` to ensure Stylus can handle memory instructions effectively.
- A custom `abort` function must be declared.
- It is recommended to use AssemblyScript’s optimization options.

This repository wraps Stylus-specific logic in the `stylus` folder, so developers can focus on the `main` function in the `app.ts` file. The `main` function takes input bytes as a `Uint8Array` and returns output bytes as a `Uint8Array`.

## What Changed

The original contract implemented the Sieve of Eratosthenes to find the largest prime number up to a given input `n`. This fork simplifies the logic in `app.ts` to return the minimum even prime number (2), demonstrating a minimal Stylus smart contract for educational and testing purposes.

## Installation of the Stylus Cargo Subcommand

Install the latest version of [Rust](https://www.rust-lang.org/tools/install), then install the Stylus CLI tool with Cargo:
```shell
cargo install cargo-stylus