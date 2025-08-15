#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address};
use soroban_sdk::testutils::Address as AddressTestUtils;

fn setup() -> (Env, Sep41Client<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(Sep41, ());
    let client = Sep41Client::new(&env, &contract_id);
    
    // Create test addresses
    let selina = Address::generate(&env);
    let gomez = Address::generate(&env);
    
    (env, client, selina, gomez)
}

#[test]
fn test_metadata() {
    let (env, client, _, _) = setup();
    
    assert_eq!(client.name(), String::from_str(&env, "Age Devs"));
    assert_eq!(client.symbol(), String::from_str(&env, "AGEDEVS"));
    assert_eq!(client.decimals(), 18);
}

#[test]
fn test_balance_initially_zero() {
    let (_, client, selina, _) = setup();
    
    assert_eq!(client.balance(&selina), 0);
}

#[test]
fn test_allowance_initially_zero() {
    let (_, client, selina, gomez) = setup();
    
    assert_eq!(client.allowance(&selina, &gomez), 0);
}
