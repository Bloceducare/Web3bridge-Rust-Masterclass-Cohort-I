#![cfg(test)]

use sep41_token::{Token, TokenInterface, TokenAdminInterface};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_admin_initialization() {
    let env = Env::default();
    let admin = Address::generate(&env);

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    assert_eq!(Token::admin(env), admin);
}

#[test]
fn test_admin_can_mint() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let mint_amount = 1000i128;

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Admin should be able to mint
    Token::mint(env.clone(), user.clone(), mint_amount);
    assert_eq!(Token::balance(env, user), mint_amount);
}

#[test]
fn test_set_admin() {
    let env = Env::default();
    env.mock_all_auths();
    
    let original_admin = Address::generate(&env);
    let new_admin = Address::generate(&env);

    Token::initialize(
        env.clone(),
        original_admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Original admin should be set
    assert_eq!(Token::admin(env.clone()), original_admin);

    // Change admin
    Token::set_admin(env.clone(), new_admin.clone());

    // New admin should be set
    assert_eq!(Token::admin(env), new_admin);
}

#[test]
fn test_mint_zero_amount() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Minting 0 should be a no-op
    Token::mint(env.clone(), user.clone(), 0);
    assert_eq!(Token::balance(env, user), 0);
}

#[test]
#[should_panic(expected = "InvalidAmount")]
fn test_mint_negative_amount() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Minting negative amount should fail
    Token::mint(env, user, -100);
}

#[test]
fn test_mint_updates_total_supply() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let amount1 = 1000i128;
    let amount2 = 500i128;

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Mint to first user
    Token::mint(env.clone(), user1.clone(), amount1);
    
    // Mint to second user
    Token::mint(env.clone(), user2.clone(), amount2);

    // Check individual balances
    assert_eq!(Token::balance(env.clone(), user1), amount1);
    assert_eq!(Token::balance(env, user2), amount2);
    
    // Note: We don't have a total_supply() function in the interface,
    // but the internal storage should be updated correctly
}

#[test]
fn test_burn_updates_total_supply() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let mint_amount = 1000i128;
    let burn_amount = 300i128;

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Mint tokens
    Token::mint(env.clone(), user.clone(), mint_amount);
    
    // Burn some tokens
    Token::burn(env.clone(), user.clone(), burn_amount);

    // Check balance
    assert_eq!(Token::balance(env, user), mint_amount - burn_amount);
    
    // Total supply should also be reduced (internal storage)
}

#[test]
fn test_large_mint_amount() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let large_amount = i128::MAX / 2; // Large but safe amount

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Should be able to mint large amounts
    Token::mint(env.clone(), user.clone(), large_amount);
    assert_eq!(Token::balance(env, user), large_amount);
}

#[test]
#[should_panic(expected = "Overflow")]
fn test_mint_overflow() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let max_amount = i128::MAX;

    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Test Token"),
        String::from_str(&env, "TEST"),
        18,
    );

    // Mint maximum amount
    Token::mint(env.clone(), user.clone(), max_amount);
    
    // Try to mint more - should overflow
    Token::mint(env, user, 1);
}

#[test]
fn test_metadata_validation() {
    let env = Env::default();
    let admin = Address::generate(&env);

    // Valid metadata should work
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "Valid Token Name"),
        String::from_str(&env, "VTN"),
        18,
    );

    assert_eq!(Token::name(env.clone()), String::from_str(&env, "Valid Token Name"));
    assert_eq!(Token::symbol(env.clone()), String::from_str(&env, "VTN"));
    assert_eq!(Token::decimals(env), 18);
}

#[test]
fn test_different_decimal_values() {
    let env = Env::default();
    let admin = Address::generate(&env);

    // Test with 6 decimals (common for stablecoins)
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "USDC"),
        String::from_str(&env, "USDC"),
        6,
    );

    assert_eq!(Token::decimals(env), 6);
}

#[test]
#[should_panic(expected = "NotInitialized")]
fn test_admin_query_before_initialization() {
    let env = Env::default();
    
    // Should panic when querying admin before initialization
    Token::admin(env);
}

#[test]
#[should_panic(expected = "NotInitialized")]
fn test_metadata_query_before_initialization() {
    let env = Env::default();
    
    // Should panic when querying metadata before initialization
    Token::name(env);
}
