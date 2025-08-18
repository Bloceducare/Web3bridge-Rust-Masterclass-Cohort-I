use soroban_sdk::{Env, String};

use crate::storage::{TOKEN_NAME, TOKEN_SYMBOL, TOKEN_DECIMALS};

pub struct TokenMetadata;

impl TokenMetadata {
    pub fn set_metadata(env: &Env, name: String, symbol: String, decimals: u32) {
        env.storage().instance().set(&TOKEN_NAME, &name);
        env.storage().instance().set(&TOKEN_SYMBOL, &symbol);
        env.storage().instance().set(&TOKEN_DECIMALS, &decimals);
    }

    pub fn get_name(env: &Env) -> String {
        env.storage()
            .instance()
            .get(&TOKEN_NAME)
            .unwrap_or_else(|| String::from_str(env, "Unknown Token"))
    }

    pub fn get_symbol(env: &Env) -> String {
        env.storage()
            .instance()
            .get(&TOKEN_SYMBOL)
            .unwrap_or_else(|| String::from_str(env, "UNK"))
    }

    pub fn get_decimals(env: &Env) -> u32 {
        env.storage()
            .instance()
            .get(&TOKEN_DECIMALS)
            .unwrap_or(7)
    }

    pub fn update_name(env: &Env, new_name: String) {
        env.storage().instance().set(&TOKEN_NAME, &new_name);
    }

    pub fn update_symbol(env: &Env, new_symbol: String) {
        env.storage().instance().set(&TOKEN_SYMBOL, &new_symbol);
    }

    pub fn metadata_exists(env: &Env) -> bool {
        env.storage().instance().has(&TOKEN_NAME) &&
        env.storage().instance().has(&TOKEN_SYMBOL) &&
        env.storage().instance().has(&TOKEN_DECIMALS)
    }
}