use soroban_sdk::{Address, Env, String};
use crate::{token::TokenTrait, storage, events};

pub struct Token;

impl TokenTrait for Token {
    fn initialize(env: Env, admin: Address, total_supply: i128) {
        storage::set_admin(&env, &admin);
        storage::set_total_supply(&env, total_supply);
        storage::set_balance(&env, &admin, total_supply);
    }

    fn name(env: Env) -> String {
        String::from_str(&env, "MyToken")
    }

    fn symbol(env: Env) -> String {
        String::from_str(&env, "MTK")
    }

    fn decimals(_env: Env) -> u32 {
        18
    } 

    fn total_supply(env: Env) -> i128 {
        storage::get_total_supply(&env)
    }

    fn balance_of(env: Env, owner: Address) -> i128 {
        storage::get_balance(&env, &owner)
    }

    fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        storage::get_allowance(&env, &owner, &spender)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();
        storage::set_allowance(&env, &from, &spender, amount);
        events::approve_event(&env, from, spender, amount);
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let mut balance_from = storage::get_balance(&env, &from);
        assert!(balance_from >= amount, "Insufficient balance");

        balance_from -= amount;
        let mut balance_to = storage::get_balance(&env, &to);
        balance_to += amount;

        storage::set_balance(&env, &from, balance_from);
        storage::set_balance(&env, &to, balance_to);

        events::transfer_event(&env, from, to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let mut allowance = storage::get_allowance(&env, &from, &spender);
        assert!(allowance >= amount, "Allowance exceeded");

        let mut balance_from = storage::get_balance(&env, &from);
        assert!(balance_from >= amount, "Insufficient balance");

        balance_from -= amount;
        allowance -= amount;

        let mut balance_to = storage::get_balance(&env, &to);
        balance_to += amount;

        storage::set_balance(&env, &from, balance_from);
        storage::set_balance(&env, &to, balance_to);
        storage::set_allowance(&env, &from, &spender, allowance);

        events::transfer_event(&env, from, to, amount);
    }

    fn mint(env: Env, to: Address, amount: i128) {
        let admin = env.current_contract_address();
        let mut total_supply = Self::total_supply(env.clone());
        total_supply += amount;

        storage::set_total_supply(&env, total_supply);
        let mut balance_to = storage::get_balance(&env, &to);
        balance_to += amount;

        storage::set_balance(&env, &to, balance_to);

        events::transfer_event(&env, admin, to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let mut balance_from = storage::get_balance(&env, &from);
        assert!(balance_from >= amount, "Insufficient balance");

        balance_from -= amount;
        let mut total_supply = Self::total_supply(env.clone());
        total_supply -= amount;

        storage::set_balance(&env, &from, balance_from);
        storage::set_total_supply(&env, total_supply);

        events::transfer_event(&env, from, env.current_contract_address(), amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        let mut allowance = storage::get_allowance(&env, &from, &spender);
        assert!(allowance >= amount, "Allowance exceeded");

        let mut balance_from = storage::get_balance(&env, &from);
        assert!(balance_from >= amount, "Insufficient balance");

        balance_from -= amount;
        allowance -= amount;
        let mut total_supply = Self::total_supply(env.clone());
        total_supply -= amount;

        storage::set_balance(&env, &from, balance_from);
        storage::set_allowance(&env, &from, &spender, allowance);
        storage::set_total_supply(&env, total_supply);

        events::transfer_event(&env, from, env.current_contract_address(), amount);
    }
}
