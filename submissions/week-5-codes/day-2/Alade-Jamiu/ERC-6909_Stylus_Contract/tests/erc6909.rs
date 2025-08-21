#[cfg(test)]
mod tests {
    use super::*;
    use stylus_sdk::test_utils::{TestBackend, deploy_contract};

    #[test]
    fn test_erc6909() {
        let mut backend = TestBackend::new();
        let contract = deploy_contract::<ERC6909>(&mut backend);

        let owner = Address::from([1; 20]);
        let spender = Address::from([2; 20]);
        let operator = Address::from([3; 20]);
        let token_id = U256::from(1);
        let amount = U256::from(100);

        // Test minting
        contract.mint(owner, token_id, amount);
        assert_eq!(contract.total_supply(token_id), amount);
        assert_eq!(contract.balance_of(owner, token_id), amount);

        // Test approval
        backend.set_caller(owner);
        assert!(contract.approve(spender, token_id, amount));
        assert_eq!(contract.allowances.get(owner).get(spender).get(token_id), amount);

        // Test transfer_from
        backend.set_caller(spender);
        assert!(contract.transfer_from(owner, spender, token_id, amount / 2));
        assert_eq!(contract.balance_of(owner, token_id), amount / 2);
        assert_eq!(contract.balance_of(spender, token_id), amount / 2);

        // Test operator approval
        backend.set_caller(owner);
        assert!(contract.set_operator(operator, true));
        assert!(contract.operator_approval(owner, operator));

        // Test operator transfer
        backend.set_caller(operator);
        assert!(contract.transfer_from(owner, spender, token_id, amount / 2));
        assert_eq!(contract.balance_of(owner, token_id), U256::zero());
        assert_eq!(contract.balance_of(spender, token_id), amount);

        // Test error cases
        let result = std::panic::catch_unwind(|| {
            contract.transfer_from(owner, spender, token_id, U256::from(1));
        });
        assert!(result.is_err(), "Expected InsufficientBalance error");
    }
}