# Stylus AssemblyScript Example – Minimum Even Prime

This fork modifies the original [Stylus AssemblyScript Sieve of Eratosthenes](https://github.com/OffchainLabs/stylus-as-example) example to demonstrate a minimal Stylus smart contract that returns the smallest even prime number — which is always **2**.

## 🧠 What Changed

Instead of computing all primes up to a given number `n` using the Sieve of Eratosthenes algorithm, the logic has been simplified to return the only even prime number (2). This showcases a basic AssemblyScript contract in Stylus for educational and testing purposes, demonstrating how mathematical understanding can lead to dramatic code optimization.

## 🚀 Quick Start

Install the latest version of [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo

```bash
cargo install cargo-stylus
```

Add the wasm32-unknown-unknown build target to your Rust compiler:

```bash
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Install dependencies

```bash
yarn
```

Compile to WASM

```bash
yarn asbuild
```

Test locally (optional)

```bash
yarn test:local 56
```

Check WASM contract with stylus

```bash
cargo stylus check --wasm-file ./build/release.wasm
```

Estimate gas usage for deployment

```bash
cargo stylus deploy --wasm-file ./build/release.wasm --private-key=YOUR_PRIVATE_KEY --estimate-gas --no-verify
```

Deploy smart contract

```bash
cargo stylus deploy --wasm-file ./build/release.wasm --private-key=YOUR_PRIVATE_KEY --no-verify
```

Test on-chain (modify the contract address at the beginning of the file)

```bash
yarn test:onchain 56
```

## 📦 Usage

After building and deploying the contract, calling it with any number ≥ 2 will return:

```bash
yarn test:onchain 100  # returns 2
yarn test:onchain 50   # returns 2  
yarn test:onchain 2    # returns 2
```

## 📁 Files Modified

* `app.ts` → replaced Sieve of Eratosthenes logic with `getMinEvenPrime` function
* `README.md` → updated documentation to reflect new functionality

## 🔬 Mathematical Background

The function returns 2 because:
- 2 is prime (only divisible by 1 and itself)
- 2 is the only even number that is prime
- All other even numbers are divisible by 2, making them composite
- Therefore, 2 is both the minimum AND maximum even prime number

## 🏗️ About Stylus

Stylus is an upgrade to Arbitrum, an Ethereum-focused, smart contract blockchain that scales the network. In addition to supporting Ethereum smart contracts written in Solidity, Stylus supports contracts written in WebAssembly. Because AssemblyScript compiles to WASM, it can be used to create smart contracts to be used on Stylus.

## 📄 License

This project is fully open source, including an Apache-2.0 or MIT license at your choosing under your own copyright.
