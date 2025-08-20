#![cfg(test)]
extern crate std;

use crate::{SepToken, SepTokenClient};
use soroban_sdk::{
    log, symbol_short,
    testutils::{Address as _, Events as _},
    token::TokenClient,
    vec, Address, Env, IntoVal, String,
};

fn create_token<'a>(e: &Env, admin: &Address) -> SepTokenClient<'a> {
    let token = SepTokenClient::new(e, &e.register(SepToken, ()));
    token.initialize(
        admin,
        &7,
        &String::from_str(e, "Test Token"),
        &String::from_str(e, "TEST"),
    );
    token
}

#[test]
fn test_initialize() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let token = create_token(&e, &admin);

    assert_eq!(token.decimals(), 7);
    assert_eq!(token.name(), String::from_str(&e, "Test Token"));
    assert_eq!(token.symbol(), String::from_str(&e, "TEST"));
    assert_eq!(token.admin(), admin);
    assert_eq!(token.total_supply(), 0);
}

#[test]
fn test_mint() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);
    assert_eq!(token.total_supply(), 1000);
}

#[test]
fn test_transfer() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &300);
    assert_eq!(token.balance(&user1), 700);
    assert_eq!(token.balance(&user2), 300);
}

#[test]
fn test_allowance_and_transfer_from() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &500, &200);
    assert_eq!(token.allowance(&user1, &user3), 500);

    token.transfer_from(&user3, &user1, &user2, &200);
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);
    assert_eq!(token.allowance(&user1, &user3), 300);
}

#[test]
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);
    assert_eq!(token.total_supply(), 1000);

    token.burn(&user, &300);
    assert_eq!(token.balance(&user), 700);
    assert_eq!(token.total_supply(), 700);
}

#[test]
fn test_burn_from() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    token.approve(&user1, &user2, &500, &200);

    token.burn_from(&user2, &user1, &200);
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.allowance(&user1, &user2), 300);
    assert_eq!(token.total_supply(), 800);
}

#[test]
fn test_clawback() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);
    assert_eq!(token.total_supply(), 1000);

    token.clawback(&user, &300);
    assert_eq!(token.balance(&user), 700);
    assert_eq!(token.total_supply(), 700);
}

#[test]
fn test_set_admin() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);
    let token = create_token(&e, &admin1);

    token.set_admin(&admin2);
    assert_eq!(token.admin(), admin2);
}

#[test]
#[should_panic(expected = "InsufficientBalance")]
fn test_transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    token.transfer(&user1, &user2, &1500); // Should panic
}

#[test]
#[should_panic(expected = "InsufficientAllowance")]
fn test_transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    token.approve(&user1, &user3, &100, &200);
    token.transfer_from(&user3, &user1, &user2, &500); // Should panic
}

#[test]
fn test_token_client_compatibility() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);

    let token_contract = SepTokenClient::new(&e, &e.register(SepToken, ()));
    token_contract.initialize(
        &admin,
        &18,
        &String::from_str(&e, "USD Coin"),
        &String::from_str(&e, "USDC"),
    );

    // Test using the standard TokenClient
    let token = TokenClient::new(&e, &token_contract.address);

    // Mint some tokens
    token_contract.mint(&user1, &1_000_000_000_000_000_000i128); // 1 USDC (18 decimals)

    assert_eq!(token.balance(&user1), 1_000_000_000_000_000_000i128);
    assert_eq!(token.decimals(), 18);
    assert_eq!(token.name(), String::from_str(&e, "USD Coin"));
    assert_eq!(token.symbol(), String::from_str(&e, "USDC"));

    // Test transfer
    token.transfer(&user1, &user2, &500_000_000_000_000_000i128); // 0.5 USDC
    assert_eq!(token.balance(&user1), 500_000_000_000_000_000i128);
    assert_eq!(token.balance(&user2), 500_000_000_000_000_000i128);

    // Test approve and transfer_from
    token.approve(&user1, &user2, &100_000_000_000_000_000i128, &1000); // 0.1 USDC allowance
    assert_eq!(token.allowance(&user1, &user2), 100_000_000_000_000_000i128);

    let user3 = Address::generate(&e);
    token.transfer_from(&user2, &user1, &user3, &50_000_000_000_000_000i128); // 0.05 USDC
    assert_eq!(token.balance(&user1), 450_000_000_000_000_000i128);
    assert_eq!(token.balance(&user3), 50_000_000_000_000_000i128);
    assert_eq!(token.allowance(&user1, &user2), 50_000_000_000_000_000i128);
}

#[test]
fn test_zero_amount_operations() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);

    // Test zero amount transfer (should succeed)
    token.transfer(&user1, &user2, &0);
    assert_eq!(token.balance(&user1), 1000);
    assert_eq!(token.balance(&user2), 0);

    // Test zero amount burn (should succeed)
    token.burn(&user1, &0);
    assert_eq!(token.balance(&user1), 1000);
    assert_eq!(token.total_supply(), 1000);
}

#[test]
fn test_large_amounts() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token = create_token(&e, &admin);

    // Test with large amounts (close to i128 max)
    let large_amount = 1_000_000_000_000_000_000_000_000_000i128; // 1 billion tokens with 18 decimals

    token.mint(&user, &large_amount);
    assert_eq!(token.balance(&user), large_amount);
    assert_eq!(token.total_supply(), large_amount);
}

#[test]
fn test_allowance_expiration() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);

    // Set allowance with current ledger sequence (should be valid)
    let current_seq = e.ledger().sequence();
    token.approve(&user1, &user2, &500, &(current_seq + 100));
    assert_eq!(token.allowance(&user1, &user2), 500);

    // Test setting allowance to 0 (should remove allowance)
    token.approve(&user1, &user2, &0, &(current_seq + 100));
    assert_eq!(token.allowance(&user1, &user2), 0);
}

#[test]
fn test_events() {
    // take away for me regarding this event tests:
    /*
     * The key points are:
     * Each event is a tuple: (contract_address, topics.into_val(&e), data.into_val(&e))
     * Use .into_val(&e) to convert the topics and data to the proper Soroban values
     * Use .clone() for the addresses
     * This matches the format shown in the Soroban events documentation where they use vec![&env, ...] with the environment reference as the first parameter.
     */
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    // Count events before operations
    let event_count = e.events().all();

    log!(&e, "admin: {}", admin);
    log!(&e, "user1: {}", user1);
    log!(&e, "user2: {}", user2);
    log!(&e, "token: {}", token.address);

    log!(&e, "Initial event count: {}", event_count);

    // Test mint event - should emit an event
    token.mint(&user1, &1000);
    let events_after_mint = e.events().all();
    // [[CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAITA4, [mint, CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM, CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFCT4], 1000]]]
    log!(&e, "Events after mint: {}", events_after_mint);

    assert_eq!(
        events_after_mint,
        vec![
            &e,
            (
                token.address.clone(),
                (symbol_short!("mint"), admin.clone(), user1.clone()).into_val(&e),
                1000_i128.into_val(&e)
            )
        ]
    );

    // Test transfer event - should emit an event
    let events_before_transfer = e.events().all();
    log!(&e, "Events before transfer: {}", events_before_transfer);
    token.transfer(&user1, &user2, &300);
    let events_after_transfer = e.events().all();
    log!(&e, "Events after transfer: {}", events_after_transfer);

    assert_eq!(
      events_after_transfer,
      vec![
        &e,
        (token.address.clone(), (symbol_short!("transfer"), user1.clone(), user2.clone()).into_val(&e), 300_i128.into_val(&e))
      ]
    );


    // Test burn event - should emit an event
    let events_before_burn = e.events().all();
    log!(&e, "Events before burn: {}", events_before_burn);
    token.burn(&user1, &200);
    let events_after_burn = e.events().all();
    log!(&e, "Events after burn: {}", events_after_burn);

    assert_eq!(
      events_after_burn,
      vec![
        &e,
        (token.address.clone(), (symbol_short!("burn"), user1.clone()).into_val(&e), 200_i128.into_val(&e))
      ]
    )
}
