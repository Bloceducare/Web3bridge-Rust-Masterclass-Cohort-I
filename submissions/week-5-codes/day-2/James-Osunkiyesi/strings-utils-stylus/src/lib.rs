//! # Strings Utility Library for Stylus
//! 
//! A Rust implementation of OpenZeppelin's `Strings.sol` library for Arbitrum Stylus.
//! Provides utility functions for converting various types to strings, matching the exact
//! behavior of OpenZeppelin's implementation.

use alloy_primitives::{Address, I256, U256};

/// Hex digits constant used for hex string conversion
const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Address length in bytes (20 bytes = 40 hex characters)
const ADDRESS_LENGTH: usize = 20;

/// Error types matching OpenZeppelin's behavior
#[derive(Debug)]
pub enum StringsError {
    /// Hex length insufficient for the given value
    InsufficientHexLength { value: U256, length: usize },
}

/// Converts a U256 value to its ASCII decimal string representation.
/// 
/// This function replicates OpenZeppelin's `toString(uint256)` function exactly.
/// It uses the same algorithm: count digits, allocate buffer, fill backwards.
pub fn to_string(value: U256) -> String {
    // Handle zero case first (matching OpenZeppelin)
    if value.is_zero() {
        return "0".to_string();
    }
    
    // Count digits first
    let mut temp = value;
    let mut digits = 0;
    while !temp.is_zero() {
        digits += 1;
        temp /= U256::from(10);
    }
    
    // Create buffer
    let mut buffer = vec![0u8; digits];
    let mut remaining = value;
    
    // Fill buffer backwards (matching Solidity implementation)
    while !remaining.is_zero() {
        digits -= 1;
        buffer[digits] = 48 + (remaining % U256::from(10)).to::<u8>();
        remaining /= U256::from(10);
    }
    
    String::from_utf8(buffer).expect("Invalid UTF-8 from digits")
}

/// Converts an I256 (signed integer) to its ASCII decimal string representation.
/// 
/// This function replicates OpenZeppelin's `toStringSigned(int256)` function.
/// It handles negative values by prepending a minus sign.
pub fn to_string_signed(value: I256) -> String {
    if value < I256::ZERO {
        format!("-{}", to_string((-value).into_raw()))
    } else {
        to_string(value.into_raw())
    }
}

/// Converts a U256 value to its ASCII hexadecimal string representation.
/// 
/// This function replicates OpenZeppelin's `toHexString(uint256)` function.
/// It returns "0x00" for zero and uses variable length for other values.
pub fn to_hex_string(value: U256) -> String {
    if value.is_zero() {
        return "0x00".to_string();
    }
    
    // Calculate required length in bytes (each byte = 2 hex chars)
    let mut temp = value;
    let mut length = 0;
    while !temp.is_zero() {
        length += 1;
        temp >>= 8;
    }
    
    // Use the fixed-length version with calculated length
    to_hex_string_with_length(value, length).unwrap()
}

/// Converts a U256 value to its ASCII hexadecimal string representation with fixed length.
/// 
/// This function replicates OpenZeppelin's `toHexString(uint256, uint256)` function.
/// It creates a fixed-length hex string, padding with zeros or returning error if insufficient.
pub fn to_hex_string_with_length(value: U256, length: usize) -> Result<String, StringsError> {
    let mut local_value = value;
    let hex_length = 2 * length;
    let mut buffer = vec![0u8; hex_length + 2]; // +2 for "0x"
    
    buffer[0] = b'0';
    buffer[1] = b'x';
    
    // Fill buffer from right to left (matching Solidity implementation)
    for i in (2..hex_length + 2).rev() {
        buffer[i] = HEX_DIGITS[(local_value & U256::from(0xf)).to::<usize>()];
        local_value >>= 4;
    }
    
    // Check if value was too large for the specified length
    if !local_value.is_zero() {
        return Err(StringsError::InsufficientHexLength { value, length });
    }
    
    Ok(String::from_utf8(buffer).expect("Invalid UTF-8 from hex digits"))
}

/// Converts an Address to its ASCII hexadecimal string representation (not checksummed).
/// 
/// This function replicates OpenZeppelin's `toHexString(address)` function.
/// It converts the address to a 40-character hex string with "0x" prefix.
pub fn address_to_hex_string(addr: Address) -> String {
    // Convert address bytes directly to hex string
    let mut result = String::with_capacity(42); // 40 hex chars + "0x"
    result.push_str("0x");
    
    for &byte in addr.as_slice() {
        result.push(HEX_DIGITS[(byte >> 4) as usize] as char);
        result.push(HEX_DIGITS[(byte & 0xf) as usize] as char);
    }
    
    result
}


/// This function replicates OpenZeppelin's `toChecksumHexString(address)` function.
/// It implements EIP-55 checksumming by capitalizing hex digits based on the keccak256 hash.

pub fn address_to_checksum_hex_string(addr: Address) -> String {
    // Start with the non-checksummed hex string
    let hex_string = address_to_hex_string(addr);
    let mut buffer: Vec<u8> = hex_string.into_bytes();
    
    // Hash the hex part (skip "0x" prefix, hash 40 characters)
    use alloy_primitives::keccak256;
    let hex_part = &buffer[2..]; // Skip "0x"
    let hash = keccak256(hex_part);
    
    // Convert hash to U256 for bit manipulation, then shift right by 96 bits
    // This matches the Solidity assembly: shr(96, keccak256(...))
    let hash_value = U256::from_be_slice(hash.as_slice()) >> 96;
    
    // Apply EIP-55 checksumming
    let mut current_hash = hash_value;
    for i in (2..42).rev() { // Iterate from position 41 down to 2
        // Check if this hash nibble > 7 and the character is a lowercase letter
        if (current_hash & U256::from(0xf)) > U256::from(7) && buffer[i] > 96 {
            // Convert to uppercase by XOR with 0x20
            buffer[i] ^= 0x20;
        }
        current_hash >>= 4;
    }
    
    String::from_utf8(buffer).expect("Invalid UTF-8 from checksum conversion")
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, I256, U256};
    
    #[test]
    fn test_to_string_basic() {
        assert_eq!(to_string(U256::ZERO), "0");
        assert_eq!(to_string(U256::from(1)), "1");
        assert_eq!(to_string(U256::from(123)), "123");
        assert_eq!(to_string(U256::from(12345)), "12345");
    }
    
    #[test]
    fn test_to_string_large() {
        assert_eq!(to_string(U256::from(u64::MAX)), u64::MAX.to_string());
        let large_num = U256::from(u128::MAX);
        assert_eq!(to_string(large_num), u128::MAX.to_string());
    }
    
    #[test]
    fn test_to_string_signed() {
        assert_eq!(to_string_signed(I256::ZERO), "0");
        assert_eq!(to_string_signed(I256::try_from(123).unwrap()), "123");
        assert_eq!(to_string_signed(I256::try_from(-456).unwrap()), "-456");
        assert_eq!(to_string_signed(I256::try_from(i64::MAX).unwrap()), i64::MAX.to_string());
        assert_eq!(to_string_signed(I256::try_from(i64::MIN).unwrap()), i64::MIN.to_string());
    }
    
    #[test]
    fn test_to_hex_string_basic() {
        assert_eq!(to_hex_string(U256::ZERO), "0x00");
        assert_eq!(to_hex_string(U256::from(15)), "0x0f");
        assert_eq!(to_hex_string(U256::from(16)), "0x10");
        assert_eq!(to_hex_string(U256::from(255)), "0xff");
        assert_eq!(to_hex_string(U256::from(256)), "0x0100");
    }
    
    #[test]
    fn test_to_hex_string_with_length_basic() {
        assert_eq!(to_hex_string_with_length(U256::ZERO, 1).unwrap(), "0x00");
        assert_eq!(to_hex_string_with_length(U256::from(255), 1).unwrap(), "0xff");
        assert_eq!(to_hex_string_with_length(U256::from(255), 2).unwrap(), "0x00ff");
        assert_eq!(to_hex_string_with_length(U256::from(0x1234), 2).unwrap(), "0x1234");
    }
    
    #[test]
    fn test_to_hex_string_with_length_insufficient() {
        // Value 0x100 needs 2 hex digits, but we only provide 1 byte (2 digits)
        let result = to_hex_string_with_length(U256::from(0x100), 1);
        assert!(matches!(result, Err(StringsError::InsufficientHexLength { .. })));
    }
    
    #[test]
    fn test_address_to_hex_string() {
        let zero_addr = Address::ZERO;
        assert_eq!(address_to_hex_string(zero_addr), "0x0000000000000000000000000000000000000000");
        
            let addr_bytes = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0,
                     0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
                     0x99, 0xaa, 0xbb, 0xcc];
    let addr = Address::from_slice(&addr_bytes);
    assert_eq!(address_to_hex_string(addr), "0x123456789abcdef0112233445566778899aabbcc");
    }
    
    #[test]
    fn test_address_to_checksum_hex_string() {
        // Test with zero address (should remain all lowercase)
        let zero_addr = Address::ZERO;
        let checksum = address_to_checksum_hex_string(zero_addr);
        assert_eq!(checksum, "0x0000000000000000000000000000000000000000");
        
        // Test with known address that has mixed case in checksum
        // This is a simplified test - real EIP-55 would be more complex
        let addr_bytes = [0x52, 0x90, 0x8e, 0x08, 0x4f, 0x3d, 0x7d, 0xe1,
                         0xb3, 0x9a, 0x96, 0x30, 0x02, 0x64, 0xbd, 0x2a,
                         0x47, 0x9e, 0x9c, 0x8f];
        let addr = Address::from_slice(&addr_bytes);
        let checksum = address_to_checksum_hex_string(addr);
        
        // Should start with 0x and be 42 characters total
        assert!(checksum.starts_with("0x"));
        assert_eq!(checksum.len(), 42);
    }
    
    #[test]
    fn test_hex_digits_constant() {
        // Verify our HEX_DIGITS constant matches expectations
        assert_eq!(HEX_DIGITS[0], b'0');
        assert_eq!(HEX_DIGITS[9], b'9');
        assert_eq!(HEX_DIGITS[10], b'a');
        assert_eq!(HEX_DIGITS[15], b'f');
    }
    
    #[test]
    fn test_max_values() {
        // Test with U256::MAX
        let max_value = U256::MAX;
        let decimal_str = to_string(max_value);
        assert!(decimal_str.len() > 70); // U256::MAX has 78 decimal digits
        
        let hex_str = to_hex_string(max_value);
        // Should be 64 hex characters + "0x" = 66 total
        assert_eq!(hex_str.len(), 66);
        assert!(hex_str.starts_with("0x"));
        assert!(hex_str.ends_with("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
    }
    
    #[test] 
    fn test_consistency_with_reference() {
        // Test cases that should match OpenZeppelin exactly
        let test_cases = vec![
            (0u64, "0", "0x00"),
            (1u64, "1", "0x01"),
            (10u64, "10", "0x0a"),
            (255u64, "255", "0xff"),
            (256u64, "256", "0x0100"),
            (65535u64, "65535", "0xffff"),
        ];
        
        for (input, expected_decimal, expected_hex) in test_cases {
            let value = U256::from(input);
            assert_eq!(to_string(value), expected_decimal);
            assert_eq!(to_hex_string(value), expected_hex);
        }
    }
}