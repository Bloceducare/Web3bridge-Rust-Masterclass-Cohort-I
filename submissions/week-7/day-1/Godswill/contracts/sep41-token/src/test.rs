#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    assert_eq!(token.name(), String::from_str(&env, "Test Token"));
    assert_eq!(token.symbol(), String::from_str(&env, "TEST"));
    assert_eq!(token.decimals(), 18);
}

#[test]
fn test_mint_and_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    env.mock_all_auths();
    
    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    env.mock_all_auths();
    
    token.mint(&user1, &1000);
    token.transfer(&user1, &user2, &500);
    
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user2), 500);
}

#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let spender = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    env.mock_all_auths();
    
    token.mint(&user1, &1000);
    token.approve(&user1, &spender, &500, &(env.ledger().sequence() + 100));
    
    assert_eq!(token.allowance(&user1, &spender), 500);
    
    token.transfer_from(&spender, &user1, &user2, &200);
    
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);
    assert_eq!(token.allowance(&user1, &spender), 300);
}

#[test]
fn test_burn() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    env.mock_all_auths();
    
    token.mint(&user1, &1000);
    token.burn(&user1, &300);
    
    assert_eq!(token.balance(&user1), 700);
}

#[test]
#[should_panic(expected = "negative amount is not allowed")]
fn test_negative_amount() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    env.mock_all_auths();
    token.mint(&user1, &-100);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Token);
    let token = TokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    token.initialize(
        &admin,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TEST"),
        &18,
    );

    env.mock_all_auths();
    
    token.mint(&user1, &100);
    token.transfer(&user1, &user2, &200);
}