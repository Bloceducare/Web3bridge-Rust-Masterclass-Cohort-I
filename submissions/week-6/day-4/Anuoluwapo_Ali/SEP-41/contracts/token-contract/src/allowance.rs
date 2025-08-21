use soroban_sdk::{Address, Env};
use crate::helpers::{get_allowance, set_allowance, get_allowance_expiration, set_allowance_expiration};
use crate::errors::TokenError;

pub fn approve_allowance(
    env: &Env,
    owner: &Address,
    spender: &Address,
    amount: &i128,
    expiration_ledger: &u32,
) -> Result<(), TokenError> {
    owner.require_auth();
    
    if *amount < 0 {
        return Err(TokenError::InvalidAmount); 
    }
    
    let current_ledger = env.ledger().sequence();
    if *expiration_ledger <= current_ledger {
        return Err(TokenError::InvalidExpiration);
    }
    
    set_allowance(env, owner, spender, amount);
    set_allowance_expiration(env, owner, spender, expiration_ledger);
    
    Ok(()) 
}

pub fn check_allowance(env: &Env, owner: &Address, spender: &Address) -> i128 {
    let expiration = get_allowance_expiration(env, owner, spender);
    let current_ledger = env.ledger().sequence();
    
    if expiration <= current_ledger {
        return 0;
    }
    
    get_allowance(env, owner, spender)
}