#![cfg(test)]

use sep41_token::{Token, TokenInterface, TokenAdminInterface};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_token_initialization() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Test Token");
    let symbol = String::from_str(&env, "TEST");
    let decimals = 18u32;

    // Initialize the token
    Token::initialize(env.clone(), admin.clone(), name.clone(), symbol.clone(), decimals);

    // Verify metadata
    assert_eq!(Token::name(env.clone()), name);
    assert_eq!(Token::symbol(env.clone()), symbol);
    assert_eq!(Token::decimals(env.clone()), decimals);
    assert_eq!(Token::admin(env), admin);
}

#[test]
#[should_panic(expected = "AlreadyInitialized")]
fn test_double_initialization_fails() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let name = String::from_str(&env, "Test Token");
    let symbol = String::from_str(&env, "TEST");

    // Initialize once
    Token::initialize(env.clone(), admin.clone(), name.clone(), symbol.clone(), 18);

    // Try to initialize again - should panic
    Token::initialize(env, admin, name, symbol, 18);
}

#[test]
fn test_mint_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let mint_amount = 1_000_000i128;

    // Initialize token
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Initial balance should be 0
    assert_eq!(Token::balance(env.clone(), user.clone()), 0);

    // Mint tokens
    Token::mint(env.clone(), user.clone(), mint_amount);

    // Check balance after minting
    assert_eq!(Token::balance(env, user), mint_amount);
}

#[test]
fn test_basic_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let initial_amount = 1000i128;
    let transfer_amount = 300i128;

    // Setup
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );
    Token::mint(env.clone(), sender.clone(), initial_amount);

    // Transfer
    Token::transfer(env.clone(), sender.clone(), recipient.clone(), transfer_amount);

    // Verify balances
    assert_eq!(Token::balance(env.clone(), sender), initial_amount - transfer_amount);
    assert_eq!(Token::balance(env, recipient), transfer_amount);
}

#[test]
#[should_panic(expected = "InsufficientBalance")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let balance = 100i128;
    let transfer_amount = 200i128; // More than balance

    // Setup
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );
    Token::mint(env.clone(), sender.clone(), balance);

    // This should fail
    Token::transfer(env, sender, recipient, transfer_amount);
}

#[test]
fn test_zero_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let initial_amount = 1000i128;

    // Setup
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );
    Token::mint(env.clone(), sender.clone(), initial_amount);

    // Transfer 0 tokens (should be no-op)
    Token::transfer(env.clone(), sender.clone(), recipient.clone(), 0);

    // Balances should remain unchanged
    assert_eq!(Token::balance(env.clone(), sender), initial_amount);
    assert_eq!(Token::balance(env, recipient), 0);
}

#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_negative_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Setup
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );
    Token::mint(env.clone(), sender.clone(), 1000);

    // This should fail
    Token::transfer(env, sender, recipient, -100);
}

#[test]
fn test_burn_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let initial_amount = 1000i128;
    let burn_amount = 300i128;

    // Setup
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );
    Token::mint(env.clone(), user.clone(), initial_amount);

    // Burn tokens
    Token::burn(env.clone(), user.clone(), burn_amount);

    // Check balance after burning
    assert_eq!(Token::balance(env, user), initial_amount - burn_amount);
}

#[test]
#[should_panic(expected = "InsufficientBalance")]
fn test_burn_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let balance = 100i128;
    let burn_amount = 200i128; // More than balance

    // Setup
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );
    Token::mint(env.clone(), user.clone(), balance);

    // This should fail
    Token::burn(env, user, burn_amount);
}

#[test]
fn test_balance_query_nonexistent_account() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let nonexistent_user = Address::generate(&env);

    // Initialize token
    Token::initialize(
        env.clone(),
        admin,
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Balance of non-existent account should be 0
    assert_eq!(Token::balance(env, nonexistent_user), 0);
}
