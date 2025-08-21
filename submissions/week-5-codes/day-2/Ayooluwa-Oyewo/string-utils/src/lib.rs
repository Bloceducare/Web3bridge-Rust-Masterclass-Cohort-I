//! # Strings Utilities for Stylus
//! 
//! A Rust implementation of OpenZeppelin's Strings.sol functionality for Stylus contracts.
//! Provides utility functions for converting integers to decimal and hexadecimal strings.

use alloy_primitives::U256;


pub fn to_string(value: U256) -> String {
    if value.is_zero() {
        return "0".to_string();
    }
    
    let mut buffer = Vec::new();
    let mut temp = value;
    
    // Extract digits in reverse order
    while !temp.is_zero() {
        let digit = temp % U256::from(10);
        buffer.push((digit.to::<u8>() + b'0') as char);
        temp /= U256::from(10);
    }
    
    // Reverse to get correct order
    buffer.reverse();
    buffer.into_iter().collect()
}


pub fn to_hex_string(value: U256) -> String {
    if value.is_zero() {
        return "0x0".to_string();
    }
    
    format!("0x{:x}", value)
}

pub fn to_hex_string_fixed(value: U256, length: usize) -> String {
    let hex_without_prefix = format!("{:x}", value);
    
    if hex_without_prefix.len() > length {
        panic!("Value requires more than {} hex digits", length);
    }
    
    format!("0x{:0width$x}", value, width = length)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::U256;

    #[test]
    fn test_to_string_zero() {
        assert_eq!(to_string(U256::ZERO), "0");
    }

    #[test]
    fn test_to_string_small_values() {
        assert_eq!(to_string(U256::from(1)), "1");
        assert_eq!(to_string(U256::from(42)), "42");
        assert_eq!(to_string(U256::from(123)), "123");
        assert_eq!(to_string(U256::from(999)), "999");
    }

    #[test]
    fn test_to_string_large_values() {
        assert_eq!(to_string(U256::from(1000)), "1000");
        assert_eq!(to_string(U256::from(123456789)), "123456789");
        assert_eq!(to_string(U256::from(u64::MAX)), u64::MAX.to_string());
    }

    #[test]
    fn test_to_string_max_u256() {
        let max_value = U256::MAX;
        let result = to_string(max_value);
        // U256::MAX = 2^256 - 1 = 115792089237316195423570985008687907853269984665640564039457584007913129639935
        assert_eq!(result, "115792089237316195423570985008687907853269984665640564039457584007913129639935");
    }

    #[test]
    fn test_to_hex_string_zero() {
        assert_eq!(to_hex_string(U256::ZERO), "0x0");
    }

    #[test]
    fn test_to_hex_string_small_values() {
        assert_eq!(to_hex_string(U256::from(1)), "0x1");
        assert_eq!(to_hex_string(U256::from(15)), "0xf");
        assert_eq!(to_hex_string(U256::from(16)), "0x10");
        assert_eq!(to_hex_string(U256::from(255)), "0xff");
        assert_eq!(to_hex_string(U256::from(256)), "0x100");
    }

    #[test]
    fn test_to_hex_string_large_values() {
        assert_eq!(to_hex_string(U256::from(4096)), "0x1000");
        assert_eq!(to_hex_string(U256::from(65535)), "0xffff");
        assert_eq!(to_hex_string(U256::from(1048576)), "0x100000");
    }

    #[test]
    fn test_to_hex_string_max_u256() {
        let max_value = U256::MAX;
        let result = to_hex_string(max_value);
        assert_eq!(result, "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
    }

    #[test]
    fn test_to_hex_string_fixed_zero() {
        assert_eq!(to_hex_string_fixed(U256::ZERO, 1), "0x0");
        assert_eq!(to_hex_string_fixed(U256::ZERO, 4), "0x0000");
        assert_eq!(to_hex_string_fixed(U256::ZERO, 8), "0x00000000");
    }

    #[test]
    fn test_to_hex_string_fixed_padding() {
        assert_eq!(to_hex_string_fixed(U256::from(42), 4), "0x002a");
        assert_eq!(to_hex_string_fixed(U256::from(255), 8), "0x000000ff");
        assert_eq!(to_hex_string_fixed(U256::from(4096), 6), "0x001000");
    }

    #[test]
    fn test_to_hex_string_fixed_exact_length() {
        assert_eq!(to_hex_string_fixed(U256::from(255), 2), "0xff");
        assert_eq!(to_hex_string_fixed(U256::from(4095), 3), "0xfff");
    }

    #[test]
    #[should_panic(expected = "Value requires more than 2 hex digits")]
    fn test_to_hex_string_fixed_overflow() {
        // 256 = 0x100, which requires 3 hex digits
        to_hex_string_fixed(U256::from(256), 2);
    }

    #[test]
    fn test_to_hex_string_fixed_64_chars() {
        // Test with full 32-byte (64 hex chars) representation
        let large_value = U256::from_str_radix("123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0", 16).unwrap();
        let result = to_hex_string_fixed(large_value, 64);
        assert_eq!(result, "0x123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0");
    }

    // Integration tests comparing different functions
    #[test]
    fn test_integration_same_values() {
        let test_values = [
            U256::from(0),
            U256::from(1),
            U256::from(42),
            U256::from(255),
            U256::from(256),
            U256::from(4096),
        ];

        for value in test_values {
            let decimal = to_string(value);
            let hex = to_hex_string(value);
            let hex_fixed = to_hex_string_fixed(value, 8);
            
            println!("Value: {}, Decimal: {}, Hex: {}, Hex Fixed: {}", value, decimal, hex, hex_fixed);
            
            // Verify conversions are consistent
            assert_eq!(value.to_string(), decimal);
            assert!(hex.starts_with("0x"));
            assert!(hex_fixed.starts_with("0x"));
            assert_eq!(hex_fixed.len(), 10); // "0x" + 8 characters
        }
    }
}