use soroban_sdk::{Address, Env, Symbol, symbol_short};

pub const TOTAL_SUPPLY: Symbol = symbol_short!("total");

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&Symbol::new(env, "admin"), admin);
}

pub fn get_balance(env: &Env, addr: &Address) -> i128 {
    env.storage().persistent().get(addr).unwrap_or(0)
}

pub fn set_balance(env: &Env, addr: &Address, amount: i128) {
    env.storage().persistent().set(addr, &amount);
}

pub fn get_total_supply(env: &Env) -> i128 {
    env.storage().persistent().get(&TOTAL_SUPPLY).unwrap_or(0)
}

pub fn set_total_supply(env: &Env, amount: i128) {
    env.storage().persistent().set(&TOTAL_SUPPLY, &amount);
}

pub fn get_allowance(env: &Env, owner: &Address, spender: &Address) -> i128 {
    let key = (owner.clone(), spender.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn set_allowance(env: &Env, owner: &Address, spender: &Address, amount: i128) {
    let key = (owner.clone(), spender.clone());
    env.storage().persistent().set(&key, &amount);
}
