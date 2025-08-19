# SEP-41 Token Contract

A complete implementation of the SEP-41 token standard for Stellar/Soroban smart contracts.

## Overview

This project implements a fully compliant SEP-41 token contract with the following features:

- **Standard Token Operations**: Transfer, approve, burn functionality
- **Administrative Controls**: Minting, admin management
- **Security Features**: Authorization requirements, input validation, allowance expiration
- **Event Emission**: Comprehensive event logging for transparency
- **Modular Design**: Clean separation of concerns with trait-based architecture

## Project Structure

```
contracts/sep41-token/
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── contract.rs      # Core contract implementation
│   ├── interface.rs     # SEP-41 trait definitions
│   ├── storage.rs       # State management and storage operations
│   └── test.rs          # Comprehensive test suite
├── Cargo.toml           # Contract dependencies and metadata
└── Makefile             # Build and deployment scripts
```

## Features

### Core Token Functionality (SEP-41 Compliant)

- `transfer(from, to, amount)` - Transfer tokens between addresses
- `transfer_from(spender, from, to, amount)` - Transfer using allowance
- `approve(from, spender, amount, expiration)` - Set spending allowance with expiration
- `allowance(from, spender)` - Query current allowance
- `balance(address)` - Query token balance
- `burn(from, amount)` - Burn tokens from address
- `burn_from(spender, from, amount)` - Burn tokens using allowance
- `decimals()` - Get token decimal places
- `name()` - Get token name
- `symbol()` - Get token symbol

### Administrative Functions

- `initialize(admin, decimals, name, symbol)` - Initialize the contract
- `mint(to, amount)` - Mint new tokens (admin only)
- `set_admin(new_admin)` - Change contract admin (admin only)
- `admin()` - Get current admin address
- `initialized()` - Check if contract is initialized

### Security Features

1. **Authorization**: All state-changing operations require proper authorization
2. **Input Validation**: Comprehensive validation of all parameters
3. **Allowance Expiration**: Prevents stale allowances with ledger-based expiration
4. **Overflow Protection**: Safe arithmetic operations
5. **Initialization Guard**: Prevents operations before proper initialization

## Usage

### Building the Contract

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Build optimized version
make build
```

### Deployment

1. **Initialize the contract**:
   ```rust
   token.initialize(
       admin_address,
       18,  // decimals
       String::from_str(&env, "My Token"),
       String::from_str(&env, "MTK")
   );
   ```

2. **Mint initial supply**:
   ```rust
   token.mint(recipient_address, 1_000_000_000_000_000_000); // 1 token with 18 decimals
   ```

3. **Users can then interact with the token**:
   ```rust
   // Transfer tokens
   token.transfer(from_address, to_address, amount);
   
   // Approve spending
   token.approve(owner, spender, amount, expiration_ledger);
   
   // Transfer from allowance
   token.transfer_from(spender, from, to, amount);
   ```

## Testing

The contract includes a comprehensive test suite covering:

- ✅ Basic functionality (transfer, approve, burn)
- ✅ Administrative operations (mint, set_admin)
- ✅ Edge cases (zero amounts, insufficient balance/allowance)
- ✅ Security validations (authorization, input validation)
- ✅ Allowance expiration mechanics
- ✅ Error conditions and proper panic handling

Run tests with:
```bash
cargo test
```

## Smart Contract Best Practices Implemented

1. **Trait-Based Design**: Clean separation using traits for different interfaces
2. **Modular Architecture**: Separate modules for storage, interface, and implementation
3. **Comprehensive Testing**: Full test coverage including edge cases
4. **Event Emission**: Proper events for all state changes
5. **Input Validation**: Thorough validation of all inputs
6. **Authorization Patterns**: Consistent use of `require_auth()`
7. **Storage Optimization**: Efficient use of different storage types (instance, persistent, temporary)
8. **Error Handling**: Clear error messages and proper panic conditions

## Events

The contract emits the following events:

- `("transfer", from, to)` with data `amount`
- `("approve", from, spender)` with data `(amount, expiration_ledger)`
- `("burn", from)` with data `amount`
- `("mint", to)` with data `amount`
- `("set_admin", old_admin, new_admin)` with no data

## Deployment Guide

### Prerequisites

- Rust toolchain with `wasm32-unknown-unknown` target
- Soroban CLI
- Stellar account with XLM for fees

### Steps

1. **Build the contract**:
   ```bash
   make build
   ```

2. **Deploy to network**:
   ```bash
   soroban contract deploy \
     --wasm target/wasm32-unknown-unknown/release/sep41_token.wasm \
     --source-account YOUR_ACCOUNT \
     --network testnet
   ```

3. **Initialize the contract**:
   ```bash
   soroban contract invoke \
     --id CONTRACT_ID \
     --source-account YOUR_ACCOUNT \
     --network testnet \
     -- initialize \
     --admin YOUR_ACCOUNT \
     --decimal 18 \
     --name "My Token" \
     --symbol "MTK"
   ```

## License

This project is licensed under the Apache License 2.0.

## Author

Oghenerukevwe Sandra Idjighere - Web3bridge Rust Masterclass Cohort I
