//! # Example Stylus Contract
//!
//! This module demonstrates how to use the Stylus String Utils library
//! in an Arbitrum Stylus smart contract.

use crate::Strings;
use alloc::format;
use alloc::string::String;
use stylus_sdk::{
    alloy_primitives::U256,
    prelude::*,
    storage::{StorageString, StorageU256},
};

/// Example contract demonstrating string utility functions
#[entrypoint]
#[storage]
pub struct StringUtilsExample {
    /// Storage for a U256 value that can be converted to strings
    pub stored_value: StorageU256,
    /// Storage for the last decimal string conversion
    pub last_decimal: StorageString,
    /// Storage for the last hex string conversion
    pub last_hex: StorageString,
}

#[public]
impl StringUtilsExample {
    /// Set a value and store its string representations
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to store and convert
    pub fn set_value(&mut self, value: U256) {
        self.stored_value.set(value);

        // Convert to decimal and store
        let decimal_str = Strings::to_string(value);
        self.last_decimal.set_str(&decimal_str);

        // Convert to hex and store
        let hex_str = Strings::to_hex_string(value);
        self.last_hex.set_str(&hex_str);
    }

    /// Get the stored value as a decimal string
    ///
    /// # Returns
    ///
    /// The decimal string representation of the stored value
    pub fn get_decimal_string(&self) -> String {
        let value = self.stored_value.get();
        Strings::to_string(value)
    }

    /// Get the stored value as a hexadecimal string
    ///
    /// # Returns
    ///
    /// The hexadecimal string representation of the stored value with "0x" prefix
    pub fn get_hex_string(&self) -> String {
        let value = self.stored_value.get();
        Strings::to_hex_string(value)
    }

    /// Get the stored value as an uppercase hexadecimal string
    ///
    /// # Returns
    ///
    /// The uppercase hexadecimal string representation with "0x" prefix
    pub fn get_hex_string_upper(&self) -> String {
        let value = self.stored_value.get();
        Strings::to_hex_string_upper(value)
    }

    /// Get the stored value as a hexadecimal string without "0x" prefix
    ///
    /// # Returns
    ///
    /// The hexadecimal string representation without "0x" prefix
    pub fn get_hex_string_no_prefix(&self) -> String {
        let value = self.stored_value.get();
        Strings::to_hex_string_no_prefix(value)
    }

    /// Get the stored value as a fixed-length hexadecimal string
    ///
    /// # Arguments
    ///
    /// * `length` - The desired length of the hex string (excluding "0x" prefix)
    ///
    /// # Returns
    ///
    /// The hexadecimal string representation padded to the specified length
    pub fn get_hex_string_fixed_length(&self, length: U256) -> String {
        let value = self.stored_value.get();
        // Convert U256 length to usize (safe for reasonable values)
        let length_usize = length.to::<usize>();
        Strings::to_hex_string_fixed_length(value, length_usize)
    }

    /// Convert any U256 value to decimal string (pure function)
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert
    ///
    /// # Returns
    ///
    /// The decimal string representation of the input value
    pub fn convert_to_decimal(&self, value: U256) -> String {
        Strings::to_string(value)
    }

    /// Convert any U256 value to hex string (pure function)
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to convert
    ///
    /// # Returns
    ///
    /// The hexadecimal string representation of the input value
    pub fn convert_to_hex(&self, value: U256) -> String {
        Strings::to_hex_string(value)
    }

    /// Get the current stored value
    ///
    /// # Returns
    ///
    /// The current U256 value stored in the contract
    pub fn get_stored_value(&self) -> U256 {
        self.stored_value.get()
    }

    /// Get the last stored decimal string
    ///
    /// # Returns
    ///
    /// The last decimal string that was stored
    pub fn get_last_decimal(&self) -> String {
        self.last_decimal.get_string()
    }

    /// Get the last stored hex string
    ///
    /// # Returns
    ///
    /// The last hex string that was stored
    pub fn get_last_hex(&self) -> String {
        self.last_hex.get_string()
    }

    /// Demonstrate all string conversion methods for a given value
    ///
    /// # Arguments
    ///
    /// * `value` - The U256 value to demonstrate conversions for
    ///
    /// # Returns
    ///
    /// A formatted string showing all conversion results
    pub fn demonstrate_conversions(&self, value: U256) -> String {
        let decimal = Strings::to_string(value);
        let hex = Strings::to_hex_string(value);
        let hex_upper = Strings::to_hex_string_upper(value);
        let hex_no_prefix = Strings::to_hex_string_no_prefix(value);
        let hex_fixed = Strings::to_hex_string_fixed_length(value, 8);

        format!(
            "Value: {} | Decimal: {} | Hex: {} | Hex Upper: {} | Hex No Prefix: {} | Hex Fixed(8): {}",
            value, decimal, hex, hex_upper, hex_no_prefix, hex_fixed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the pure functions that don't require storage
    #[test]
    fn test_string_functions_used_in_contract() {
        // Test the core string utility functions that the contract uses
        let value = U256::from(12345);

        // Test decimal conversion
        let decimal = Strings::to_string(value);
        assert_eq!(decimal, "12345");

        // Test hex conversion
        let hex = Strings::to_hex_string(value);
        assert_eq!(hex, "0x3039");

        // Test other variants
        let hex_upper = Strings::to_hex_string_upper(value);
        assert_eq!(hex_upper, "0x3039");

        let hex_no_prefix = Strings::to_hex_string_no_prefix(value);
        assert_eq!(hex_no_prefix, "3039");

        let hex_fixed = Strings::to_hex_string_fixed_length(value, 8);
        assert_eq!(hex_fixed, "0x00003039");
    }

    #[test]
    fn test_demonstrate_conversions_format() {
        // Test that the demonstration format works correctly
        let value = U256::from(255);
        let decimal = Strings::to_string(value);
        let hex = Strings::to_hex_string(value);
        let hex_upper = Strings::to_hex_string_upper(value);
        let hex_no_prefix = Strings::to_hex_string_no_prefix(value);
        let hex_fixed = Strings::to_hex_string_fixed_length(value, 8);

        let expected_format = format!(
            "Value: {} | Decimal: {} | Hex: {} | Hex Upper: {} | Hex No Prefix: {} | Hex Fixed(8): {}",
            value, decimal, hex, hex_upper, hex_no_prefix, hex_fixed
        );

        assert!(expected_format.contains("Decimal: 255"));
        assert!(expected_format.contains("Hex: 0xff"));
        assert!(expected_format.contains("Hex Upper: 0xFF"));
        assert!(expected_format.contains("Hex No Prefix: ff"));
        assert!(expected_format.contains("Hex Fixed(8): 0x000000ff"));
    }
}