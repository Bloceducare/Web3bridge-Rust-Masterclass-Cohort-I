#![cfg(test)]

use crate::Token;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

fn create_token_contract(env: &Env) -> TokenClient<'_> {
    let contract_address = env.register(Token {}, ());
    TokenClient::new(env, &contract_address)
}

// Import the auto-generated client
use crate::contract::TokenClient;

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
fn test_mint_and_balance() {
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
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user1, &1000);
    token.transfer(&user1, &user2, &300);

    assert_eq!(token.balance(&user1), 700);
    assert_eq!(token.balance(&user2), 300);
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
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user1, &1000);
    
    let expiration = env.ledger().sequence() + 100;
    token.approve(&user1, &spender, &300, &expiration);
    
    assert_eq!(token.allowance(&user1, &spender), 300);
    
    token.transfer_from(&spender, &user1, &user2, &200);
    
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);
    assert_eq!(token.allowance(&user1, &spender), 100);
}

#[test]
fn test_burn() {
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
    token.burn(&user, &300);

    assert_eq!(token.balance(&user), 700);
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
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user1, &1000);
    
    let expiration = env.ledger().sequence() + 10;
    token.approve(&user1, &spender, &300, &expiration);
    
    assert_eq!(token.allowance(&user1, &spender), 300);
    
    // Advance ledger past expiration
    advance_ledger(&env, 15);
    
    assert_eq!(token.allowance(&user1, &spender), 0);
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
#[should_panic(expected = "insufficient balance")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let token = create_token_contract(&env);

    token.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user1, &100);
    token.transfer(&user1, &user2, &200); // Should panic
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
        &admin,
        &7,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
    );

    token.mint(&user1, &1000);
    
    let expiration = env.ledger().sequence() + 100;
    token.approve(&user1, &spender, &100, &expiration);
    
    token.transfer_from(&spender, &user1, &user2, &200); // Should panic
}
