#![cfg(test)]
use crate::ballor_token::{TokenContract, TokenContractClient};
use soroban_sdk::{testutils::Address as AddressUtils, Address, Env, String};

fn setup(env: &Env, admin: &Address) -> TokenContractClient<'static> {
    let name = String::from_str(&env, &"ballor-token");
    let symbol = String::from_str(&env, &"BLT");

    let contract_id = env.register(TokenContract, (admin, name.clone(), symbol.clone(), 18_u32));
    TokenContractClient::new(&env, &contract_id)
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let user3 = Address::generate(&env);
    let user4 = Address::generate(&env);

    let ballor_token = setup(&env, &admin);

    let total_supply = ballor_token.get_total_supply();
    assert_eq!(total_supply, 0);

    let name = String::from_str(&env, &"ballor-token");
    let symbol = String::from_str(&env, &"BLT");

    // test getting metadata
    let token_name = ballor_token.name();
    let token_symbol = ballor_token.symbol();
    let decimals = ballor_token.decimals();

    assert_eq!(token_name, name);
    assert_eq!(token_symbol, symbol);
    assert_eq!(decimals, 18_u32);

    // Testing Minting
    let amount: i128 = 1_000_000;
    ballor_token.mint(&user1, &amount);
    ballor_token.mint(&user2, &amount);

    let user1_balance = ballor_token.balance(&user1);
    let user2_balance = ballor_token.balance(&user2);
    let total_supply = ballor_token.get_total_supply();

    assert_eq!(user1_balance, amount);
    assert_eq!(user2_balance, amount);
    assert_eq!(total_supply, (amount + amount));

    // Testing Burning
    let burn_amount: i128 = 500_000;
    ballor_token.burn(&user1, &burn_amount);
    let user1_balance_after_burn = ballor_token.balance(&user1);
    let total_supply = ballor_token.get_total_supply();

    assert_eq!(user1_balance_after_burn, amount - burn_amount);
    assert_eq!(total_supply, (amount + amount - burn_amount));

    // Testing Transfer
    let transfer_amount: i128 = 500_000;
    ballor_token.transfer(&user2, &user3, &transfer_amount);
    let user2_balance_after_transfer = ballor_token.balance(&user2);
    let user3_balance_after_transfer = ballor_token.balance(&user3);

    assert_eq!(user2_balance_after_transfer, amount - transfer_amount);
    assert_eq!(user3_balance_after_transfer, transfer_amount);

    // Testing Approve, Allowance, TransferFrom, BurnFrom
    let transfer_from_amount: i128 = 250_000;
    let expiration_ledger = 100_u32;

    ballor_token.approve(&user2, &user3, &transfer_from_amount, &expiration_ledger);

    let allowed_amount = ballor_token.allowance(&user2, &user3);
    assert_eq!(allowed_amount, transfer_from_amount);

    let burn_from_amount: i128 = 50_000;
    ballor_token.burn_from(&user3, &user2, &burn_from_amount);

    let allowed_amount_after_burn = ballor_token.allowance(&user2, &user3);
    assert_eq!(
        allowed_amount_after_burn,
        transfer_from_amount - burn_from_amount
    );

    let total_supply = ballor_token.get_total_supply();
    assert_eq!(
        total_supply,
        (amount + amount - burn_amount - burn_from_amount)
    );

    ballor_token.transfer_from(&user3, &user2, &user4, &allowed_amount_after_burn);

    let user2_balance_after_transfer_from = ballor_token.balance(&user2);
    let user4_balance_after_transfer_from = ballor_token.balance(&user4);

    assert_eq!(user2_balance_after_transfer_from, transfer_from_amount);
    assert_eq!(user4_balance_after_transfer_from, allowed_amount_after_burn);
}
