use soroban_sdk::{ Address, Env, String };

pub trait TokenInterface {
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);

    fn balance(env: Env, id: Address) -> i128;

    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    fn burn(env: Env, from: Address, amount: i128);

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    fn decimals(env: Env) -> u32;

    fn name(env: Env) -> String;

    fn symbol(env: Env) -> String;
}
