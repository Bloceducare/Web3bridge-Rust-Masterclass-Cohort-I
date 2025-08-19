use soroban_sdk::{Address, Env, String};
use crate::types::DataKey;

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_balance(env: &Env, addr: &Address, amount: &i128) {
    let key = DataKey::Balance(addr.clone());
    if *amount == 0 {
        env.storage().persistent().remove(&key);
    } else {
        env.storage().persistent().set(&key, amount);
        env.storage().persistent().extend_ttl(&key, 100, 100);
    }
}

pub fn get_balance(env: &Env, addr: &Address) -> i128 {
    let key = DataKey::Balance(addr.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_allowance(env: &Env, owner: &Address, spender: &Address, amount: &i128) {
    let key = DataKey::Allowance(owner.clone(), spender.clone());
    if *amount == 0 {
        env.storage().persistent().remove(&key);
    } else {
        env.storage().persistent().set(&key, amount);
        env.storage().persistent().extend_ttl(&key, 100, 100);
    }
}

pub fn get_allowance(env: &Env, owner: &Address, spender: &Address) -> i128 {
    let key = DataKey::Allowance(owner.clone(), spender.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_allowance_expiration(env: &Env, owner: &Address, spender: &Address, expiration: &u32) {
    let key = DataKey::AllowanceExpiration(owner.clone(), spender.clone());
    env.storage().persistent().set(&key, expiration);
    env.storage().persistent().extend_ttl(&key, 100, 100);
}

pub fn get_allowance_expiration(env: &Env, owner: &Address, spender: &Address) -> u32 {
    let key = DataKey::AllowanceExpiration(owner.clone(), spender.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_frozen(env: &Env, addr: &Address, frozen: &bool) {
    let key = DataKey::Frozen(addr.clone());
    if *frozen {
        env.storage().persistent().set(&key, frozen);
        env.storage().persistent().extend_ttl(&key, 100, 100);
    } else {
        env.storage().persistent().remove(&key);
    }
}

pub fn get_frozen(env: &Env, addr: &Address) -> bool {
    let key = DataKey::Frozen(addr.clone());
    env.storage().persistent().get(&key).unwrap_or(false)
}

pub fn set_name(env: &Env, name: &String) {
    env.storage().instance().set(&DataKey::Name, name);
}

pub fn get_name(env: &Env) -> String {
    env.storage().instance().get(&DataKey::Name).unwrap()
}

pub fn set_symbol(env: &Env, symbol: &String) {
    env.storage().instance().set(&DataKey::Symbol, symbol);
}

pub fn get_symbol(env: &Env) -> String {
    env.storage().instance().get(&DataKey::Symbol).unwrap()
}

pub fn set_decimals(env: &Env, decimals: &u32) {
    env.storage().instance().set(&DataKey::Decimals, decimals);
}

pub fn get_decimals(env: &Env) -> u32 {
    env.storage().instance().get(&DataKey::Decimals).unwrap()
}

pub fn set_total_supply(env: &Env, total_supply: &i128) {
    env.storage().instance().set(&DataKey::TotalSupply, total_supply);
}

pub fn get_total_supply(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::TotalSupply).unwrap()
}
