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

#[test]
fn test_metadata() {
    let (_env, _contract_id, client) = create_contract();

    // Test token metadata
    assert_eq!(client.name(), String::from_str(&_env, "RUG PULL"));
    assert_eq!(client.symbol(), String::from_str(&_env, "RGP"));
    assert_eq!(client.decimals(), 18);
}

#[test]
fn test_initial_balance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, _user2, _user3) = create_users(&env);

    // Initially all balances should be 0
    assert_eq!(client.balance(&user1), 0);
}

#[test]
fn test_initial_allowance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    // Initially all allowances should be 0
    assert_eq!(client.allowance(&user1, &user2), 0);
}

#[test]
fn test_approve_and_allowance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Approve user2 to spend 100 tokens from user1
    client.approve(&user1, &user2, &100, &1000);

    // Check allowance
    assert_eq!(client.allowance(&user1, &user2), 100);
}

#[test]
fn test_transfer_with_sufficient_balance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // First, we need to give user1 some balance by setting it directly in storage
    // Since there's no mint function, we'll simulate having a balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    // Transfer 100 tokens from user1 to user2
    client.transfer(&user1, &user2, &100);

    // Check balances
    assert_eq!(client.balance(&user1), 900);
    assert_eq!(client.balance(&user2), 100);
}

#[test]
#[should_panic(expected = "Insufficient Balance")]
fn test_transfer_insufficient_balance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Try to transfer without sufficient balance
    client.transfer(&user1, &user2, &100);
}

#[test]
fn test_transfer_from_with_allowance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Give user1 some balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

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
#[should_panic(expected = "Insufficient Allowance")]
fn test_transfer_from_insufficient_allowance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Give user1 some balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    // Try to transfer_from without sufficient allowance
    client.transfer_from(&user2, &user1, &user3, &100);
}

#[test]
#[should_panic(expected = "Insufficient Balance")]
fn test_transfer_from_insufficient_balance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Approve user2 to spend 200 tokens from user1 (but user1 has no balance)
    client.approve(&user1, &user2, &200, &1000);

    // Try to transfer_from without sufficient balance
    client.transfer_from(&user2, &user1, &user3, &100);
}

#[test]
fn test_burn() {
    let (env, _contract_id, client) = create_contract();
    let (user1, _user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Give user1 some balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    // Burn 100 tokens from user1
    client.burn(&user1, &100);

    // Check remaining balance
    assert_eq!(client.balance(&user1), 900);
}

#[test]
#[should_panic(expected = "Insufficient Balance")]
fn test_burn_insufficient_balance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, _user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Try to burn without sufficient balance
    client.burn(&user1, &100);
}

#[test]
fn test_burn_from_with_allowance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Give user1 some balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    // Approve user2 to spend 200 tokens from user1
    client.approve(&user1, &user2, &200, &1000);

    // User2 burns 100 tokens from user1
    client.burn_from(&user2, &user1, &100);

    // Check remaining balance and allowance
    assert_eq!(client.balance(&user1), 900);
    assert_eq!(client.allowance(&user1, &user2), 100); // 200 - 100 = 100
}

#[test]
#[should_panic(expected = "Insufficient Allowance")]
fn test_burn_from_insufficient_allowance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Give user1 some balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    // Try to burn_from without sufficient allowance
    client.burn_from(&user2, &user1, &100);
}

#[test]
#[should_panic(expected = "Insufficient Balance")]
fn test_burn_from_insufficient_balance() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Approve user2 to spend 200 tokens from user1 (but user1 has no balance)
    client.approve(&user1, &user2, &200, &1000);

    // Try to burn_from without sufficient balance
    client.burn_from(&user2, &user1, &100);
}

#[test]
fn test_multiple_operations() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, user3) = create_users(&env);

    env.mock_all_auths();

    // Give user1 initial balance
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    // Approve user2 to spend 500 tokens from user1
    client.approve(&user1, &user2, &500, &1000);

    // Transfer 100 tokens from user1 to user2 directly
    client.transfer(&user1, &user2, &100);

    // User2 transfers 50 tokens from user1 to user3 using allowance
    client.transfer_from(&user2, &user1, &user3, &50);

    // User2 burns 25 tokens from user1 using allowance
    client.burn_from(&user2, &user1, &25);

    // Check final balances and allowances
    assert_eq!(client.balance(&user1), 825); // 1000 - 100 - 50 - 25
    assert_eq!(client.balance(&user2), 100);
    assert_eq!(client.balance(&user3), 50);
    assert_eq!(client.allowance(&user1, &user2), 425); // 500 - 50 - 25
}

#[test]
fn test_zero_amounts() {
    let (env, _contract_id, client) = create_contract();
    let (user1, user2, _user3) = create_users(&env);

    env.mock_all_auths();

    // Test zero approve
    client.approve(&user1, &user2, &0, &1000);
    assert_eq!(client.allowance(&user1, &user2), 0);

    // Give user1 some balance for zero transfer tests
    let balance_key = DataKey::Balance(user1.clone());
    env.as_contract(&_contract_id, || {
        env.storage().persistent().set(&balance_key, &1000i128);
    });

    let initial_balance = client.balance(&user1);

    // Test zero transfer
    client.transfer(&user1, &user2, &0);
    assert_eq!(client.balance(&user1), initial_balance);
    assert_eq!(client.balance(&user2), 0);

    // Test zero burn
    client.burn(&user1, &0);
    assert_eq!(client.balance(&user1), initial_balance);
}
