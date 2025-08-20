//! # Stylus String Utils
//!
//! A Rust implementation of OpenZeppelin's Strings.sol library for Arbitrum Stylus.
//! This utility library provides essential string conversion functions for U256 values,
//! enabling decimal and hexadecimal string representations in Stylus smart contracts.
//!
//! ## Features
//!
//! - Convert U256 to decimal string representation
//! - Convert U256 to hexadecimal string representation (with various formatting options)
//! - Full compatibility with OpenZeppelin's Strings.sol library
//! - Optimized for Arbitrum Stylus smart contracts
//!
//! ## Usage
//!
//! ```rust
//! use stylus_string_utils::Strings;
//! use alloy_primitives::U256;
//!
//! let value = U256::from(255);
//! let decimal = Strings::to_string(value);
//! let hex = Strings::to_hex_string(value);
//! ```

#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]
#![cfg_attr(not(test), no_std)]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};

#[cfg(feature = "stylus")]
use stylus_sdk::alloy_primitives::U256;

#[cfg(not(feature = "stylus"))]
use alloy_primitives::U256;

/// The main Strings utility struct providing string conversion functions for U256 values.
/// This is designed to be compatible with OpenZeppelin's Strings.sol library.
pub struct Strings;

impl Strings {
    /// Converts a U256 value to its decimal string representation.
    ///
    /// This function is equivalent to OpenZeppelin's `toString(uint256 value)` function.
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert to a decimal string
    ///
    /// # Returns
    ///
    /// A String containing the decimal representation of the input value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylus_string_utils::Strings;
    /// use alloy_primitives::U256;
    ///
    /// let value = U256::from(12345);
    /// let result = Strings::to_string(value);
    /// assert_eq!(result, "12345");
    /// ```
    pub fn to_string(value: U256) -> String {
        value.to_string()
    }

    /// Converts a U256 value to its hexadecimal string representation with "0x" prefix.
    ///
    /// This function is equivalent to OpenZeppelin's `toHexString(uint256 value)` function.
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert to a hexadecimal string
    ///
    /// # Returns
    ///
    /// A String containing the hexadecimal representation of the input value with "0x" prefix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylus_string_utils::Strings;
    /// use alloy_primitives::U256;
    ///
    /// let value = U256::from(255);
    /// let result = Strings::to_hex_string(value);
    /// assert_eq!(result, "0xff");
    /// ```
    pub fn to_hex_string(value: U256) -> String {
        if value.is_zero() {
            return "0x0".to_string();
        }
        format!("{:#x}", value)
    }

    /// Converts a U256 value to its hexadecimal string representation with "0x" prefix
    /// and ensures the result has a specific length (padding with zeros if necessary).
    ///
    /// This function is equivalent to OpenZeppelin's `toHexString(uint256 value, uint256 length)` function.
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert to a hexadecimal string
    /// * `length` - The desired length of the hexadecimal part (excluding "0x" prefix)
    ///
    /// # Returns
    ///
    /// A String containing the hexadecimal representation with "0x" prefix, padded to the specified length
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylus_string_utils::Strings;
    /// use alloy_primitives::U256;
    ///
    /// let value = U256::from(255);
    /// let result = Strings::to_hex_string_fixed_length(value, 4);
    /// assert_eq!(result, "0x00ff");
    /// ```
    pub fn to_hex_string_fixed_length(value: U256, length: usize) -> String {
        format!("0x{:0width$x}", value, width = length)
    }

    /// Converts a U256 value to its uppercase hexadecimal string representation with "0x" prefix.
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert to an uppercase hexadecimal string
    ///
    /// # Returns
    ///
    /// A String containing the uppercase hexadecimal representation with "0x" prefix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylus_string_utils::Strings;
    /// use alloy_primitives::U256;
    ///
    /// let value = U256::from(255);
    /// let result = Strings::to_hex_string_upper(value);
    /// assert_eq!(result, "0xFF");
    /// ```
    pub fn to_hex_string_upper(value: U256) -> String {
        if value.is_zero() {
            return "0x0".to_string();
        }
        format!("{:#X}", value)
    }

    /// Converts a U256 value to its hexadecimal string representation without "0x" prefix.
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert to a hexadecimal string
    ///
    /// # Returns
    ///
    /// A String containing the hexadecimal representation without "0x" prefix
    ///
    /// # Examples
    ///
    /// ```rust
    /// use stylus_string_utils::Strings;
    /// use alloy_primitives::U256;
    ///
    /// let value = U256::from(255);
    /// let result = Strings::to_hex_string_no_prefix(value);
    /// assert_eq!(result, "ff");
    /// ```
    pub fn to_hex_string_no_prefix(value: U256) -> String {
        if value.is_zero() {
            return "0".to_string();
        }
        format!("{:x}", value)
    }
}

/// Example contract demonstrating usage of the string utilities
#[cfg(feature = "stylus")]
pub mod example;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_zero() {
        let value = U256::ZERO;
        let result = Strings::to_string(value);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_to_string_small_number() {
        let value = U256::from(42);
        let result = Strings::to_string(value);
        assert_eq!(result, "42");
    }

    #[test]
    fn test_to_string_large_number() {
        let value = U256::from(1234567890u64);
        let result = Strings::to_string(value);
        assert_eq!(result, "1234567890");
    }

    #[test]
    fn test_to_hex_string_zero() {
        let value = U256::ZERO;
        let result = Strings::to_hex_string(value);
        assert_eq!(result, "0x0");
    }

    #[test]
    fn test_to_hex_string_small_number() {
        let value = U256::from(15);
        let result = Strings::to_hex_string(value);
        assert_eq!(result, "0xf");
    }

    #[test]
    fn test_to_hex_string_255() {
        let value = U256::from(255);
        let result = Strings::to_hex_string(value);
        assert_eq!(result, "0xff");
    }

    #[test]
    fn test_to_hex_string_large_number() {
        let value = U256::from(1234567890u64);
        let result = Strings::to_hex_string(value);
        assert_eq!(result, "0x499602d2");
    }

    #[test]
    fn test_to_hex_string_fixed_length_padding() {
        let value = U256::from(255);
        let result = Strings::to_hex_string_fixed_length(value, 8);
        assert_eq!(result, "0x000000ff");
    }

    #[test]
    fn test_to_hex_string_fixed_length_no_padding() {
        let value = U256::from(0x12345678u32);
        let result = Strings::to_hex_string_fixed_length(value, 4);
        assert_eq!(result, "0x12345678");
    }

    #[test]
    fn test_to_hex_string_upper() {
        let value = U256::from(255);
        let result = Strings::to_hex_string_upper(value);
        assert_eq!(result, "0xFF");
    }

    #[test]
    fn test_to_hex_string_no_prefix() {
        let value = U256::from(255);
        let result = Strings::to_hex_string_no_prefix(value);
        assert_eq!(result, "ff");
    }

    #[test]
    fn test_to_hex_string_no_prefix_zero() {
        let value = U256::ZERO;
        let result = Strings::to_hex_string_no_prefix(value);
        assert_eq!(result, "0");
    }
}