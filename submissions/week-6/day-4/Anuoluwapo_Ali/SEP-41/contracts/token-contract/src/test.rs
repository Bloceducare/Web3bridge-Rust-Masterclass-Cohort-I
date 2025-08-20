#![cfg(test)]

use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn create_token_contract(e: &Env, admin: &Address) -> Address {
        e.register_contract(None, Token {})
    }

    #[test]
    fn test_initialize() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);

        assert_eq!(client.name(), name);
        assert_eq!(client.symbol(), symbol);
        assert_eq!(client.decimals(), decimals);
        assert_eq!(client.total_supply(), total_supply);
        assert_eq!(client.admin(), admin);
        assert_eq!(client.balance(&admin), total_supply);
    }

    #[test]
    fn test_transfer() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);
        
        let transfer_amount = 100000i128;
        client.transfer(&admin, &user1, &transfer_amount);

        assert_eq!(client.balance(&admin), total_supply - transfer_amount);
        assert_eq!(client.balance(&user1), transfer_amount);

        let transfer_amount2 = 50000i128;
        client.transfer(&user1, &user2, &transfer_amount2);

        assert_eq!(client.balance(&user1), transfer_amount - transfer_amount2);
        assert_eq!(client.balance(&user2), transfer_amount2);
    }

    #[test]
    fn test_approve_and_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);
        
        let owner_amount = 100000i128;
        client.transfer(&admin, &owner, &owner_amount);

        let allowance_amount = 50000i128;
        let expiration = env.ledger().sequence() + 100;
        client.approve(&owner, &spender, &allowance_amount, &expiration);

        assert_eq!(client.allowance(&owner, &spender), allowance_amount);

        let transfer_amount = 30000i128;
        client.transfer_from(&spender, &owner, &recipient, &transfer_amount);

        assert_eq!(client.balance(&owner), owner_amount - transfer_amount);
        assert_eq!(client.balance(&recipient), transfer_amount);
        assert_eq!(client.allowance(&owner, &spender), allowance_amount - transfer_amount);
    }

    #[test]
    fn test_mint() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);

        let mint_amount = 500000i128;
        client.mint(&user, &mint_amount);

        assert_eq!(client.balance(&user), mint_amount);
        assert_eq!(client.total_supply(), total_supply + mint_amount);
    }

    #[test]
    fn test_burn() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);

        let user_amount = 100000i128;
        client.transfer(&admin, &user, &user_amount);

        let burn_amount = 50000i128;
        client.burn(&user, &burn_amount);

        assert_eq!(client.balance(&user), user_amount - burn_amount);
        assert_eq!(client.total_supply(), total_supply - burn_amount);
    }

    #[test]
    fn test_freeze_account() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);

        client.freeze_account(&user);
        assert_eq!(client.is_frozen(&user), true);

        client.unfreeze_account(&user);
        assert_eq!(client.is_frozen(&user), false);
    }

    #[test]
    #[should_panic(expected = "Already initialized")]
    fn test_double_initialize() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);
        client.initialize(&admin, &name, &symbol, &decimals, &total_supply); 
    }

    #[test]
    #[should_panic(expected = "Insufficient balance")]
    fn test_insufficient_balance() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        
        let token = create_token_contract(&env, &admin);
        let client = TokenClient::new(&env, &token);

        let name = String::from_str(&env, "Test Token");
        let symbol = String::from_str(&env, "TEST");
        let decimals = 7u32;
        let total_supply = 1000000i128;

        client.initialize(&admin, &name, &symbol, &decimals, &total_supply);

        client.transfer(&user, &admin, &1000i128); 
    }