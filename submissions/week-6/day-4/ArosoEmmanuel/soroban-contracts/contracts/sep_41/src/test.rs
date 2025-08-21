#![cfg(test)]

use crate::datatypes::{DECIMAL, NAME, SYMBOL, ZERO_ADDRESS};
use crate::token::{EmarcToken, EmarcTokenClient};
use soroban_sdk::testutils::{
    Address as _, AuthorizedFunction, AuthorizedInvocation, Events, MockAuth, MockAuthInvoke,
};
use soroban_sdk::{vec, Address, Env, IntoVal, String, Symbol, TryFromVal, Val, Vec};

// Helper setup function with static lifetime
fn setup_env<'a>() -> (Env, Address, EmarcTokenClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmarcToken, ());
    let client = EmarcTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let symbol = String::from_str(&env, SYMBOL);
    let name = String::from_str(&env, NAME);

    // Initialize the contract
    client.initialize(&admin, &INITIAL_SUPPLY, &DECIMAL, &name, &symbol);

    (env, admin, client)
}

fn generate_user(env: &Env) -> Address {
    Address::generate(env)
}

fn mock_unauthorized_action(
    env: &Env,
    user: &Address,
    contract: &EmarcTokenClient,
    fn_name: &str,
    args: Vec<Val>,
) {
    env.mock_auths(&[MockAuth {
        address: user,
        invoke: &MockAuthInvoke {
            contract: &contract.address,
            fn_name,
            args,
            sub_invokes: &[],
        },
    }]);
}

static INITIAL_SUPPLY: i128 = 1_000_000_0000000i128; // 1,000,000 tokens with 7 decimals
static TRANSFER_AMOUNT: i128 = 100_0000000i128; // 100 tokens
const MINT_AMOUNT: i128 = 500_0000000i128; // 500 tokens
const BURN_AMOUNT: i128 = 100_0000000i128; // 100 tokens

#[test]
fn test_initialization() {
    let (env, admin, client) = setup_env();

    assert_eq!(client.total_supply(), INITIAL_SUPPLY);
    assert_eq!(client.balance(&admin), INITIAL_SUPPLY);
    assert_eq!(client.decimals(), 7);
    assert_eq!(client.name(), String::from_str(&env, "EmarcToken"));
    assert_eq!(client.symbol(), String::from_str(&env, "EMARC"));
}

#[test]
fn test_transfer() {
    let (env, admin, client) = setup_env();
    let user1 = generate_user(&env);

    // Transfer from admin to user1
    client.transfer(&admin, &user1, &TRANSFER_AMOUNT);

    assert_eq!(client.balance(&admin), INITIAL_SUPPLY - TRANSFER_AMOUNT);
    assert_eq!(client.balance(&user1), TRANSFER_AMOUNT);
}

#[test]
fn test_approve_and_transfer_from() {
    let (env, admin, client) = setup_env();

    let user1 = generate_user(&env);
    let user2 = generate_user(&env);

    // Transfer some tokens to user1
    client.transfer(&admin, &user1, &500_0000000i128);

    // User1 approves user2 to spend 100 tokens
    let allowance_amount = 100_0000000i128;
    let expiration = env.ledger().sequence() + 1000;
    client.approve(&user1, &user2, &allowance_amount, &expiration);

    assert_eq!(client.allowance(&user1, &user2), allowance_amount);

    // User2 transfers from user1 to themselves
    let transfer_amount = 50_0000000i128;
    client.transfer_from(&user2, &user1, &user2, &transfer_amount);

    assert_eq!(client.balance(&user1), 500_0000000 - transfer_amount);
    assert_eq!(client.balance(&user2), transfer_amount);
    assert_eq!(
        client.allowance(&user1, &user2),
        allowance_amount - transfer_amount
    );
}

#[test]
fn test_burn() {
    let (_, admin, client) = setup_env();

    // Burn some tokens
    client.burn(&admin, &BURN_AMOUNT);

    assert_eq!(client.balance(&admin), INITIAL_SUPPLY - BURN_AMOUNT);
    assert_eq!(client.total_supply(), INITIAL_SUPPLY - BURN_AMOUNT);
}

#[test]
fn test_mint() {
    let (env, _, client) = setup_env();
    let user1 = generate_user(&env);

    // Mint new tokens to user1
    client.mint(&user1, &MINT_AMOUNT);

    assert_eq!(client.balance(&user1), MINT_AMOUNT);
    assert_eq!(client.total_supply(), INITIAL_SUPPLY + MINT_AMOUNT);
}

#[test]
fn test_workflow_emit_events() {
    let (env, _, client) = setup_env();
    let user1 = generate_user(&env);
    let user2 = generate_user(&env);

    let zero_address: Address = Address::from_str(&env, ZERO_ADDRESS);

    // Admin mints tokens to user1
    client.mint(&user1, &MINT_AMOUNT);

    //-----     MINTING EVENT TESTING   -----
    let events_after_mint = env.events().all();
    assert_eq!(events_after_mint.len(), 1);

    let mint_event = events_after_mint.get(0).unwrap();
    // assert_eq!(mint_event.0, contract_id);

    let mint_topics = mint_event.1;
    assert_eq!(
        mint_topics,
        vec![&env, Symbol::new(&env, "mint")].into_val(&env)
    );

    let mint_data = mint_event.2;
    let mint_data_map: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::Val> =
        soroban_sdk::Map::try_from_val(&env, &mint_data).unwrap();

    let expected_mint_data: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::Val> =
        soroban_sdk::Map::from_array(
            &env,
            [
                (Symbol::new(&env, "to"), user1.clone().into_val(&env)),
                (
                    Symbol::new(&env, "from"),
                    zero_address.clone().into_val(&env),
                ),
                (Symbol::new(&env, "amount"), MINT_AMOUNT.into_val(&env)),
            ],
        );

    assert_eq!(mint_data_map, expected_mint_data);

    // -----    TRANSFER EVENT TESTING      -----
    // User1 transfers tokens to user2
    let transfer_amount = TRANSFER_AMOUNT;
    client.transfer(&user1, &user2, &transfer_amount);

    // Verify transfer event
    let events_after_transfer = env.events().all();
    assert_eq!(events_after_transfer.len(), 1);

    let transfer_event = events_after_transfer.get(0).unwrap();

    let transfer_topics = transfer_event.1;
    assert_eq!(
        transfer_topics,
        vec![&env, Symbol::new(&env, "transfer")].into_val(&env)
    );

    let transfer_data = transfer_event.2;
    let transfer_data_map: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::Val> =
        soroban_sdk::Map::try_from_val(&env, &transfer_data).unwrap();

    let expected_transfer_data: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::Val> =
        soroban_sdk::Map::from_array(
            &env,
            [
                (Symbol::new(&env, "from"), user1.clone().into_val(&env)),
                (Symbol::new(&env, "to"), user2.clone().into_val(&env)),
                (Symbol::new(&env, "amount"), transfer_amount.into_val(&env)),
            ],
        );

    assert_eq!(transfer_data_map, expected_transfer_data);

    // ------   BURNING EVENT TESTING   -----
    // User2 burns some tokens
    let burn_amount = BURN_AMOUNT;
    client.burn(&user2, &burn_amount);

    // Verify burn event
    let events_after_burn = env.events().all();
    assert_eq!(events_after_burn.len(), 1);

    let burn_event = events_after_burn.get(0).unwrap();

    let burn_topics = burn_event.1;
    assert_eq!(
        burn_topics,
        vec![&env, Symbol::new(&env, "burn")].into_val(&env)
    );

    let burn_data = burn_event.2;
    let burn_data_map: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::Val> =
        soroban_sdk::Map::try_from_val(&env, &burn_data).unwrap();

    let expected_burn_data: soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::Val> =
        soroban_sdk::Map::from_array(
            &env,
            [
                (Symbol::new(&env, "from"), user2.clone().into_val(&env)),
                (Symbol::new(&env, "to"), zero_address.clone().into_val(&env)),
                (Symbol::new(&env, "amount"), burn_amount.into_val(&env)),
            ],
        );

    assert_eq!(burn_data_map, expected_burn_data);

    // -----    FINAL BALANCE VERIFICATION     -----
    assert_eq!(client.balance(&user1), MINT_AMOUNT - transfer_amount);
    assert_eq!(client.balance(&user2), transfer_amount - burn_amount);
    assert_eq!(
        client.total_supply(),
        INITIAL_SUPPLY + MINT_AMOUNT - burn_amount
    );
}
