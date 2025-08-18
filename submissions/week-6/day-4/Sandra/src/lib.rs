#![no_std]

use soroban_sdk::{
    contract, contractimpl, contractmeta, contracttype, Address, Env, String, panic_with_error,
};

pub mod storage;
pub mod events;
pub mod metadata;
pub mod errors;
pub mod admin;

#[cfg(test)]
mod test;

use storage::{ALLOWANCE, BALANCE, TOTAL_SUPPLY, ADMIN};
use events::{Event, ApproveEvent, TransferEvent, BurnEvent, MintEvent};
use errors::TokenError;
use metadata::TokenMetadata;

contractmeta!(
    key = "Description",
    val = "SEP-41 compliant token contract"
);

#[contracttype]
#[derive(Clone)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contract]
pub struct SandraToken;

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
    fn total_supply(env: Env) -> i128;
}

pub trait TokenAdminInterface {
    fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32);
    fn mint(env: Env, to: Address, amount: i128);
    fn set_admin(env: Env, new_admin: Address);
    fn get_admin(env: Env) -> Address;
}

#[contractimpl]
impl TokenInterface for SandraToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = AllowanceDataKey { from: from.clone(), spender: spender.clone() };
        env.storage().temporary().get(&ALLOWANCE(&key)).unwrap_or(0)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        if amount < 0 {
            panic_with_error!(&env, TokenError::NegativeAmount);
        }

        let key = AllowanceDataKey { from: from.clone(), spender: spender.clone() };
        
        if amount == 0 {
            env.storage().temporary().remove(&ALLOWANCE(&key));
        } else {
            env.storage().temporary().set(&ALLOWANCE(&key), &amount);
            if expiration_ledger < env.ledger().sequence() {
                panic_with_error!(&env, TokenError::AllowanceError);
            }
            env.storage().temporary().extend_ttl(&ALLOWANCE(&key), expiration_ledger - env.ledger().sequence(), expiration_ledger - env.ledger().sequence());
        }

        Event::Approve(ApproveEvent {
            from: from.clone(),
            to: spender.clone(),
            amount,
            expiration_ledger,
        })
        .publish(&env);
    }

    fn balance(env: Env, id: Address) -> i128 {
        env.storage().persistent().get(&BALANCE(&id)).unwrap_or(0)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        Self::transfer_internal(&env, &from, &to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        Self::spend_allowance(&env, &from, &spender, amount);
        Self::transfer_internal(&env, &from, &to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        Self::burn_internal(&env, &from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        Self::spend_allowance(&env, &from, &spender, amount);
        Self::burn_internal(&env, &from, amount);
    }

    fn decimals(env: Env) -> u32 {
        TokenMetadata::get_decimals(&env)
    }

    fn name(env: Env) -> String {
        TokenMetadata::get_name(&env)
    }

    fn symbol(env: Env) -> String {
        TokenMetadata::get_symbol(&env)
    }

    fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0)
    }
}

#[contractimpl]
impl TokenAdminInterface for SandraToken {
    fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32) {
        if env.storage().instance().has(&ADMIN) {
            panic_with_error!(&env, TokenError::AlreadyInitialized);
        }

        env.storage().instance().set(&ADMIN, &admin);
        TokenMetadata::set_metadata(&env, name, symbol, decimals);
        
        env.storage().instance().set(&TOTAL_SUPPLY, &0i128);
    }

    fn mint(env: Env, to: Address, amount: i128) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();

        if amount < 0 {
            panic_with_error!(&env, TokenError::NegativeAmount);
        }

        if amount == 0 {
            return;
        }

        let current_balance = Self::balance(env.clone(), to.clone());
        let new_balance = current_balance
            .checked_add(amount)
            .unwrap_or_else(|| panic_with_error!(&env, TokenError::Overflow));

        env.storage().persistent().set(&BALANCE(&to), &new_balance);

        let current_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0);
        let new_supply = current_supply
            .checked_add(amount)
            .unwrap_or_else(|| panic_with_error!(&env, TokenError::Overflow));
        
        env.storage().instance().set(&TOTAL_SUPPLY, &new_supply);

        Event::Mint(MintEvent {
            admin: admin.clone(),
            to: to.clone(),
            amount,
        })
        .publish(&env);
    }

    fn set_admin(env: Env, new_admin: Address) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &new_admin);
    }

    fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&ADMIN)
            .unwrap_or_else(|| panic_with_error!(&env, TokenError::NotInitialized))
    }
}

impl SandraToken {
    fn transfer_internal(env: &Env, from: &Address, to: &Address, amount: i128) {
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }

        if amount == 0 {
            return;
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic_with_error!(env, TokenError::InsufficientBalance);
        }

        let to_balance = Self::balance(env.clone(), to.clone());
        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance
            .checked_add(amount)
            .unwrap_or_else(|| panic_with_error!(env, TokenError::Overflow));

        env.storage().persistent().set(&BALANCE(from), &new_from_balance);
        env.storage().persistent().set(&BALANCE(to), &new_to_balance);

        Event::Transfer(TransferEvent {
            from: from.clone(),
            to: to.clone(),
            amount,
        })
        .publish(env);
    }

    fn burn_internal(env: &Env, from: &Address, amount: i128) {
        if amount < 0 {
            panic_with_error!(env, TokenError::NegativeAmount);
        }

        if amount == 0 {
            return;
        }

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic_with_error!(env, TokenError::InsufficientBalance);
        }

        let new_balance = from_balance - amount;
        env.storage().persistent().set(&BALANCE(from), &new_balance);

        let current_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0);
        let new_supply = current_supply - amount;
        env.storage().instance().set(&TOTAL_SUPPLY, &new_supply);

        Event::Burn(BurnEvent {
            from: from.clone(),
            amount,
        })
        .publish(env);
    }

    fn spend_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) {
        let key = AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone(),
        };
        
        let current_allowance = env.storage().temporary().get(&ALLOWANCE(&key)).unwrap_or(0);
        
        if current_allowance < amount {
            panic_with_error!(env, TokenError::InsufficientAllowance);
        }

        let new_allowance = current_allowance - amount;
        if new_allowance == 0 {
            env.storage().temporary().remove(&ALLOWANCE(&key));
        } else {
            env.storage().temporary().set(&ALLOWANCE(&key), &new_allowance);
        }
    }
}