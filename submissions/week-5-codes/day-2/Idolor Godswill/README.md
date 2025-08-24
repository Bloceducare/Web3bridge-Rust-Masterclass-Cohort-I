# Stylus String Utils

A Rust implementation of OpenZeppelin's Strings.sol library for Arbitrum Stylus. This utility library provides essential string conversion functions for U256 values, enabling decimal and hexadecimal string representations in Stylus smart contracts.

## Features

- **Full OpenZeppelin Compatibility**: Implements all functions from OpenZeppelin's Strings.sol library
- **Decimal Conversion**: Convert U256 to decimal string representation
- **Hexadecimal Conversion**: Convert U256 to hex with various formatting options
- **Zero-cost Abstractions**: Optimized for Arbitrum Stylus smart contracts
- **Comprehensive Testing**: Full test coverage for all functions
- **Well Documented**: Complete documentation with examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
stylus-string-utils = "0.1.0"
stylus-sdk = "0.6.0"
```

## Quick Start

```rust
use stylus_string_utils::Strings;
use stylus_sdk::alloy_primitives::U256;

// Convert U256 to decimal string
let value = U256::from(12345);
let decimal = Strings::to_string(value);
assert_eq!(decimal, "12345");

// Convert U256 to hex string
let hex = Strings::to_hex_string(value);
assert_eq!(hex, "0x3039");
```

## API Reference

### Core Functions

#### `Strings::to_string(value: U256) -> String`

Converts a U256 value to its decimal string representation.

```rust
let value = U256::from(255);
let result = Strings::to_string(value);
assert_eq!(result, "255");
```

#### `Strings::to_hex_string(value: U256) -> String`

Converts a U256 value to its hexadecimal string representation with "0x" prefix.

```rust
let value = U256::from(255);
let result = Strings::to_hex_string(value);
assert_eq!(result, "0xff");
```

#### `Strings::to_hex_string_fixed_length(value: U256, length: usize) -> String`

Converts a U256 value to hexadecimal string with fixed length (zero-padded).

```rust
let value = U256::from(255);
let result = Strings::to_hex_string_fixed_length(value, 8);
assert_eq!(result, "0x000000ff");
```

### Additional Utility Functions

#### `Strings::to_hex_string_upper(value: U256) -> String`

Returns uppercase hexadecimal representation.

```rust
let value = U256::from(255);
let result = Strings::to_hex_string_upper(value);
assert_eq!(result, "0xFF");
```

#### `Strings::to_hex_string_no_prefix(value: U256) -> String`

Returns hexadecimal representation without "0x" prefix.

```rust
let value = U256::from(255);
let result = Strings::to_hex_string_no_prefix(value);
assert_eq!(result, "ff");
```

## Usage in Stylus Contracts

```rust
use stylus_string_utils::Strings;
use stylus_sdk::{
    alloy_primitives::U256,
    prelude::*,
    storage::StorageU256,
};

#[entrypoint]
#[storage]
pub struct MyContract {
    pub value: StorageU256,
}

#[public]
impl MyContract {
    pub fn set_value(&mut self, new_value: U256) {
        self.value.set(new_value);
    }

    pub fn get_value_as_string(&self) -> String {
        let value = self.value.get();
        Strings::to_string(value)
    }

    pub fn get_value_as_hex(&self) -> String {
        let value = self.value.get();
        Strings::to_hex_string(value)
    }
}
```

## OpenZeppelin Compatibility

This library provides full compatibility with OpenZeppelin's Strings.sol library:

| Solidity Function                            | Rust Equivalent                                      |
| -------------------------------------------- | ---------------------------------------------------- |
| `toString(uint256 value)`                    | `Strings::to_string(value)`                          |
| `toHexString(uint256 value)`                 | `Strings::to_hex_string(value)`                      |
| `toHexString(uint256 value, uint256 length)` | `Strings::to_hex_string_fixed_length(value, length)` |

## Examples

### Basic Usage

```rust
use stylus_string_utils::Strings;
use stylus_sdk::alloy_primitives::U256;

fn main() {
    // Decimal conversion
    let number = U256::from(1234567890u64);
    println!("Decimal: {}", Strings::to_string(number));
    // Output: Decimal: 1234567890

    // Hexadecimal conversion
    println!("Hex: {}", Strings::to_hex_string(number));
    // Output: Hex: 0x499602d2

    // Fixed-length hex
    println!("Hex (8 chars): {}", Strings::to_hex_string_fixed_length(number, 8));
    // Output: Hex (8 chars): 0x499602d2

    // Uppercase hex
    println!("Hex Upper: {}", Strings::to_hex_string_upper(number));
    // Output: Hex Upper: 0x499602D2
}
```

### Contract Integration

See the complete example contract in [`src/example.rs`](src/example.rs) for a full demonstration of how to integrate these utilities into your Stylus smart contracts.

## Building and Testing

```bash
# Build the library
cargo build

# Run tests
cargo test

# Build for Stylus deployment
cargo build --target wasm32-unknown-unknown --release

# Generate documentation
cargo doc --open
```

## Development

### Prerequisites

- Rust 1.70+
- Arbitrum Stylus SDK
- WASM target: `rustup target add wasm32-unknown-unknown`

### Project Structure

```
src/
├── lib.rs              # Main library implementation
├── example.rs # Example Stylus contract
Cargo.toml             # Project configuration
README.md              # This file
```
