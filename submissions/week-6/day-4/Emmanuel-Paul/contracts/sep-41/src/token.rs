use crate::errors::TokenError;
use crate::events::Events;
use crate::storage::{AllowanceData, Storage};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, String};

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
    fn initialize(env: Env, admin: Address, decimals: u32, name: String, symbol: String);
    fn mint(env: Env, to: Address, amount: i128);
}

#[contract]
pub struct Sep41Token;

#[contractimpl]
impl TokenInterface for Sep41Token {
    fn initialize(env: Env, admin: Address, decimals: u32, name: String, symbol: String) {
        if Storage::get_admin(&env).is_some() {
            panic_with_error!(env, TokenError::AlreadyInitialized);
        }
        admin.require_auth();
        Storage::set_admin(&env, &admin);
        Storage::set_metadata(&env, decimals, name, symbol);
    }

    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        if let Some(allowance_data) = Storage::get_allowance(&env, &from, &spender) {
            if allowance_data.expiration_ledger < env.ledger().sequence() {
                0
            } else {
                allowance_data.amount
            }
        } else {
            0
        }
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }
        if amount > 0 && expiration_ledger < env.ledger().sequence() {
            panic_with_error!(env, TokenError::ExpiredAllowance);
        }
        let allowance = AllowanceData {
            amount,
            expiration_ledger,
        };
        Storage::set_allowance(&env, &from, &spender, allowance);
        Events::approve(&env, &from, &spender, amount, expiration_ledger);
    }

    fn balance(env: Env, id: Address) -> i128 {
        Storage::get_balance(&env, &id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }
        let from_balance = Storage::get_balance(&env, &from);
        if from_balance < amount {
            panic_with_error!(env, TokenError::InsufficientBalance);
        }
        Storage::set_balance(&env, &from, from_balance - amount);
        Storage::set_balance(&env, &to, Storage::get_balance(&env, &to) + amount);
        Events::transfer(&env, &from, &to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }
        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic_with_error!(env, TokenError::InsufficientAllowance);
        }
        let from_balance = Storage::get_balance(&env, &from);
        if from_balance < amount {
            panic_with_error!(env, TokenError::InsufficientBalance);
        }
        Storage::set_balance(&env, &from, from_balance - amount);
        Storage::set_balance(&env, &to, Storage::get_balance(&env, &to) + amount);
        if allowance == amount {
            Storage::remove_allowance(&env, &from, &spender);
        } else {
            Storage::set_allowance(
                &env,
                &from,
                &spender,
                AllowanceData {
                    amount: allowance - amount,
                    expiration_ledger: env.ledger().sequence(),
                },
            );
        }
        Events::transfer(&env, &from, &to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        let admin = Storage::get_admin(&env).expect("Admin not set");
        if from != admin {
            panic_with_error!(env, TokenError::Unauthorized);
        }
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }
        let from_balance = Storage::get_balance(&env, &from);
        if from_balance < amount {
            panic_with_error!(env, TokenError::InsufficientBalance);
        }
        Storage::set_balance(&env, &from, from_balance - amount);
        Events::burn(&env, &from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        let admin = Storage::get_admin(&env).expect("Admin not set");
        if spender != admin {
            panic_with_error!(env, TokenError::Unauthorized);
        }
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }
        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());
        if allowance < amount {
            panic_with_error!(env, TokenError::InsufficientAllowance);
        }
        let from_balance = Storage::get_balance(&env, &from);
        if from_balance < amount {
            panic_with_error!(env, TokenError::InsufficientBalance);
        }
        Storage::set_balance(&env, &from, from_balance - amount);
        if allowance == amount {
            Storage::remove_allowance(&env, &from, &spender);
        } else {
            Storage::set_allowance(
                &env,
                &from,
                &spender,
                AllowanceData {
                    amount: allowance - amount,
                    expiration_ledger: env.ledger().sequence(),
                },
            );
        }
        Events::burn(&env, &from, amount);
    }

    fn decimals(env: Env) -> u32 {
        Storage::get_metadata(&env)
            .map(|m| m.0)
            .expect("Metadata not set")
    }

    fn name(env: Env) -> String {
        Storage::get_metadata(&env)
            .map(|m| m.1)
            .expect("Metadata not set")
    }

    fn symbol(env: Env) -> String {
        Storage::get_metadata(&env)
            .map(|m| m.2)
            .expect("Metadata not set")
    }

    fn mint(env: Env, to: Address, amount: i128) {
        let admin = Storage::get_admin(&env).expect("Admin not set");
        admin.require_auth();
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }
        Storage::set_balance(&env, &to, Storage::get_balance(&env, &to) + amount);
        Events::transfer(&env, &admin, &to, amount);
    }
}
