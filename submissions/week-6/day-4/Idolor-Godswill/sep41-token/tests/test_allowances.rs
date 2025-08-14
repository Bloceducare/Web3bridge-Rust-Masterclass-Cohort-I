#![cfg(test)]

use sep41_token::{Token, TokenInterface, TokenAdminInterface};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_token_with_balance(env: &Env, owner: &Address, balance: i128) -> Address {
    let admin = Address::generate(env);
    
    Token::initialize(
        env.clone(),
        admin.clone(),
        String::from_str(env, "Test Token"),
        String::from_str(env, "TEST"),
        18,
    );
    
    Token::mint(env.clone(), owner.clone(), balance);
    admin
}

#[test]
fn test_approve_and_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let allowance_amount = 500i128;
    let expiration = env.ledger().sequence() + 100;

    setup_token_with_balance(&env, &owner, 1000);

    // Initially no allowance
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), 0);

    // Approve allowance
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // Check allowance
    assert_eq!(Token::allowance(env, owner, spender), allowance_amount);
}

#[test]
fn test_transfer_from_with_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let initial_balance = 1000i128;
    let allowance_amount = 500i128;
    let transfer_amount = 300i128;
    let expiration = env.ledger().sequence() + 100;

    setup_token_with_balance(&env, &owner, initial_balance);

    // Approve allowance
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // Transfer from owner to recipient via spender
    Token::transfer_from(env.clone(), spender.clone(), owner.clone(), recipient.clone(), transfer_amount);

    // Check balances
    assert_eq!(Token::balance(env.clone(), owner), initial_balance - transfer_amount);
    assert_eq!(Token::balance(env.clone(), recipient), transfer_amount);
    
    // Check remaining allowance
    assert_eq!(Token::allowance(env, owner, spender), allowance_amount - transfer_amount);
}

#[test]
#[should_panic(expected = "InsufficientAllowance")]
fn test_transfer_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let allowance_amount = 200i128;
    let transfer_amount = 300i128; // More than allowance
    let expiration = env.ledger().sequence() + 100;

    setup_token_with_balance(&env, &owner, 1000);

    // Approve smaller allowance
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // This should fail
    Token::transfer_from(env, spender, owner, recipient, transfer_amount);
}

#[test]
#[should_panic(expected = "InsufficientAllowance")]
fn test_transfer_from_no_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);

    setup_token_with_balance(&env, &owner, 1000);

    // Try to transfer without any allowance - should fail
    Token::transfer_from(env, spender, owner, recipient, 100);
}

#[test]
fn test_burn_from_with_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let initial_balance = 1000i128;
    let allowance_amount = 500i128;
    let burn_amount = 300i128;
    let expiration = env.ledger().sequence() + 100;

    setup_token_with_balance(&env, &owner, initial_balance);

    // Approve allowance
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // Burn from owner via spender
    Token::burn_from(env.clone(), spender.clone(), owner.clone(), burn_amount);

    // Check balance
    assert_eq!(Token::balance(env.clone(), owner), initial_balance - burn_amount);
    
    // Check remaining allowance
    assert_eq!(Token::allowance(env, owner, spender), allowance_amount - burn_amount);
}

#[test]
#[should_panic(expected = "InsufficientAllowance")]
fn test_burn_from_insufficient_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let allowance_amount = 200i128;
    let burn_amount = 300i128; // More than allowance
    let expiration = env.ledger().sequence() + 100;

    setup_token_with_balance(&env, &owner, 1000);

    // Approve smaller allowance
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // This should fail
    Token::burn_from(env, spender, owner, burn_amount);
}

#[test]
fn test_allowance_expiration() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let allowance_amount = 500i128;
    let current_ledger = env.ledger().sequence();
    let expiration = current_ledger + 1; // Expires soon

    setup_token_with_balance(&env, &owner, 1000);

    // Approve with short expiration
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // Check allowance is active
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), allowance_amount);

    // Advance ledger past expiration
    env.ledger().with_mut(|li| {
        li.sequence_number = expiration + 1;
    });

    // Allowance should now be 0 (expired)
    assert_eq!(Token::allowance(env, owner, spender), 0);
}

#[test]
#[should_panic(expected = "AllowanceExpired")]
fn test_transfer_from_expired_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let recipient = Address::generate(&env);
    let allowance_amount = 500i128;
    let current_ledger = env.ledger().sequence();
    let expiration = current_ledger + 1;

    setup_token_with_balance(&env, &owner, 1000);

    // Approve with short expiration
    Token::approve(env.clone(), owner.clone(), spender.clone(), allowance_amount, expiration);

    // Advance ledger past expiration
    env.ledger().with_mut(|li| {
        li.sequence_number = expiration + 1;
    });

    // This should fail with expired allowance
    Token::transfer_from(env, spender, owner, recipient, 100);
}

#[test]
#[should_panic(expected = "InvalidExpiration")]
fn test_approve_past_expiration() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let allowance_amount = 500i128;
    let current_ledger = env.ledger().sequence();
    let past_expiration = current_ledger - 1; // Already expired

    setup_token_with_balance(&env, &owner, 1000);

    // This should fail - can't approve with past expiration
    Token::approve(env, owner, spender, allowance_amount, past_expiration);
}

#[test]
fn test_approve_zero_amount_with_past_expiration() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let current_ledger = env.ledger().sequence();
    let past_expiration = current_ledger - 1;

    setup_token_with_balance(&env, &owner, 1000);

    // Approving 0 amount with past expiration should work (clearing allowance)
    Token::approve(env.clone(), owner.clone(), spender.clone(), 0, past_expiration);
    
    assert_eq!(Token::allowance(env, owner, spender), 0);
}

#[test]
fn test_overwrite_allowance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let owner = Address::generate(&env);
    let spender = Address::generate(&env);
    let first_allowance = 300i128;
    let second_allowance = 700i128;
    let expiration = env.ledger().sequence() + 100;

    setup_token_with_balance(&env, &owner, 1000);

    // First approval
    Token::approve(env.clone(), owner.clone(), spender.clone(), first_allowance, expiration);
    assert_eq!(Token::allowance(env.clone(), owner.clone(), spender.clone()), first_allowance);

    // Second approval should overwrite
    Token::approve(env.clone(), owner.clone(), spender.clone(), second_allowance, expiration);
    assert_eq!(Token::allowance(env, owner, spender), second_allowance);
}
