#![cfg(test)]

use crate::{SandraToken, SandraTokenClient};
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

const DECIMALS: u32 = 7;
const NAME: &str = "Sandra Token";
const SYMBOL: &str = "SAND";

fn create_token_contract<'a>(env: &Env, admin: &Address) -> SandraTokenClient<'a> {
    let contract_id = env.register_contract(None, SandraToken);
    let client = SandraTokenClient::new(env, &contract_id);
    
    client.initialize(
        admin,
        &String::from_str(env, NAME),
        &String::from_str(env, SYMBOL),
        &DECIMALS,
    );
    
    client
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    assert_eq!(client.name(), String::from_str(&env, NAME));
    assert_eq!(client.symbol(), String::from_str(&env, SYMBOL));
    assert_eq!(client.decimals(), DECIMALS);
    assert_eq!(client.get_admin(), admin);
    assert_eq!(client.total_supply(), 0);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    client.initialize(
        &admin,
        &String::from_str(&env, "Second Token"),
        &String::from_str(&env, "SEC"),
        &18,
    );
}

#[test]
fn test_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    let amount = 1000;
    client.mint(&user, &amount);

    assert_eq!(client.balance(&user), amount);
    assert_eq!(client.total_supply(), amount);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    let amount = 1000;
    client.mint(&user1, &amount);
    
    let transfer_amount = 300;
    client.transfer(&user1, &user2, &transfer_amount);

    assert_eq!(client.balance(&user1), amount - transfer_amount);
    assert_eq!(client.balance(&user2), transfer_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    let amount = 1000;
    client.mint(&user1, &amount);
    
    client.transfer(&user1, &user2, &(amount + 1));
}

#[test]
fn test_approve_and_transfer_from() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let spender = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    let amount = 1000;
    let allowance_amount = 300;
    let transfer_amount = 200;

    client.mint(&user1, &amount);
    client.approve(&user1, &spender, &allowance_amount, &200);

    assert_eq!(client.allowance(&user1, &spender), allowance_amount);

    client.transfer_from(&spender, &user1, &user2, &transfer_amount);

    assert_eq!(client.balance(&user1), amount - transfer_amount);
    assert_eq!(client.balance(&user2), transfer_amount);
    assert_eq!(client.allowance(&user1, &spender), allowance_amount - transfer_amount);
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    let mint_amount = 1000;
    let burn_amount = 300;

    client.mint(&user, &mint_amount);
    client.burn(&user, &burn_amount);

    assert_eq!(client.balance(&user), mint_amount - burn_amount);
    assert_eq!(client.total_supply(), mint_amount - burn_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_negative_amount_mint() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    client.mint(&user, &-100);
}

#[test]
fn test_zero_amount_operations() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let client = create_token_contract(&env, &admin);

    client.mint(&user1, &1000);
    
    let initial_balance1 = client.balance(&user1);
    let initial_balance2 = client.balance(&user2);
    let initial_supply = client.total_supply();

    client.mint(&user1, &0);
    client.transfer(&user1, &user2, &0);
    client.burn(&user1, &0);

    assert_eq!(client.balance(&user1), initial_balance1);
    assert_eq!(client.balance(&user2), initial_balance2);
    assert_eq!(client.total_supply(), initial_supply);
}