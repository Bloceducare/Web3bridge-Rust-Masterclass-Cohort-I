use soroban_sdk::{Env, String};
use crate::storage::{DataKey, TokenMetadata};
use crate::error::TokenError;

/// Metadata management functions
pub mod metadata {
    use super::*;

    /// Set token metadata during initialization
    pub fn set_metadata(env: &Env, name: String, symbol: String, decimals: u32) {
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
        env.storage().instance().set(&DataKey::Decimals, &decimals);
    }

    /// Get token name
    pub fn get_name(env: &Env) -> Result<String, TokenError> {
        env.storage()
            .instance()
            .get(&DataKey::Name)
            .ok_or(TokenError::NotInitialized)
    }

    /// Get token symbol
    pub fn get_symbol(env: &Env) -> Result<String, TokenError> {
        env.storage()
            .instance()
            .get(&DataKey::Symbol)
            .ok_or(TokenError::NotInitialized)
    }

    /// Get token decimals
    pub fn get_decimals(env: &Env) -> Result<u32, TokenError> {
        env.storage()
            .instance()
            .get(&DataKey::Decimals)
            .ok_or(TokenError::NotInitialized)
    }

    /// Validate metadata parameters
    pub fn validate_metadata(name: &String, symbol: &String, decimals: u32) -> Result<(), TokenError> {
        // Check name is not empty
        if name.len() == 0 {
            return Err(TokenError::InvalidAmount);
        }

        // Check symbol is not empty
        if symbol.len() == 0 {
            return Err(TokenError::InvalidAmount);
        }

        // Check decimals is reasonable (0-18 is typical)
        if decimals > 18 {
            return Err(TokenError::InvalidAmount);
        }

        Ok(())
    }
}
