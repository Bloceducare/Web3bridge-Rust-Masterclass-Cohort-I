use soroban_sdk::{Address, Env};
use crate::helpers::{set_admin, get_admin};
use crate::errors::TokenError;

pub fn require_admin(env: &Env, addr: &Address) -> Result<(), TokenError> {
    let admin = get_admin(env);
    if *addr != admin {
        return Err(TokenError::Unauthorized);
    }
    Ok(()) 
}

pub fn change_admin(env: &Env, current_admin: &Address, new_admin: &Address) -> Result<(), TokenError> {
    current_admin.require_auth();
    require_admin(env, current_admin)?; 
    set_admin(env, new_admin);
    Ok(()) 
}