#![cfg(test)]

use sep41_token::{Token, TokenInterface, TokenAdminInterface};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_initialized_token(env: &Env) -> Address {
    let admin = Address::generate(env);
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(env, "Test Token"),
        String::from_str(env, "TEST"),
        18,
    );
    admin
}

#[test]
fn test_self_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user = Address::generate(&env);
    let amount = 1000i128;

    // Mint tokens to user
    Token::mint(env.clone(), user.clone(), amount);

    // Transfer to self
    Token::transfer(env.clone(), user.clone(), user.clone(), 500);

    // Balance should remain the same
    assert_eq!(Token::balance(env, user), amount);
}

#[test]
fn test_self_approval() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user = Address::generate(&env);
    let amount = 1000i128;
    let expiration = env.ledger().sequence() + 100;

    // Mint tokens to user
    Token::mint(env.clone(), user.clone(), amount);

    // Approve self
    Token::approve(env.clone(), user.clone(), user.clone(), 500, expiration);

    // Should be able to query self-allowance
    assert_eq!(Token::allowance(env, user.clone(), user), 500);
}

#[test]
fn test_transfer_from_self_to_self() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user = Address::generate(&env);
    let amount = 1000i128;
    let expiration = env.ledger().sequence() + 100;

    // Setup
    Token::mint(env.clone(), user.clone(), amount);
    Token::approve(env.clone(), user.clone(), user.clone(), 500, expiration);

    // Transfer from self to self
    Token::transfer_from(env.clone(), user.clone(), user.clone(), user.clone(), 300);

    // Balance should remain the same, allowance should be reduced
    assert_eq!(Token::balance(env.clone(), user), amount);
    assert_eq!(Token::allowance(env, user.clone(), user), 200);
}

#[test]
fn test_multiple_approvals_same_spender() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 100;

    Token::mint(env.clone(), owner.clone(), 1000);

    // Multiple approvals should overwrite
    Token::approve(env.clone(), owner.clone(), spender.clone(), 100, expiration);
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), 100);

    Token::approve(env.clone(), owner.clone(), spender.clone(), 200, expiration);
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), 200);

    Token::approve(env.clone(), owner.clone(), spender.clone(), 50, expiration);
    assert_eq!(Token::allowance(env, owner, spender), 50);
}

#[test]
fn test_approve_zero_clears_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let expiration = env.ledger().sequence() + 100;

    Token::mint(env.clone(), owner.clone(), 1000);

    // Set allowance
    Token::approve(env.clone(), owner.clone(), spender.clone(), 500, expiration);
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), 500);

    // Clear allowance with zero
    Token::approve(env.clone(), owner.clone(), spender.clone(), 0, expiration);
    assert_eq!(Token::allowance(env, owner, spender), 0);
}

#[test]
fn test_burn_all_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user = Address::generate(&env);
    let amount = 1000i128;

    // Mint and burn all tokens
    Token::mint(env.clone(), user.clone(), amount);
    Token::burn(env.clone(), user.clone(), amount);

    // Balance should be zero
    assert_eq!(Token::balance(env, user), 0);
}

#[test]
fn test_transfer_all_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let amount = 1000i128;

    // Mint and transfer all tokens
    Token::mint(env.clone(), sender.clone(), amount);
    Token::transfer(env.clone(), sender.clone(), recipient.clone(), amount);

    // Check balances
    assert_eq!(Token::balance(env.clone(), sender), 0);
    assert_eq!(Token::balance(env, recipient), amount);
}

#[test]
fn test_complex_transfer_chain() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let initial_amount = 1000i128;

    // Setup: user1 has all tokens
    Token::mint(env.clone(), user1.clone(), initial_amount);

    // Chain: user1 -> user2 -> user3
    Token::transfer(env.clone(), user1.clone(), user2.clone(), 600);
    Token::transfer(env.clone(), user2.clone(), user3.clone(), 300);

    // Check final balances
    assert_eq!(Token::balance(env.clone(), user1), 400);
    assert_eq!(Token::balance(env.clone(), user2), 300);
    assert_eq!(Token::balance(env, user3), 300);
}

#[test]
fn test_allowance_edge_at_expiration() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let current_ledger = env.ledger().sequence();
    let expiration = current_ledger + 1;

    Token::mint(env.clone(), owner.clone(), 1000);

    // Set allowance that expires next ledger
    Token::approve(env.clone(), owner.clone(), spender.clone(), 500, expiration);

    // Should work at current ledger
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), 500);

    // Advance to expiration ledger (should still work)
    env.ledger().with_mut(|li| {
        li.sequence_number = expiration;
    });
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), 500);

    // Advance past expiration (should be 0)
    env.ledger().with_mut(|li| {
        li.sequence_number = expiration + 1;
    });
    assert_eq!(Token::allowance(env, owner, spender), 0);
}

#[test]
fn test_maximum_values() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user = Address::generate(&env);
    let max_safe = i128::MAX / 2; // Safe maximum to avoid overflow

    // Should handle large values
    Token::mint(env.clone(), user.clone(), max_safe);
    assert_eq!(Token::balance(env, user), max_safe);
}

#[test]
fn test_precision_with_decimals() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Initialize with high precision
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(&env, "High Precision Token"),
        String::from_str(&env, "HPT"),
        18,
    );

    // Mint 1 token with 18 decimals (1 * 10^18)
    let one_token = 1_000_000_000_000_000_000i128;
    Token::mint(env.clone(), user.clone(), one_token);

    // Should handle precise amounts
    assert_eq!(Token::balance(env, user), one_token);
}

#[test]
fn test_concurrent_operations_simulation() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = setup_initialized_token(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);

    // Simulate concurrent operations
    Token::mint(env.clone(), user1.clone(), 1000);
    Token::mint(env.clone(), user2.clone(), 1000);
    
    // Multiple transfers
    Token::transfer(env.clone(), user1.clone(), user3.clone(), 100);
    Token::transfer(env.clone(), user2.clone(), user3.clone(), 200);
    Token::transfer(env.clone(), user3.clone(), user1.clone(), 50);

    // Check final state
    assert_eq!(Token::balance(env.clone(), user1), 950);  // 1000 - 100 + 50
    assert_eq!(Token::balance(env.clone(), user2), 800);  // 1000 - 200
    assert_eq!(Token::balance(env, user3), 250);          // 100 + 200 - 50
}
