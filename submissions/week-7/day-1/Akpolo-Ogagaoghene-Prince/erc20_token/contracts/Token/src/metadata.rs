use crate::contract::TokenMetadata;
use crate::storage_types::DataKey;
use soroban_sdk::{Env, String};

pub fn read_decimal(env: &Env) -> u32 {
    let key = DataKey::TokenMetadata;
    env.storage().instance().get::<DataKey, TokenMetadata>(&key).unwrap().decimal
}

pub fn read_name(env: &Env) -> String {
    let key = DataKey::TokenMetadata;
    env.storage().instance().get::<DataKey, TokenMetadata>(&key).unwrap().name
}

pub fn read_symbol(env: &Env) -> String {
    let key = DataKey::TokenMetadata;
    env.storage().instance().get::<DataKey, TokenMetadata>(&key).unwrap().symbol
}

pub fn write_metadata(env: &Env, metadata: TokenMetadata) {
    let key = DataKey::TokenMetadata;
    env.storage().instance().set(&key, &metadata);
}