#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_basic_functionality() {
    let env = Env::default();
    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    
    client.initialize(&admin, &String::from_str(&env, "Test Token"), &String::from_str(&env, "TEST"), &18u32);
    
    assert_eq!(client.name(), String::from_str(&env, "Test Token"));
    assert_eq!(client.symbol(), String::from_str(&env, "TEST"));
    assert_eq!(client.decimals(), 18u32);
    
    assert_eq!(client.balance(&user1), 0);
    assert_eq!(client.balance(&user2), 0);
    
    client.approve(&user1, &user2, &1000i128, &1000u32);
    assert_eq!(client.allowance(&user1, &user2), 1000);
}
