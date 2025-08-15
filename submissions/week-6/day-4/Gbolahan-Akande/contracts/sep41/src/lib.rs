#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Address};

pub trait TokenInterface {
    fn allowance(env: Env, owner: Address, spender: Address) -> i128;

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

#[contract]
pub struct Sep41;

#[contractimpl]
impl TokenInterface for Sep41 {
    fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        let (amount, expiration_ledger) = env.storage().persistent().get(&(owner, spender)).unwrap_or((0, 0));
        if (amount > 0) && (expiration_ledger > env.ledger().sequence()) {
            amount
        } else {
            0
        }
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        env.storage().persistent().set(&(from, spender), &(amount, expiration_ledger))
    }

    fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&id).unwrap_or(0)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let from_balance = env.storage().persistent().get(&from).unwrap_or(0);
        assert!(from_balance >= amount, "Not enough balance");
        env.storage().persistent().set(&from, &(from_balance - amount));
        let to_balance = env.storage().persistent().get(&to).unwrap_or(0);
        env.storage().persistent().set(&to, &(to_balance + amount));
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let (current_allowance, expiration_ledger) = env.storage().persistent().get(&(from.clone(), spender.clone())).unwrap_or((0, 0));
        assert!(current_allowance >= amount, "Not allowed");
        Self::transfer(env.clone(), from.clone(), to.clone(), amount);
        let new_allowance = current_allowance - amount;
        if new_allowance > 0 {
            env.storage().persistent().set(&(from, spender), &(new_allowance, expiration_ledger));
        } else {
            env.storage().persistent().remove(&(from, spender));
        }
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let from_balance = env.storage().persistent().get(&from).unwrap_or(0);
        assert!(from_balance >= amount, "Not enough balance to burn");
        env.storage().persistent().set(&from, &(from_balance - amount));
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        let (current_allowance, expiration_ledger) = env.storage().persistent().get(&(from.clone(), spender.clone())).unwrap_or((0, 0));
        assert!(current_allowance >= amount, "Not allowed to burn");
        Self::burn(env.clone(), from.clone(), amount);
        let new_allowance = current_allowance - amount;
        if new_allowance > 0 {
            env.storage().persistent().set(&(from, spender), &(new_allowance, expiration_ledger));
        } else {
            env.storage().persistent().remove(&(from, spender));
        }
    }

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> String {
        String::from_str(&env, "Age Devs")
    }

    fn symbol(env: Env) -> String {
        String::from_str(&env, "AGEDEVS")
    }
}

mod test;
