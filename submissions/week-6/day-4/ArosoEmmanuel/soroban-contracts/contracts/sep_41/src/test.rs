#![cfg(test)]

use soroban_sdk::{Env, String, Address};
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use crate::token::{EmarcToken, EmarcTokenClient};

// Helper setup function with static lifetime
fn setup_env<'a>() -> (Env, Address, EmarcTokenClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(EmarcToken, ());
    let client = EmarcTokenClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    
    // Initialize the contract
    client.initialize(&admin, &INITIAL_SUPPLY);
    
    (env, admin, client)
}

// Helper to generate test users
fn generate_user(env: &Env) -> Address {
    Address::generate(env)
}

const INITIAL_SUPPLY: i128 = 1_000_000_0000000i128; // 1,000,000 tokens with 7 decimals
const TRANSFER_AMOUNT: i128 = 100_0000000i128; // 100 tokens
const MINT_AMOUNT: i128 = 500_0000000i128; // 500 tokens
const BURN_AMOUNT: i128 = 100_0000000i128; // 100 tokens

 #[test]
    fn test_initialization() {
        let (env, admin, client) = setup_env();
                    
        assert_eq!(client.total_supply(), INITIAL_SUPPLY);
        assert_eq!(client.balance(&admin), INITIAL_SUPPLY);
        assert_eq!(client.decimals(), 7);
        assert_eq!(client.name(), String::from_str(&env,"EmarcToken"));
        assert_eq!(client.symbol(), String::from_str(&env,"EMARC"));
    }
    
    #[test]
    fn test_transfer() {
        let (env, admin, client) = setup_env();
        let user1 = generate_user(&env);
                
        // Transfer from admin to user1
        client.transfer(&admin, &user1, &TRANSFER_AMOUNT);
        
        assert_eq!(client.balance(&admin), INITIAL_SUPPLY - TRANSFER_AMOUNT);
        assert_eq!(client.balance(&user1), TRANSFER_AMOUNT);
    }
    
    #[test]
    fn test_approve_and_transfer_from() {
        let (env, admin, client) = setup_env();
        
        let user1 = generate_user(&env);
        let user2 = generate_user(&env);
        
        // Transfer some tokens to user1
        client.transfer(&admin, &user1, &500_0000000i128);
        
        // User1 approves user2 to spend 100 tokens
        let allowance_amount = 100_0000000i128;
        let expiration = env.ledger().sequence() + 1000;
        client.approve(&user1, &user2, &allowance_amount, &expiration);
        
        assert_eq!(client.allowance(&user1, &user2), allowance_amount);
        
        // User2 transfers from user1 to themselves
        let transfer_amount = 50_0000000i128;
        client.transfer_from(&user2, &user1, &user2, &transfer_amount);
        
        assert_eq!(client.balance(&user1), 500_0000000 - transfer_amount);
        assert_eq!(client.balance(&user2), transfer_amount);
        assert_eq!(client.allowance(&user1, &user2), allowance_amount - transfer_amount);
    }
    
    #[test]
    fn test_burn() {
         let (_, admin, client) = setup_env();
        
        // Burn some tokens
        client.burn(&admin, &BURN_AMOUNT);
        
        assert_eq!(client.balance(&admin), INITIAL_SUPPLY - BURN_AMOUNT);
        assert_eq!(client.total_supply(), INITIAL_SUPPLY - BURN_AMOUNT);
    }
    
    #[test]
    fn test_mint() {
        let (env, _, client) = setup_env();
        let user1 = generate_user(&env);
        
        // Mint new tokens to user1
        client.mint(&user1, &MINT_AMOUNT);
        
        assert_eq!(client.balance(&user1), MINT_AMOUNT);
        assert_eq!(client.total_supply(), INITIAL_SUPPLY + MINT_AMOUNT);
    }

