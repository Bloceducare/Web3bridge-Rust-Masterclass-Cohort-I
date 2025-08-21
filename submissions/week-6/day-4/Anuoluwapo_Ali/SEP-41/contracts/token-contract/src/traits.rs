use soroban_sdk::{Address, Env};
use crate::helpers::get_admin;
use crate::errors::TokenError;

pub trait AdminCheck {
    fn require_admin(&self, env: &Env); 
}

impl AdminCheck for Address {
    fn require_admin(&self, env: &Env) { 
        let admin = get_admin(env);
        if *self != admin {
            panic!("Unauthorized");
        }
    }
}

pub trait TokenValidation {
    fn validate_positive_amount(&self); 
    fn validate_non_zero(&self); 
}

impl TokenValidation for i128 {
    fn validate_positive_amount(&self) { 
        if *self < 0 {
            panic!("Invalid amount"); 
        }
    }
    
    fn validate_non_zero(&self) { 
        if *self == 0 {
            panic!("Balance underflow"); 
        }
    }
}

pub fn require_admin_result(addr: &Address, env: &Env) -> Result<(), TokenError> {
    let admin = get_admin(env);
    if *addr != admin {
        return Err(TokenError::Unauthorized);
    }
    Ok(())
}

pub fn validate_positive_amount_result(amount: &i128) -> Result<(), TokenError> {
    if *amount < 0 {
        return Err(TokenError::InvalidAmount);
    }
    Ok(())
}

pub fn validate_non_zero_result(amount: &i128) -> Result<(), TokenError> {
    if *amount == 0 {
        return Err(TokenError::BalanceUnderflow);
    }
    Ok(())
}