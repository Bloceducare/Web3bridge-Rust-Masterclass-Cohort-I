use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn has_administrator(env: &Env) -> bool {
    let key = DataKey::Admin;
    env.storage().instance().has(&key)
}

pub fn read_administrator(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(env: &Env, id: &Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, id);
}