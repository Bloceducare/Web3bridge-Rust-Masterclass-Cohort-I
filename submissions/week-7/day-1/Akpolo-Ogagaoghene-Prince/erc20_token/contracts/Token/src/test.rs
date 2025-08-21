
#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String, 
};

fn create_token<'a>(env: &Env, admin: &Address) -> contract::TokenClient<'a> {
    let token = contract::TokenClient::new(env, &env.register(contract::Token {}, ()));
    token.initialize(
        admin,
        &7u32,
        &String::from_str(env, "name"),
        &String::from_str(env, "symbol"),
    );
    token
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let token = create_token(&env, &admin1);

    assert_eq!(token.decimals(), 7);
    assert_eq!(token.name(), String::from_str(&env, "name"));
    assert_eq!(token.symbol(), String::from_str(&env, "symbol"));
    assert_eq!(token.admin(), admin1);

    token.set_admin(&admin2);
    assert_eq!(token.admin(), admin2);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token = create_token(&env, &admin);
    token.initialize(
        &admin,
        &10u32,
        &String::from_str(&env, "name2"),
        &String::from_str(&env, "symbol2"),
    );
}

#[test]
#[should_panic(expected = "decimal must not be greater than 18")]
fn initialize_invalid_decimal() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token_addr = env.register(contract::Token {}, ());
    let token = contract::TokenClient::new(&env, &token_addr);
    
    token.initialize(
        &admin,
        &19u32,
        &String::from_str(&env, "name"),
        &String::from_str(&env, "symbol"),
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.mint(&user2, &1);
    assert_eq!(token.balance(&user2), 1);

    token.mint(&user1, &2000);
    assert_eq!(token.balance(&user1), 3000);
}

#[test]
#[should_panic(expected = "negative amount is not allowed")]
fn mint_negative() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &-1);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &1001);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &600);
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    token.transfer(&user1, &user2, &400);
    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 1000);
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.burn(&user1, &500);
    assert_eq!(token.balance(&user1), 500);

    token.burn(&user1, &500);
    assert_eq!(token.balance(&user1), 0);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn burn_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.burn(&user1, &1001);
}

#[test]
fn test_approve() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

 
    let current_ledger = env.ledger().sequence();
    let expiration_ledger = current_ledger + 1000000;
    token.approve(&user1, &user2, &500, &expiration_ledger);
    assert_eq!(token.allowance(&user1, &user2), 500);
}

#[test]
fn test_transfer_from() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

  
    let current_ledger = env.ledger().sequence();
    let expiration_ledger = current_ledger + 1000000;
    token.approve(&user1, &user3, &500, &expiration_ledger);
    assert_eq!(token.allowance(&user1, &user3), 500);

    token.transfer_from(&user3, &user1, &user2, &400);
    assert_eq!(token.allowance(&user1, &user3), 100);
    assert_eq!(token.balance(&user1), 600);
    assert_eq!(token.balance(&user2), 400);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    let current_ledger = env.ledger().sequence();
    let expiration_ledger = current_ledger + 1000000;
    token.approve(&user1, &user3, &100, &expiration_ledger);
    assert_eq!(token.allowance(&user1, &user3), 100);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
fn test_burn_from() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

 
    let current_ledger = env.ledger().sequence();
    let expiration_ledger = current_ledger + 1000000;
    token.approve(&user1, &user3, &500, &expiration_ledger);
    assert_eq!(token.allowance(&user1, &user3), 500);

    token.burn_from(&user3, &user1, &400);
    assert_eq!(token.allowance(&user1, &user3), 100);
    assert_eq!(token.balance(&user1), 600);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn burn_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let token = create_token(&env, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);


    let current_ledger = env.ledger().sequence();
    let expiration_ledger = current_ledger + 1000000;
    token.approve(&user1, &user3, &100, &expiration_ledger);
    assert_eq!(token.allowance(&user1, &user3), 100);

    token.burn_from(&user3, &user1, &101);
}


#[test]
fn test_metadata_functions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_addr = env.register(contract::Token {}, ());
    let token = contract::TokenClient::new(&env, &token_addr);
    
    let name = String::from_str(&env, "Test Token");
    let symbol = String::from_str(&env, "TEST");
    let decimals = 8u32;
    
    token.initialize(&admin, &decimals, &name, &symbol);
    
    assert_eq!(token.name(), name);
    assert_eq!(token.symbol(), symbol);
    assert_eq!(token.decimals(), decimals);
}

#[test]
fn test_admin_change() {
    let env = Env::default();
    env.mock_all_auths();

    let admin1 = Address::generate(&env);
    let admin2 = Address::generate(&env);
    let token = create_token(&env, &admin1);

    assert_eq!(token.admin(), admin1);
    
    token.set_admin(&admin2);
    assert_eq!(token.admin(), admin2);
    
    // Old admin should not be able to mint anymore
    let user = Address::generate(&env);
    
    // This should work with new admin
    token.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);
}

#[test]
fn test_large_amounts() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token = create_token(&env, &admin);

    
    let large_amount = 1000000000000000000i128; // 1 token with 18 decimals
    
    token.mint(&user, &large_amount);
    assert_eq!(token.balance(&user), large_amount);
    
    token.burn(&user, &(large_amount / 2));
    assert_eq!(token.balance(&user), large_amount / 2);
}

#[test]
fn test_zero_amounts() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token(&env, &admin);

    // Test zero mint
    token.mint(&user1, &0);
    assert_eq!(token.balance(&user1), 0);
    
    // Test zero transfer
    token.mint(&user1, &1000);
    token.transfer(&user1, &user2, &0);
    assert_eq!(token.balance(&user1), 1000);
    assert_eq!(token.balance(&user2), 0);
    
    // Test zero approval (no expiration check for zero amounts)
    token.approve(&user1, &user2, &0, &0);
    assert_eq!(token.allowance(&user1, &user2), 0);
}
