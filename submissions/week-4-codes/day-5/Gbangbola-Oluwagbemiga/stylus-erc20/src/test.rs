#[cfg(test)]
mod tests {
    use alloc::string::String;
    use stylus_erc20::ERC20Token;
    use stylus_sdk::alloy_primitives::{Address, U256};
    extern crate alloc;

    fn create_test_token() -> ERC20Token {
        let mut token = ERC20Token {
            name: Default::default(),
            symbol: Default::default(),
            decimals: Default::default(),
            total_supply: Default::default(),
            balances: Default::default(),
            allowances: Default::default(),
        };

        // Initialize with test values
        let _ = token.init(
            "TestToken".to_string(),
            "TEST".to_string(),
            18,
            U256::from(1000000u64) * U256::from(10u64).pow(U256::from(18u64)),
        );

        token
    }

    #[test]
    fn test_token_initialization() {
        let token = create_test_token();

        // Test basic properties
        assert_eq!(token.name().unwrap(), "TestToken");
        assert_eq!(token.symbol().unwrap(), "TEST");
        assert_eq!(token.decimals().unwrap(), 18);

        let expected_supply = U256::from(1000000u64) * U256::from(10u64).pow(U256::from(18u64));
        assert_eq!(token.totalSupply().unwrap(), expected_supply);
    }

    #[test]
    fn test_basic_functionality() {
        let token = create_test_token();

        // Test that total supply is correct
        let expected_supply = U256::from(1000000u64) * U256::from(10u64).pow(U256::from(18u64));
        assert_eq!(token.totalSupply().unwrap(), expected_supply);

        // Test name and symbol
        assert_eq!(token.name().unwrap(), "TestToken");
        assert_eq!(token.symbol().unwrap(), "TEST");
        assert_eq!(token.decimals().unwrap(), 18);
    }

    #[test]
    fn test_zero_balance() {
        let token = create_test_token();
        let random_address = Address::from([1u8; 20]);

        // Random address should have zero balance
        assert_eq!(token.balanceOf(random_address).unwrap(), U256::ZERO);
    }

    #[test]
    fn test_mint_functionality() {
        let mut token = create_test_token();
        let recipient = Address::from([2u8; 20]);
        let mint_amount = U256::from(1000u64);

        // Check initial state
        let initial_total_supply = token.totalSupply().unwrap();
        let initial_recipient_balance = token.balanceOf(recipient).unwrap();

        // Mint tokens
        let result = token.mint(recipient, mint_amount);
        assert!(result.is_ok());

        // Check final state
        assert_eq!(
            token.totalSupply().unwrap(),
            initial_total_supply + mint_amount
        );
        assert_eq!(
            token.balanceOf(recipient).unwrap(),
            initial_recipient_balance + mint_amount
        );
    }

    #[test]
    fn test_mint_to_zero_address_fails() {
        let mut token = create_test_token();
        let mint_amount = U256::from(1000u64);

        // This should fail - cannot mint to zero address
        let result = token.mint(Address::ZERO, mint_amount);
        assert!(result.is_err());

        let error_message = String::from_utf8(result.unwrap_err()).unwrap();
        assert!(error_message.contains("mint to zero address"));
    }
}
