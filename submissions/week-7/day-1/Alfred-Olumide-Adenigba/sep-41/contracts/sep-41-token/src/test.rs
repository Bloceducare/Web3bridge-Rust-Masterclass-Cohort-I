#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String, Symbol};

use crate::{TokenContract, TokenContractClient};

fn setup() -> (Env, TokenContractClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    // Register contract
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    // Create test accounts
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);

    client.initialize(&admin, &1000);

    (env, client, admin, user1)
}
#[test]
fn test_name() {
    let (env, client, _admin, _user1) = setup();

    let name = client.name();
    assert_eq!(name, String::from_str(&env, "MyToken"));
}

#[test]
fn test_symbol() {
    let (env, client, _admin, _user1) = setup();

    let symbol = client.symbol();
    assert_eq!(symbol, String::from_str(&env, "MTK"));
}

#[test]
fn test_total_supply() {
    let (env, client, _admin, _user1) = setup();

    let supply = client.total_supply();
    assert_eq!(supply, 1000);
}

#[test]
fn test_transfer() {
    let (env, client, admin, user1) = setup();

    // transfer from admin -> user1
    client.transfer(&admin, &user1, &100);

    assert_eq!(client.balance_of(&admin), 900);
    assert_eq!(client.balance_of(&user1), 100);
}

#[test]
fn test_approve_and_transfer_from() {
    let (env, client, admin, user1) = setup();

    // admin approves user1 to spend 200 tokens
    client.approve(&admin, &user1, &200);

    // check allowance
    let allowance = client.allowance(&admin, &user1);
    assert_eq!(allowance, 200);

    // user1 transfers from admin -> user1
    client.transfer_from(&user1, &admin, &user1, &150);

    assert_eq!(client.balance_of(&admin), 850);
    assert_eq!(client.balance_of(&user1), 150);

    // remaining allowance
    let allowance_after = client.allowance(&admin, &user1);
    assert_eq!(allowance_after, 50);
}

#[test]
fn test_burn() {
    let (env, client, admin, user1) = setup();

    // Transfer some tokens to user1 first
    client.transfer(&admin, &user1, &200);
    assert_eq!(client.balance_of(&user1), 200);
    assert_eq!(client.total_supply(), 1000);

    // User1 burns 50 tokens
    client.burn(&user1, &50);

    assert_eq!(client.balance_of(&user1), 150);
    assert_eq!(client.total_supply(), 950);
}

#[test]
fn test_burn_from() {
    let (env, client, admin, user1) = setup();

    // Transfer some tokens to user1
    client.transfer(&admin, &user1, &300);
    
    // User1 approves admin to burn 100 tokens on their behalf
    client.approve(&user1, &admin, &100);
    
    // Check initial state
    assert_eq!(client.balance_of(&user1), 300);
    assert_eq!(client.total_supply(), 1000);
    assert_eq!(client.allowance(&user1, &admin), 100);

    // Admin burns 80 tokens from user1's balance
    client.burn_from(&admin, &user1, &80);

    assert_eq!(client.balance_of(&user1), 220);
    assert_eq!(client.total_supply(), 920);
    assert_eq!(client.allowance(&user1, &admin), 20);
}
