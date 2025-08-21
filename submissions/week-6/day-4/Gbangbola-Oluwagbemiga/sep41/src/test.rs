#![cfg(test)]

use super::*;
use crate::contract::{Token, TokenClient};
use soroban_sdk::{testutils::Address as _, vec, Address, Env, String};

fn setup() -> (Env, TokenClient<'static>, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);

    let contract_id = env.register(
        Token,
        (
            admin.clone(),
            18u32,
            String::from_str(&env, "Test Token"),
            String::from_str(&env, "TEST"),
        ),
    );

    let client = TokenClient::new(&env, &contract_id);
    (env, client, admin)
}

#[test]
fn test_token_creation() {
    let (env, client, admin) = setup();

    let name = client.name();
    assert_eq!(name, String::from_str(&env, "Test Token"));

    let symbol = client.symbol();
    assert_eq!(symbol, String::from_str(&env, "TEST"));

    let decimals = client.decimals();
    assert_eq!(decimals, 18u32);

    let stored_admin = client.admin();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_minting() {
    let (env, client, admin) = setup();

    let user = Address::generate(&env);
    let mint_amount = 1000i128;

    env.mock_all_auths();
    client.mint(&user, &mint_amount);

    let user_balance = client.balance(&user);
    assert_eq!(user_balance, mint_amount);

    let admin_balance = client.balance(&admin);
    assert_eq!(admin_balance, 0i128);
}

#[test]
fn test_transfer() {
    let (env, client, admin) = setup();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    env.mock_all_auths();
    let initial_amount = 1000i128;
    client.mint(&alice, &initial_amount);

    let transfer_amount = 300i128;
    client.transfer(&alice, &bob, &transfer_amount);

    let alice_balance = client.balance(&alice);
    let bob_balance = client.balance(&bob);

    assert_eq!(alice_balance, 700i128);
    assert_eq!(bob_balance, 300i128);
}

#[test]
#[should_panic]
fn test_insufficient_balance_transfer() {
    let (env, client, admin) = setup();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    env.mock_all_auths();
    client.mint(&alice, &50i128);

    client.transfer(&alice, &bob, &100i128);
}

#[test]
fn test_approve_and_transfer_from() {
    let (env, client, admin) = setup();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    env.mock_all_auths();
    client.mint(&alice, &1000i128);

    client.approve(&alice, &bob, &500i128, &1000u32);

    let allowance = client.allowance(&alice, &bob);
    assert_eq!(allowance, 500i128);

    client.transfer_from(&bob, &alice, &charlie, &200i128);

    assert_eq!(client.balance(&alice), 800i128);
    assert_eq!(client.balance(&charlie), 200i128);

    let remaining_allowance = client.allowance(&alice, &bob);
    assert_eq!(remaining_allowance, 300i128);
}

#[test]
fn test_burn() {
    let (env, client, admin) = setup();

    let alice = Address::generate(&env);

    env.mock_all_auths();
    client.mint(&alice, &1000i128);
    client.burn(&alice, &300i128);

    let alice_balance = client.balance(&alice);
    assert_eq!(alice_balance, 700i128);
}

#[test]
fn test_admin_functions() {
    let (env, client, admin) = setup();

    let new_admin = Address::generate(&env);

    env.mock_all_auths();
    client.set_admin(&new_admin);

    let stored_admin = client.admin();
    assert_eq!(stored_admin, new_admin);
}

#[test]
#[should_panic]
fn test_negative_amount_rejected() {
    let (env, client, admin) = setup();

    let user = Address::generate(&env);

    env.mock_all_auths();
    client.mint(&user, &(-100i128));
}

#[test]
fn test_zero_balance_for_new_accounts() {
    let (_, client, admin) = setup();

    let random_user = Address::generate(&Env::default());
    let balance = client.balance(&random_user);
    assert_eq!(balance, 0i128);
}

#[test]
fn test_complete_token_lifecycle() {
    let (env, client, admin) = setup();

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);
    let charlie = Address::generate(&env);

    env.mock_all_auths();

    client.mint(&alice, &1000i128);
    client.transfer(&alice, &bob, &300i128);
    client.approve(&alice, &charlie, &200i128, &1000u32);
    client.transfer_from(&charlie, &alice, &bob, &100i128);
    client.burn(&bob, &50i128);

    assert_eq!(client.balance(&alice), 600i128);
    assert_eq!(client.balance(&bob), 350i128);
    assert_eq!(client.balance(&charlie), 0i128);
    assert_eq!(client.allowance(&alice, &charlie), 100i128);
}
