#![cfg(test)]

use crate::Token;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

// Generate the client for the contract
soroban_sdk::contractclient! {
    pub struct TokenClient;
}

fn create_token_contract(env: &Env) -> TokenClient<'_> {
    let contract_address = env.register_contract(None, Token {});
    TokenClient::new(env, &contract_address)
}

fn advance_ledger(env: &Env, delta: u32) {
    env.ledger().with_mut(|li| {
        li.sequence_number += delta;
    });
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    assert_eq!(token.initialized(), true);
    assert_eq!(token.admin(), admin);
    assert_eq!(token.decimals(), 7);
    assert_eq!(token.name(), String::from_str(&env, "Test Token"));
    assert_eq!(token.symbol(), String::from_str(&env, "TEST"));
}

#[test]
#[should_panic(expected = "token already initialized")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    // Should panic on second initialization
    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token 2"),
        &String::from_str(&env, "TEST2"),
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);
}

#[test]
#[should_panic(expected = "negative amount")]
fn test_mint_negative() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user, &(-100));
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);
    token.transfer(user1.clone(), user2.clone(), 300);

    assert_eq!(token.balance(user1), 700);
    assert_eq!(token.balance(user2), 300);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 100);
    token.transfer(user1, user2, 200); // Should panic
}

#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let spender = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);
    
    let expiration = env.ledger().sequence() + 100;
    token.approve(user1.clone(), spender.clone(), 300, expiration);
    
    assert_eq!(token.allowance(user1.clone(), spender.clone()), 300);
    
    token.transfer_from(spender.clone(), user1.clone(), user2.clone(), 200);
    
    assert_eq!(token.balance(user1), 800);
    assert_eq!(token.balance(user2), 200);
    assert_eq!(token.allowance(user1, spender), 100);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let spender = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);
    
    let expiration = env.ledger().sequence() + 100;
    token.approve(user1.clone(), spender.clone(), 100, expiration);
    
    token.transfer_from(spender, user1, user2, 200); // Should panic
}

#[test]
fn test_allowance_expiration() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let spender = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);
    
    let expiration = env.ledger().sequence() + 10;
    token.approve(user1.clone(), spender.clone(), 300, expiration);
    
    assert_eq!(token.allowance(user1.clone(), spender.clone()), 300);
    
    // Advance ledger past expiration
    advance_ledger(&env, 15);
    
    assert_eq!(token.allowance(user1, spender), 0);
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user.clone(), 1000);
    token.burn(user.clone(), 300);

    assert_eq!(token.balance(user), 700);
}

#[test]
fn test_burn_from() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let spender = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user.clone(), 1000);
    
    let expiration = env.ledger().sequence() + 100;
    token.approve(user.clone(), spender.clone(), 300, expiration);
    
    token.burn_from(spender.clone(), user.clone(), 200);
    
    assert_eq!(token.balance(user), 800);
    assert_eq!(token.allowance(user, spender), 100);
}

#[test]
fn test_set_admin() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    assert_eq!(token.admin(), admin);
    
    token.set_admin(new_admin.clone());
    
    assert_eq!(token.admin(), new_admin);
}

#[test]
#[should_panic(expected = "token not initialized")]
fn test_operations_before_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let user = Address::generate(&env);
    let token = create_token_contract(&env);

    // Should panic when trying to get balance before initialization
    token.balance(user);
}

#[test]
#[should_panic(expected = "decimal must be <= 18")]
fn test_initialize_invalid_decimals() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin,
        19, // Invalid: > 18
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );
}

#[test]
#[should_panic(expected = "name must be 1-32 characters")]
fn test_initialize_empty_name() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin,
        7,
        String::from_str(&env, ""), // Empty name
        String::from_str(&env, "TEST"),
    );
}

#[test]
#[should_panic(expected = "symbol must be 1-12 characters")]
fn test_initialize_empty_symbol() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin,
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, ""), // Empty symbol
    );
}

#[test]
fn test_zero_amount_operations() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);

    // Zero transfers should be no-ops
    token.transfer(user1.clone(), user2.clone(), 0);
    assert_eq!(token.balance(user1), 1000);
    assert_eq!(token.balance(user2), 0);

    // Zero burns should be no-ops
    token.burn(user1.clone(), 0);
    assert_eq!(token.balance(user1), 1000);

    // Zero mints should be no-ops
    token.mint(user1.clone(), 0);
    assert_eq!(token.balance(user1), 1000);
}

#[test]
#[should_panic(expected = "expiration_ledger is in the past")]
fn test_approve_past_expiration() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let spender = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);

    // Try to approve with past expiration
    let past_expiration = env.ledger().sequence() - 1;
    token.approve(user1, spender, 100, past_expiration);
}

#[test]
fn test_approve_zero_amount_past_expiration() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let spender = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        admin.clone(),
        7,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
    );

    token.mint(user1.clone(), 1000);

    // Approve zero amount with past expiration should work (for clearing allowances)
    let past_expiration = env.ledger().sequence() - 1;
    token.approve(user1.clone(), spender.clone(), 0, past_expiration);

    assert_eq!(token.allowance(user1, spender), 0);
}
