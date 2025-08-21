#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn create_contract() -> (Env, Address, ContractClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    (env, contract_id, client)
}

fn create_users(env: &Env) -> (Address, Address, Address) {
    let user1 = Address::generate(env);
    let user2 = Address::generate(env);
    let user3 = Address::generate(env);
    (user1, user2, user3)
}

fn create_initialized_contract() -> (Env, Address, ContractClient<'static>, Address) {
    let (env, contract_id, client) = create_contract();
    let admin = Address::generate(&env);

    env.mock_all_auths();
    client.init(&admin);

    (env, contract_id, client, admin)
}

#[test]
fn test_initialization() {
    let (_env, _contract_id, client, admin) = create_initialized_contract();

    // Check that the contract is initialized
    assert!(client.check_is_initialized());

    // Admin should have initial supply
    assert_eq!(client.balance(&admin), 1_000_000_000);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_double_initialization() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let another_admin = Address::generate(&env);

    env.mock_all_auths();

    // Try to initialize again - should panic
    client.init(&another_admin);
}

#[test]
fn test_metadata() {
    let (_env, _contract_id, client, _admin) = create_initialized_contract();

    // Test token metadata
    assert_eq!(client.name(), String::from_str(&_env, "RUG PULL"));
    assert_eq!(client.symbol(), String::from_str(&_env, "RGP"));
    assert_eq!(client.decimals(), 18);
}

#[test]
fn test_initial_balance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, _user2, _user3) = create_users(&env);

    // Non-admin users should have 0 balance initially
    assert_eq!(client.balance(&user1), 0);
}

#[test]
fn test_initial_allowance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    // Initially all allowances should be 0
    assert_eq!(client.allowance(&user1, &user2), 0);
}

#[test]
fn test_approve_and_allowance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Approve user2 to spend 100 tokens from user1
    client.approve(&user1, &user2, &100, &1000);

    // Check allowance
    assert_eq!(client.allowance(&user1, &user2), 100);
}

#[test]
fn test_transfer_with_sufficient_balance() {
    let (env, _contract_id, client, admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Transfer from admin to user1 first
    client.transfer(&admin, &user1, &1000);

    // Transfer 100 tokens from user1 to user2
    client.transfer(&user1, &user2, &100);

    // Check balances
    assert_eq!(client.balance(&user1), 900);
    assert_eq!(client.balance(&user2), 100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_transfer_insufficient_balance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Try to transfer without sufficient balance
    client.transfer(&user1, &user2, &100);
}

#[test]
fn test_transfer_from_with_allowance() {
    let (env, _contract_id, client, admin) = create_initialized_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Transfer from admin to user1 first
    client.transfer(&admin, &user1, &1000);

    // Approve user2 to spend 200 tokens from user1
    client.approve(&user1, &user2, &200, &1000);

    // User2 transfers 100 tokens from user1 to user3
    client.transfer_from(&user2, &user1, &user3, &100);

    // Check balances and remaining allowance
    assert_eq!(client.balance(&user1), 900);
    assert_eq!(client.balance(&user3), 100);
    assert_eq!(client.allowance(&user1, &user2), 100); // 200 - 100 = 100
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_transfer_from_insufficient_allowance() {
    let (env, _contract_id, client, admin) = create_initialized_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Transfer from admin to user1 first
    client.transfer(&admin, &user1, &1000);

    // Try to transfer_from without sufficient allowance
    client.transfer_from(&user2, &user1, &user3, &100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_transfer_from_insufficient_balance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Approve user2 to spend 200 tokens from user1 (but user1 has no balance)
    client.approve(&user1, &user2, &200, &1000);

    // Try to transfer_from without sufficient balance
    client.transfer_from(&user2, &user1, &user3, &100);
}

#[test]
fn test_burn() {
    let (env, _contract_id, client, admin) = create_initialized_contract();
    let (user1, _user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Transfer from admin to user1 first
    client.transfer(&admin, &user1, &1000);

    // Burn 100 tokens from user1
    client.burn(&user1, &100);

    // Check remaining balance
    assert_eq!(client.balance(&user1), 900);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_burn_insufficient_balance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, _user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Try to burn without sufficient balance
    client.burn(&user1, &100);
}

#[test]
fn test_burn_from_with_allowance() {
    let (env, _contract_id, client, admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Transfer from admin to user1 first
    client.transfer(&admin, &user1, &1000);

    // Approve user2 to spend 200 tokens from user1
    client.approve(&user1, &user2, &200, &1000);

    // User2 burns 100 tokens from user1
    client.burn_from(&user2, &user1, &100);

    // Check remaining balance and allowance
    assert_eq!(client.balance(&user1), 900);
    assert_eq!(client.allowance(&user1, &user2), 100); // 200 - 100 = 100
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_burn_from_insufficient_allowance() {
    let (env, _contract_id, client, admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Transfer from admin to user1 first
    client.transfer(&admin, &user1, &1000);

    // Try to burn_from without sufficient allowance
    client.burn_from(&user2, &user1, &100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_burn_from_insufficient_balance() {
    let (env, _contract_id, client, _admin) = create_initialized_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Approve user2 to spend 200 tokens from user1 (but user1 has no balance)
    client.approve(&user1, &user2, &200, &1000);

    // Try to burn_from without sufficient balance
    client.burn_from(&user2, &user1, &100);
}
