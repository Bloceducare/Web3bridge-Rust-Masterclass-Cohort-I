use soroban_sdk::{Address, Env, String};

/// Trait for SEP-41 Token Standard
pub trait TokenTrait {
    fn initialize(env: Env, admin: Address, total_supply: i128);
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn decimals(env: Env) -> u32;

    fn total_supply(env: Env) -> i128;
    fn balance_of(env: Env, owner: Address) -> i128;

    fn allowance(env: Env, owner: Address, spender: Address) -> i128;
    fn approve(env: Env, from: Address, spender: Address, amount: i128);
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    fn mint(env: Env, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);
}
