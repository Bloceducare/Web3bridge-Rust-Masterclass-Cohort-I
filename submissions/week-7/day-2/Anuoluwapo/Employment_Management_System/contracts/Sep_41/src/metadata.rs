use soroban_sdk::{Env, String};
use crate::helper::{get_name, get_symbol, get_decimals, get_total_supply};
use crate::errors::TokenError;

pub fn get_token_info(env: &Env) -> (String, String, u32, i128) {
    (
        get_name(env),
        get_symbol(env),
        get_decimals(env),
        get_total_supply(env),
    )
}

pub fn validate_metadata(name: &String, symbol: &String, decimals: &u32) -> Result<(), TokenError> {
    if name.len() > 32 {
        return Err(TokenError::NameTooLong);
    }
    
    if symbol.len() > 16 {
        return Err(TokenError::SymbolTooLong);
    }
    
    if *decimals > 18 {
        return Err(TokenError::DecimalsTooHigh); 
    }
    
    Ok(())
} 