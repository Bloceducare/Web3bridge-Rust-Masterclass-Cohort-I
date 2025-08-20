use soroban_sdk::{Address, Env};
use crate::helper::{get_balance, set_balance, get_frozen};
use crate::errors::TokenError;

pub fn transfer_balance(env: &Env, from: &Address, to: &Address, amount: &i128) -> Result<(), TokenError> {
    if *amount <= 0 {
        return Err(TokenError::InvalidAmount); 
    }
    
    if get_frozen(env, from) {
        return Err(TokenError::AccountFrozen);
    }
    
    if get_frozen(env, to) {
        return Err(TokenError::AccountFrozen);
    }
    
    let from_balance = get_balance(env, from);
    if from_balance < *amount {
        return Err(TokenError::InsufficientBalance);
    }
    
    set_balance(env, from, &(from_balance - amount));
    
    let to_balance = get_balance(env, to);
    let new_balance = to_balance.checked_add(*amount)
        .ok_or(TokenError::BalanceOverflow)?; 
    set_balance(env, to, &new_balance);
    
    Ok(()) 
}