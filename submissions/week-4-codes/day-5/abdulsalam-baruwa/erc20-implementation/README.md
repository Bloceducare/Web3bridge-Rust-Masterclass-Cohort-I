# Arbitrum Stylus ERC20 Token

A gas-optimized ERC20 token implementation for Arbitrum Stylus written in Rust.

## Features

- Full ERC20 compliance with standard functions
- Gas-optimized for Arbitrum Stylus runtime
- Comprehensive error handling and security checks
- Event logging for Transfer and Approval operations
- Safe allowance management functions

## ERC20 Functions

### Core Functions

- `initialize(name, symbol, decimals, initial_supply)` - Initialize token parameters
- `name()` - Returns token name
- `symbol()` - Returns token symbol
- `decimals()` - Returns decimal places
- `total_supply()` - Returns total token supply
- `balance_of(account)` - Returns account balance
- `transfer(to, amount)` - Transfer tokens
- `allowance(owner, spender)` - Returns allowance amount
- `approve(spender, amount)` - Approve token spending
- `transfer_from(from, to, amount)` - Transfer tokens using allowance

### Additional Functions

- `increase_allowance(spender, added_value)` - Safely increase allowance
- `decrease_allowance(spender, subtracted_value)` - Safely decrease allowance

## Prerequisites

1. Rust with `wasm32-unknown-unknown` target:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add wasm32-unknown-unknown
   ```

2. cargo-stylus CLI tool:
   ```bash
   cargo install cargo-stylus
   ```

## Building

### Standard Build

For development and testing:

```bash
# Check compilation
cargo check

# Build for development
cargo build

# Run tests
cargo test
```

### WASM Build for Stylus

Build the optimized WASM binary for Arbitrum Stylus:

```bash
# Make script executable
chmod +x build.sh

# Build optimized WASM binary
./build.sh
```

This will create `target/wasm32-unknown-unknown/release/erc20-implementation.wasm`

## Deployment

### Environment Setup

Set required environment variables:

```bash
export PRIVATE_KEY="your_private_key_here"
export RPC_URL="https://sepolia-rollup.arbitrum.io/rpc"
export TOKEN_NAME="MyToken"
export TOKEN_SYMBOL="MTK"
export TOKEN_DECIMALS="18"
export INITIAL_SUPPLY="1000000000000000000000000"
```

### Deploy to Arbitrum Stylus

```bash
./deploy.sh
```

### Initialize Token

After deployment, call the initialize function with your token parameters:

```solidity
initialize("MyToken", "MTK", 18, 1000000000000000000000000)
```

## Testing

Run the test suite:

```bash
cargo test
```

## Network Configuration

### Arbitrum Sepolia (Testnet)

- RPC URL: `https://sepolia-rollup.arbitrum.io/rpc`
- Chain ID: 421614

### Arbitrum One (Mainnet)

- RPC URL: `https://arb1.arbitrum.io/rpc`
- Chain ID: 42161

## Project Structure

```
erc20-implementation/
├── src/
│   ├── lib.rs          # Main ERC20 implementation
│   └── tests.rs        # Test suite
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── build.sh            # Build script for WASM
├── deploy.sh           # Deployment script
├── target/             # Build artifacts
└── README.md           # Documentation
```

## License

This project is licensed under the MIT License.
