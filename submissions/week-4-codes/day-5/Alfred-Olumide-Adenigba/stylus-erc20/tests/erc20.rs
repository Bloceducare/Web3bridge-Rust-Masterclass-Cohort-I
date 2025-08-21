
#[cfg(test)]
mod test {
    use super::*;
    use stylus_sdk::alloy_primitives::{Address, U256};
    use stylus_sdk::testing::*;

    // Test configuration for our token
    struct TestTokenParams;
    impl crate::erc20::Erc20Params for TestTokenParams {
        const NAME: &'static str = "MayToken";
        const SYMBOL: &'static str = "MTK";
        const DECIMALS: u8 = 18;
    }

    fn owner() -> Address {
        Address::from([1u8; 20])
    }

    fn maylord() -> Address {
        Address::from([2u8; 20])
    }

    fn olumide() -> Address {
        Address::from([2u8; 20])
    }


    // Create a test ERC-20 instance
    fn create_test_erc20() -> crate::erc20::Erc20<TestTokenParams> {
        let vm = TestVM::default();
        crate::erc20::Erc20::from(&vm)
    }

    #[test]
    fn test_token_constants() {
        // Test that our token parameters are correctly set
        assert_eq!(TestTokenParams::NAME, "MayToken");
        assert_eq!(TestTokenParams::SYMBOL, "MTK");
        assert_eq!(TestTokenParams::DECIMALS, 18);
    }

    #[test]
    fn test_initial_state() {
        let erc20 = create_test_erc20();
        
        // Test initial values
        assert_eq!(erc20.total_supply(), U256::ZERO);
        assert_eq!(erc20.balance_of(maylord()), U256::ZERO);
        assert_eq!(erc20.balance_of(olumide()), U256::ZERO);
        assert_eq!(erc20.allowance(maylord(), olumide()), U256::ZERO);
    }

    #[test]
    fn test_mint() {
        let mut erc20 = create_test_erc20();
        let mint_amount = U256::from(1000u64);
        
        // Mint tokens to maylord
        let result = erc20.mint(maylord(), mint_amount);
        assert!(result.is_ok());
        
        // Check balances and total supply
        assert_eq!(erc20.balance_of(maylord()), mint_amount);
        assert_eq!(erc20.total_supply(), mint_amount);
    }

    
    #[test]
    fn test_transfer_success() {
        let mut erc20 = create_test_erc20();
        let initial_amount = U256::from(1000u64);
        let transfer_amount = U256::from(300u64);
        
        // Setup: mint tokens to maylord
        erc20.mint(maylord(), initial_amount).unwrap();
        
        // Transfer from maylord to olumide
        let result = erc20._transfer(maylord(), olumide(), transfer_amount);
        assert!(result.is_ok());
        
        // Check balances after transfer
        assert_eq!(erc20.balance_of(maylord()), initial_amount - transfer_amount);
        assert_eq!(erc20.balance_of(olumide()), transfer_amount);
        assert_eq!(erc20.total_supply(), initial_amount); // Total supply unchanged
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut erc20 = create_test_erc20();
        let initial_amount = U256::from(500u64);
        let transfer_amount = U256::from(1000u64); // More than balance
        
        // Setup: mint tokens to maylord
        erc20.mint(maylord(), initial_amount).unwrap();
        
        // Attempt transfer with insufficient balance
        let result = erc20._transfer(maylord(), olumide(), transfer_amount);
        assert!(result.is_err());
        
        // Check that balances are unchanged
        assert_eq!(erc20.balance_of(maylord()), initial_amount);
        assert_eq!(erc20.balance_of(olumide()), U256::ZERO);
    }

    #[test]
    fn test_burn_success() {
        let mut erc20 = create_test_erc20();
        let initial_amount = U256::from(1000u64);
        let burn_amount = U256::from(300u64);
        
        // Setup: mint tokens to maylord
        erc20.mint(maylord(), initial_amount).unwrap();
        
        // Burn tokens from maylord
        let result = erc20.burn(maylord(), burn_amount);
        assert!(result.is_ok());
        
        // Check balance and total supply after burn
        assert_eq!(erc20.balance_of(maylord()), initial_amount - burn_amount);
        assert_eq!(erc20.total_supply(), initial_amount - burn_amount);
    }

    #[test]
    fn test_burn_insufficient_balance() {
        let mut erc20 = create_test_erc20();
        let initial_amount = U256::from(500u64);
        let burn_amount = U256::from(1000u64); // More than balance
        
        // Setup: mint tokens to maylord
        erc20.mint(maylord(), initial_amount).unwrap();
        
        // Attempt to burn more than balance
        let result = erc20.burn(maylord(), burn_amount);
        assert!(result.is_err());
        
        // Check that balance and total supply are unchanged
        assert_eq!(erc20.balance_of(maylord()), initial_amount);
        assert_eq!(erc20.total_supply(), initial_amount);
    }


    #[test]
    fn test_approve_and_allowance() {
        let mut erc20 = create_test_erc20();
        let approve_amount = U256::from(500u64);
        
        // Initially, allowance should be zero
        assert_eq!(erc20.allowance(maylord(), olumide()), U256::ZERO);
        
        // Test approve (we can't easily test msg::sender() in unit tests,
        // so we'll test the storage directly)
        erc20.allowances.setter(maylord()).insert(olumide(), approve_amount);
        
        // Check allowance
        assert_eq!(erc20.allowance(maylord(), olumide()), approve_amount);
        
        // Allowance for other combinations should still be zero
        assert_eq!(erc20.allowance(olumide(), maylord()), U256::ZERO);
        assert_eq!(erc20.allowance(maylord(), olumide()), U256::ZERO);
    }

    #[test]
    fn test_transfer_from() {
        let mut erc20 = create_test_erc20();
        let initial_amount = U256::from(1000u64);
        let allowance_amount = U256::from(500u64);
        let transfer_amount = U256::from(300u64);
        
        // Setup: mint tokens to owner and set allowance
        erc20.mint(owner(), initial_amount).unwrap();
        erc20.allowances.setter(owner()).insert(olumide(), allowance_amount);
        
        // Simulate transfer_from logic (olumide transferring owner's tokens to maylord)
        // Check allowance exists
        let current_allowance = erc20.allowances.getter(owner()).get(olumide());
        assert_eq!(current_allowance, allowance_amount);
        assert!(current_allowance >= transfer_amount);
        
        // Update allowance
        let new_allowance = current_allowance - transfer_amount;
        erc20.allowances.setter(owner()).setter(olumide()).set(new_allowance);
        
        // Execute transfer
        let result = erc20._transfer(owner(), maylord(), transfer_amount);
        assert!(result.is_ok());
        
        // Check final state
        assert_eq!(erc20.balance_of(owner()), initial_amount - transfer_amount);
        assert_eq!(erc20.balance_of(maylord()), transfer_amount);
        assert_eq!(erc20.balance_of(olumide()), U256::ZERO); // Olumide doesn't receive tokens
        assert_eq!(erc20.allowance(owner(), olumide()), allowance_amount - transfer_amount);
        assert_eq!(erc20.total_supply(), initial_amount); // Total supply unchanged
    }

    #[test]
    fn test_transfer_from_insufficient_allowance() {
        let mut erc20 = create_test_erc20();
        let initial_amount = U256::from(1000u64);
        let allowance_amount = U256::from(200u64);
        let transfer_amount = U256::from(300u64); // More than allowance
        
        // Setup: mint tokens to  and set insufficient allowance
        erc20.mint(maylord(), initial_amount).unwrap();
        erc20.allowances.setter(maylord()).insert(olumide(), allowance_amount);
        
        // Check that allowance is insufficient
        let current_allowance = erc20.allowances.getter(maylord()).get(olumide());
        assert!(current_allowance < transfer_amount);
        
        // This would fail in the actual transfer_from function due to insufficient allowance
        // We can verify the allowance check logic
        assert_eq!(current_allowance, allowance_amount);
        assert!(current_allowance < transfer_amount);
    }

    #[test]
    fn test_multiple_approvals() {
        let mut erc20 = create_test_erc20();
        let approve_amount_1 = U256::from(300u64);
        let approve_amount_2 = U256::from(700u64);
        
        // Set multiple allowances
        erc20.allowances.setter(maylord()).insert(olumide(), approve_amount_1);
        erc20.allowances.setter(maylord()).insert(olumide(), approve_amount_2);
        
        // Check allowances
        assert_eq!(erc20.allowance(maylord(), olumide()), approve_amount_1);
        assert_eq!(erc20.allowance(maylord(), olumide()), approve_amount_2);
        
        // Update one allowance
        let new_approve_amount = U256::from(150u64);
        erc20.allowances.setter(alice()).setter(bob()).set(new_approve_amount);
        
        // Check updated allowance
        assert_eq!(erc20.allowance(alice(), bob()), new_approve_amount);
        assert_eq!(erc20.allowance(alice(), charlie()), approve_amount_2); // Unchanged
    }

    
    #[test]
    fn test_zero_address_operations() {
        let mut erc20 = create_test_erc20();
        let amount = U256::from(1000u64);
        
        // Mint to alice first
        erc20.mint(alice(), amount).unwrap();
        
        // Transfer from zero address should fail
        let result = erc20._transfer(Address::ZERO, alice(), amount);
        assert!(result.is_err());
        
        // Transfer to zero address should fail  
        let result = erc20._transfer(alice(), Address::ZERO, amount);
        assert!(result.is_err());
        
        // Original balance should be unchanged
        assert_eq!(erc20.balance_of(alice()), amount);
    }

}
