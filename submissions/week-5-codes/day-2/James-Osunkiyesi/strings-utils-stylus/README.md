# strings-utils-stylus

A Rust implementation of OpenZeppelin's `Strings.sol` library for Arbitrum Stylus. This utility library provides essential string conversion functions for U256 values, enabling decimal and hexadecimal string representations in Stylus smart contracts.

## Features

- **`to_string(U256)`** - Convert U256 to decimal string representation
- **`to_hex_string(U256)`** - Convert U256 to hexadecimal string with "0x" prefix
- **`to_hex_string_fixed(U256, length)`** - Convert U256 to fixed-length hex string with padding
- Zero dependencies beyond `alloy-primitives`
- Comprehensive test coverage including edge cases
- Optimized for gas efficiency in Stylus contracts

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
strings_utils = { git = "https://github.com/YOUR_USERNAME/strings-utils-stylus" }
alloy-primitives = "0.7"
```

## Quick Start

```rust
use alloy_primitives::U256;
use strings_utils::{to_string, to_hex_string, to_hex_string_fixed};

fn main() {
    let value = U256::from(12345);
    
    // Convert to decimal string
    let decimal = to_string(value);
    println!("Decimal: {}", decimal); // "12345"
    
    // Convert to hex string
    let hex = to_hex_string(value);
    println!("Hex: {}", hex); // "0x3039"
    
    // Convert to fixed-length hex string
    let hex_fixed = to_hex_string_fixed(value, 8);
    println!("Fixed Hex: {}", hex_fixed); // "0x00003039"
}
```

## API Reference

### `to_string(value: U256) -> String`

Converts a U256 value to its ASCII decimal string representation.

**Parameters:**
- `value`: The U256 value to convert

**Returns:** String containing the decimal representation

**Examples:**
```rust
assert_eq!(to_string(U256::from(0)), "0");
assert_eq!(to_string(U256::from(12345)), "12345");
assert_eq!(to_string(U256::from(u64::MAX)), "18446744073709551615");
```

### `to_hex_string(value: U256) -> String`

Converts a U256 value to its hexadecimal string representation with "0x" prefix. The output length varies based on the value (no leading zeros except for zero value).

**Parameters:**
- `value`: The U256 value to convert

**Returns:** String containing the hexadecimal representation with "0x" prefix

**Examples:**
```rust
assert_eq!(to_hex_string(U256::from(0)), "0x0");
assert_eq!(to_hex_string(U256::from(255)), "0xff");
assert_eq!(to_hex_string(U256::from(0xdeadbeef)), "0xdeadbeef");
```

### `to_hex_string_fixed(value: U256, length: usize) -> String`

Converts a U256 value to a fixed-length hexadecimal string with "0x" prefix. If the value requires fewer hex characters than specified, leading zeros are added. If the value requires more characters, all necessary characters are included (no truncation).

**Parameters:**
- `value`: The U256 value to convert
- `length`: The desired number of hex characters (excluding "0x" prefix)

**Returns:** String containing the fixed-length hexadecimal representation

**Examples:**
```rust
assert_eq!(to_hex_string_fixed(U256::from(255), 4), "0x00ff");
assert_eq!(to_hex_string_fixed(U256::from(0), 8), "0x00000000");
assert_eq!(to_hex_string_fixed(U256::from(0x12345), 4), "0x12345"); // No truncation
```

## Common Use Cases

### Token URI Generation
```rust
use strings_utils::{to_string, to_hex_string_fixed};

fn generate_token_uri(token_id: U256) -> String {
    let id_str = to_string(token_id);
    let hex_id = to_hex_string_fixed(token_id, 8);
    format!("https://api.example.com/token/{}/metadata?hex={}", id_str, hex_id)
}
```

### Address Formatting
```rust
use strings_utils::to_hex_string_fixed;

fn format_address(addr: U256) -> String {
    // Ethereum addresses are 20 bytes = 40 hex characters
    to_hex_string_fixed(addr, 40)
}
```

### Debug Output
```rust
use strings_utils::{to_string, to_hex_string};

fn debug_value(value: U256) -> String {
    format!("Value: {} (hex: {})", to_string(value), to_hex_string(value))
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

The test suite includes:
- Basic functionality tests for all three functions
- Edge cases (zero, maximum values)
- Consistency checks between functions
- Large number handling (up to U256::MAX)

## Performance Considerations

- **Efficiency**: Functions use iterative algorithms optimized for gas efficiency
- **Memory**: Minimal heap allocations, pre-calculating buffer sizes where possible
- **Gas Usage**: Designed to minimize gas consumption in Stylus contracts

## Compatibility

This library is designed specifically for:
- **Arbitrum Stylus** smart contracts
- **Alloy SDK** integration
- **Rust ecosystem** compatibility

## Comparison with OpenZeppelin Strings.sol

| Feature | Solidity (Strings.sol) | This Library |
|---------|------------------------|--------------|
| `toString(uint256)` | ✅ | ✅ `to_string(U256)` |
| `toHexString(uint256)` | ✅ | ✅ `to_hex_string(U256)` |
| `toHexString(uint256, uint256)` | ✅ | ✅ `to_hex_string_fixed(U256, usize)` |
| Gas Efficiency | Good | Optimized for Stylus |
| Type Safety | Solidity types | Rust strong typing |

## Examples

### Basic Usage in a Stylus Contract

```rust
use alloy_primitives::U256;
use strings_utils::{to_string, to_hex_string, to_hex_string_fixed};

// In your Stylus contract implementation
pub fn get_formatted_balance(balance: U256) -> String {
    // Convert wei to a readable string
    to_string(balance)
}

pub fn get_transaction_hash_formatted(hash: U256) -> String {
    // Format as a standard 64-character hex string
    to_hex_string_fixed(hash, 64)
}

pub fn get_short_address(address: U256) -> String {
    // Get last 4 bytes as hex for display
    let short_addr = address & U256::from(0xffffffffu64);
    to_hex_string_fixed(short_addr, 8)
}
```

### Error Handling

All functions in this library are designed to be infallible for valid U256 inputs. They will not panic under normal circumstances and handle edge cases gracefully:

- **Zero values**: Return appropriate representations ("0", "0x0")
- **Maximum values**: Handle U256::MAX without overflow
- **Invalid lengths**: Fixed-length functions never truncate, only pad

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Generating Documentation

```bash
cargo doc --open
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

- Inspired by [OpenZeppelin's Strings.sol](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/utils/Strings.sol)
- Built for the [Arbitrum Stylus](https://docs.arbitrum.io/stylus/stylus-gentle-introduction) ecosystem
- Uses [Alloy](https://github.com/alloy-rs/alloy) primitives for U256 type support