# Stylus AssemblyScript Example – Minimum Even Prime

This fork modifies the original [Stylus AssemblyScript Sieve of Eratosthenes](https://github.com/OffchainLabs/stylus-examples) example to demonstrate a minimal Stylus smart contract that returns the smallest even prime number — which is always **2**.

## 🧠 What Changed

Instead of computing all primes up to a given number `n`, the logic has been simplified to return the only even prime number (2). This showcases a basic AssemblyScript contract in Stylus for educational and testing purposes.

## Changes in this fork

- Updated `assembly/app.ts`: replace sieve implementation with a minimal function that always returns `2`.
- Updated `test/onchain.js`: new deployed contract address `0x525c2aba45f66987217323e8a05ea400c65d06dc`.
- Added proof-of-concept screenshot in `image/` and referenced it below.


## Proof of Concept

![Image](./image/Screenshot%202025-08-08%20at%2005.10.01.png)

## 📦 Usage

After building the contract:

```bash
npx stylus test
```


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

## Contract logic

This example intentionally returns the smallest even prime for any input, which is always `2`. It demonstrates the minimal structure of a Stylus AssemblyScript contract without additional algorithmic complexity.

## License

This project is fully open source, including an Apache-2.0 or MIT license at your choosing under your own copyright.
