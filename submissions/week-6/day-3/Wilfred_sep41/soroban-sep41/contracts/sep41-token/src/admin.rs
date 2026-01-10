use soroban_sdk::{Address, Env};
use crate::error::TokenError;
use crate::storage::storage_utils;

/// Administrative functions for the token contract
pub mod admin {
    use super::*;

    /// Check if the caller is the admin
    pub fn require_admin(env: &Env, caller: &Address) -> Result<(), TokenError> {
        let admin = storage_utils::get_admin(env)?;
        if admin != *caller {
            return Err(TokenError::Unauthorized);
        }
        Ok(())
    }

    /// Set a new admin (only callable by current admin)
    pub fn set_admin(env: &Env, caller: &Address, new_admin: &Address) -> Result<(), TokenError> {
        require_admin(env, caller)?;
        storage_utils::set_admin(env, new_admin);
        Ok(())
    }

    /// Get the current admin address
    pub fn get_admin(env: &Env) -> Result<Address, TokenError> {
        storage_utils::get_admin(env)
    }

    /// Initialize admin during contract initialization
    pub fn initialize_admin(env: &Env, admin: &Address) {
        storage_utils::set_admin(env, admin);
    }
}