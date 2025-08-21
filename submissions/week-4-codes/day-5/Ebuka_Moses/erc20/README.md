# ERC20 Token Implementation in Stylus

This project is a complete implementation of the ERC20 token standard written in Rust for Arbitrum Stylus. It demonstrates how to build efficient, gas-optimized smart contracts using Stylus while maintaining full compatibility with the Ethereum ecosystem.

## What is Stylus?

Stylus is Arbitrum's next-generation programming environment that allows developers to write smart contracts in Rust, C, and C++ instead of Solidity. These contracts compile to WebAssembly (WASM) and run alongside traditional EVM contracts, offering significantly lower gas costs and improved performance.

## Contract Structure

The implementation consists of two main files:

- `src/lib.rs` - Main contract entry point with token configuration
- `src/erc20.rs` - Core ERC20 implementation with all standard methods

### Token Configuration
```rust
struct StylusTokenParams;

impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STR";
    const DECIMALS: u8 = 18;
}
```

## Quick Start

### Prerequisites

Install Rust and the Stylus CLI:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Stylus CLI
cargo install --force cargo-stylus cargo-stylus-check

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Building the Contract

```bash
# Check compilation
cargo stylus check

# Export ABI
cargo stylus export-abi
```

### Deployment

```bash
# Deploy to Stylus testnet
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH>
```

## Gas Efficiency

Stylus contracts offer significant gas savings compared to Solidity:

- **Deployment**: ~10x cheaper
- **Execution**: ~5-10x cheaper for compute-heavy operations
- **Storage**: Similar costs to Solidity


## Development

### Testing

```bash
cargo test
```

### Optimization

For production deployments, optimize the WASM binary:

```bash
cargo stylus deploy --optimize
```