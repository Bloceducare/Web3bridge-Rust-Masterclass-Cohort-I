#[cfg(test)]
mod tests {
    use stylus_sdk::{
        alloy_primitives::{Address, U256},
        call_context::CallContext,
        prelude::*,
    };
    use super::erc20::{ERC20, Transfer, Approval};

    #[test]
    fn test_initialization() {
        let mut context = CallContext::new();
        let name = "TestToken".to_string();
        let symbol = "TST".to_string();
        let decimals = 18u8;
        let initial_supply = U256::from(1000);

        ERC20::new(&mut context, name.clone(), symbol.clone(), decimals, initial_supply);

        let erc20 = ERC20::storage(&context);
        assert_eq!(erc20.name.get(), name);
        assert_eq!(erc20.symbol.get(), symbol);
        assert_eq!(erc20.decimals.get(), decimals);
        assert_eq!(erc20.total_supply.get(), initial_supply);
        assert_eq!(erc20.balance_of(context.sender()), initial_supply);
    }

    #[test]
    fn test_transfer_success() {
        let mut context = CallContext::new();
        let initial_supply = U256::from(1000);
        ERC20::new(&mut context, "TestToken".to_string(), "TST".to_string(), 18, initial_supply);

        let to = Address::from([2; 20]);
        let amount = U256::from(100);
        let result = ERC20::transfer(&mut context, to, amount);

        assert!(result.is_ok());
        assert_eq!(ERC20::balance_of(&context, context.sender()), initial_supply - amount);
        assert_eq!(ERC20::balance_of(&context, to), amount);
    }

    #[test]
    fn test_transfer_insufficient_balance() {
        let mut context = CallContext::new();
        let initial_supply = U256::from(1000);
        ERC20::new(&mut context, "TestToken".to_string(), "TST".to_string(), 18, initial_supply);

        let to = Address::from([2; 20]);
        let amount = U256::from(1001);
        let result = ERC20::transfer(&mut context, to, amount);

        assert_eq!(result, Err("Insufficient balance".to_string()));
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let mut context = CallContext::new();
        let initial_supply = U256::from(1000);
        ERC20::new(&mut context, "TestToken".to_string(), "TST".to_string(), 18, initial_supply);

        let spender = Address::from([2; 20]);
        let to = Address::from([3; 20]);
        let amount = U256::from(100);

        // Approve
        assert!(ERC20::approve(&mut context, spender, amount));
        assert_eq!(ERC20::allowance(&context, context.sender(), spender), amount);

        // Change sender to spender
        context.set_sender(spender);
        let result = ERC20::transfer_from(&mut context, context.sender(), to, amount);

        assert!(result.is_ok());
        assert_eq!(ERC20::balance_of(&context, to), amount);
        assert_eq!(ERC20::balance_of(&context, context.sender()), initial_supply - amount);
        assert_eq!(ERC20::allowance(&context, context.sender(), spender), U256::zero());
    }

    #[test]
    fn test_transfer_from_insufficient_allowance() {
        let mut context = CallContext::new();
        let initial_supply = U256::from(1000);
        ERC20::new(&mut context, "TestToken".to_string(), "TST".to_string(), 18, initial_supply);

        let spender = Address::from([2; 20]);
        let to = Address::from([3; 20]);
        let amount = U256::from(100);

        // Approve less than needed
        ERC20::approve(&mut context, spender, U256::from(50));

        // Change sender to spender
        context.set_sender(spender);
        let result = ERC20::transfer_from(&mut context, context.sender(), to, amount);

        assert_eq!(result, Err("Insufficient allowance".to_string()));
    }
}