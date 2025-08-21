use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::event;
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) {
        if has_administrator(&env) {
            panic!("already initialized")
        }
        write_administrator(&env, &admin);
        if decimal > 18 {
            panic!("decimal must not be greater than 18")
        }
        write_metadata(
            &env,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&env);
        admin.require_auth();

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        receive_balance(&env, to.clone(), amount);
        event::mint(&env, admin, to, amount);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&env, from, spender).amount
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&env, from.clone(), spender.clone(), amount, expiration_ledger);
        event::approve(&env, from, spender, amount, expiration_ledger);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&env, id)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&env, from.clone(), amount);
        receive_balance(&env, to.clone(), amount);
        event::transfer(&env, from, to, amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&env, from.clone(), spender, amount);
        spend_balance(&env, from.clone(), amount);
        receive_balance(&env, to.clone(), amount);
        event::transfer(&env, from, to, amount);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_balance(&env, from.clone(), amount);
        event::burn(&env, from, amount);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        spend_allowance(&env, from.clone(), spender, amount);
        spend_balance(&env, from.clone(), amount);
        event::burn(&env, from, amount);
    }

    pub fn decimals(env: Env) -> u32 {
        read_decimal(&env)
    }

    pub fn name(env: Env) -> String {
        read_name(&env)
    }

    pub fn symbol(env: Env) -> String {
        read_symbol(&env)
    }

    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_administrator(&env)
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();

        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }
}