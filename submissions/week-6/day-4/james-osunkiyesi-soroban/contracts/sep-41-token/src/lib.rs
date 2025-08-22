#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use crate::events::*;
use crate::sep_0041::TokenInterface;
use crate::storage::*;

#[contract]
pub struct Contract;


#[contractimpl]
impl TokenInterface for Contract {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = DataKey::Allowance(AllowanceKey {
            from,
            spender
        });
        env.storage().persistent().get::<DataKey, i128>(&key).unwrap_or(0)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        if amount < 0 {
            panic!("Amount cannot be negative");
        }
        let key = DataKey::Allowance(AllowanceKey {
            from: get_user(from.clone()),
            spender: spender.clone()
        });
        env.storage().persistent().set(&key, &amount);
        // Emit approve event
        emit_approve_event(&env, from, spender, amount, live_until_ledger);
    }

    fn balance(env: Env, id: Address) -> i128 {
        let key = DataKey::Balance(id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        if amount < 0 {
            panic!("Amount cannot be negative");
        }

        let balance_key = DataKey::Balance(get_user(from.clone()));
        let balance = env.storage().persistent().get::<DataKey, i128>(&balance_key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient Balance")
        }

        let receiver_key = DataKey::Balance(to.clone());
        let receiver_balance = env.storage().persistent().get::<DataKey, i128>(&receiver_key).unwrap_or(0);

        env.storage().persistent().set(&balance_key, &(balance - amount));
        env.storage().persistent().set(&receiver_key, &(receiver_balance + amount));

        // Emit transfer event
        emit_transfer_event(&env, from, to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128)  {
        if amount < 0 {
            panic!("Amount cannot be negative");
        }

        let allowance = Self::allowance(env.clone(), from.clone(), get_user(spender.clone()));

        if allowance < amount {
            panic!("Insufficient Allowance")
        }

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: get_user(from.clone()),
            spender
        });

        env.storage().persistent().set(&allowance_key, &(allowance - amount));

        let balance_key = DataKey::Balance(from.clone());
        let balance = env.storage().persistent().get::<DataKey, i128>(&balance_key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient Balance")
        }

        let receiver_key = DataKey::Balance(to.clone());
        let receiver_balance = env.storage().persistent().get::<DataKey, i128>(&receiver_key).unwrap_or(0);

        env.storage().persistent().set(&balance_key, &(balance - amount));
        env.storage().persistent().set(&receiver_key, &(receiver_balance + amount));

        // Emit transfer event
        emit_transfer_event(&env, from, to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        if amount < 0 {
            panic!("Amount cannot be negative");
        }

        let balance_key = DataKey::Balance(get_user(from.clone()));
        let balance = env.storage().persistent().get::<DataKey, i128>(&balance_key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient Balance")
        }

        env.storage().persistent().set(&balance_key, &(balance - amount));

        // Emit burn event
        emit_burn_event(&env, from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        if amount < 0 {
            panic!("Amount cannot be negative");
        }

        let allowance = Self::allowance(env.clone(), from.clone(), get_user(spender.clone()));

        if allowance < amount {
            panic!("Insufficient Allowance")
        }

        let allowance_key = DataKey::Allowance(AllowanceKey {
            from: get_user(from.clone()),
            spender
        });

        env.storage().persistent().set(&allowance_key, &(allowance - amount));

        let balance_key = DataKey::Balance(from.clone());
        let balance = env.storage().persistent().get::<DataKey, i128>(&balance_key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient Balance")
        }

        env.storage().persistent().set(&balance_key, &(balance - amount));

        // Emit burn event
        emit_burn_event(&env, from, amount);
    }

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> String{
        String::from_str(&env, "RUG PULL")
    }

    fn symbol(env: Env) -> String {
        String::from_str(&env, "RGP")
    }
}

pub fn get_user(addr: Address) -> Address {
    // Uses the require_auth method to ensure that the call is made from the given address.
    addr.require_auth();
    addr
}

mod test;
mod sep_0041;
mod events;
mod storage;
