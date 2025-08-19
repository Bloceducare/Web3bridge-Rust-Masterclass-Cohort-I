use soroban_sdk::{Env, String};
use crate::storage::{get_name, get_symbol, get_decimals, get_total_supply};

pub fn get_token_info(env: &Env) -> (String, String, u32, i128) {
    (
        get_name(env),
        get_symbol(env),
        get_decimals(env),
        get_total_supply(env),
    )
}

pub fn validate_metadata(name: &String, symbol: &String, decimals: &u32) {
    if name.len() > 32 {
        panic!("Name too long");
    }
    
    if symbol.len() > 16 {
        panic!("Symbol too long");
    }
    
    if *decimals > 18 {
        panic!("Too many decimals");
    }
}