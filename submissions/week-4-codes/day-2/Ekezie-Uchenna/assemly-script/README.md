# Stylus - AssemblyScript example contract (Sieve of Erathosthenes)

Example of a basic smart contract written in AssemblyScript and compiled to WebAssembly (WASM) to be used on Arbitrum Stylus. It contains an implementation of the [sieve of Erathosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes) algorithm.

Stylus is is an upgrade to Arbitrum, an Ethereum-focused, smart contract blockchain that scales the network. In addition to supporting Ethereum smart contracts written in Solidity, Stylus supports contracts written in WebAssembly. Because AssemblyScript compiles to WASM, it can be used to create smart contracts to be used on Stylus.

## Overview

In order to make your AS contract work on Stylus, there are a few things to keep in mind:
- The main entry point for the WASM contract has to be a specific function called `user_entrypoint`. There's no need to add any configuration options to specify it, but that function must exist, and should receive an i32 (the length of the byte stream received by input) and return another i32 (0 on success, and 1 on error).
- AssemblyScript will create a [start](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start) function by default, which is not supported on Stylus. To prevent AS from doing so, you must specify option `--exportStart` and pass a different name for the start function (e.g. `myStart`). Doing this will tell AS to export the start function, instead of explicitly calling it in the compiled wasm file.
- Input data is read from memory by calling the Stylus function `read_args`
- Output data is written in the memory by calling the Stylus function `write_result`
- `bulk-memory` needs to be disabled. When using it, AS will use the DataCountSection of WASM, which is not supported by Stylus yet
- The runtime variant to use must be `minimal` or `stub`, otherwise Stylus won't be able to handle memory instructions effectively
- A custom abort function needs to be declared
- It is recommended to use the optimization options available on AS

This repository holds all these changes and also wraps the Stylus specific flow into its own folder, `stylus`, so the developer only needs to worry about working from the `main()` function in the `app.ts` file. That `main()` function takes the bytes received by the smart contract in Uint8Array form, and has to return the bytes that the smart contract will output, also in Uint8Array form.

## Installation of the Stylus Cargo subcommand

Install the latest version of [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo
```shell
cargo install cargo-stylus
```

Add the wasm32-unknown-unknown build target to your Rust compiler:
```shell
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:
```shell
cargo stylus --help
```

## Steps to build and test

Install dependencies
```shell
yarn
```

Compile to WASM
```shell
yarn asbuild
```

Test locally (optional)
```shell
yarn test:local 56
```

Check WASM contract with stylus
```shell
cargo stylus check --wasm-file ./build/release.wasm
```

Estimate gas usage for deployment
```shell
cargo stylus deploy --wasm-file ./build/release.wasm --private-key=YOUR_PRIVATE_KEY --estimate-gas --no-verify
```

Deploy smart contract
```shell
cargo stylus deploy --wasm-file ./build/release.wasm --private-key=YOUR_PRIVATE_KEY --no-verify
```

Test on-chain (modify the contract address at the beginning of the file)
```shell
yarn test:onchain 56
```

## A note on the local testing file

The file `test/local.js` contains a very basic simulation of how `read_args` and `write_result` behave on Stylus. This file is included only as an example of how one could build a local testing environment fully on JS before deploying the smart contract on Stylus.

## Algorithm implementation

The implementation of the sieve of Erathosthenes algorithm is a slightly modified version of t-katsumura's implementation, available at https://github.com/t-katsumura/webassembly-examples-eratosthenes.

## License

This project is fully open source, including an Apache-2.0 or MIT license at your choosing under your own copyright.