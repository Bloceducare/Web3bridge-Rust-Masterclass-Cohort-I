use soroban_sdk::{contracttype, Address, Env, String};

/// Storage keys for the token contract
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Token balance for a specific address
    Balance(Address),
    
    /// Allowance from one address to another with expiration
    Allowance(AllowanceDataKey),
    
    /// Token metadata
    Name,
    Symbol,
    Decimals,
    
    /// Administrative data
    Admin,
    
    /// Total supply of tokens
    TotalSupply,
    
    /// Contract initialization status
    Initialized,
}

/// Allowance data key structure
#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

/// Allowance value with expiration
#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

/// Token metadata structure
#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

impl AllowanceDataKey {
    pub fn new(from: Address, spender: Address) -> Self {
        Self { from, spender }
    }
}

impl AllowanceValue {
    pub fn new(amount: i128, expiration_ledger: u32) -> Self {
        Self {
            amount,
            expiration_ledger,
        }
    }
    
    /// Check if the allowance has expired
    pub fn is_expired(&self, env: &Env) -> bool {
        self.expiration_ledger < env.ledger().sequence()
    }
    
    /// Get the effective amount (0 if expired)
    pub fn effective_amount(&self, env: &Env) -> i128 {
        if self.is_expired(env) {
            0
        } else {
            self.amount
        }
    }
}

/// Helper functions for storage operations
pub mod storage_utils {
    use super::*;
    use soroban_sdk::{Env, Address};
    use crate::error::TokenError;

    /// Get balance for an address, returns 0 if not found
    pub fn get_balance(env: &Env, addr: &Address) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(addr.clone()))
            .unwrap_or(0)
    }

    /// Set balance for an address
    pub fn set_balance(env: &Env, addr: &Address, amount: i128) {
        env.storage()
            .instance()
            .set(&DataKey::Balance(addr.clone()), &amount);
    }

    /// Get allowance between two addresses
    pub fn get_allowance(env: &Env, from: &Address, spender: &Address) -> AllowanceValue {
        let key = AllowanceDataKey::new(from.clone(), spender.clone());
        env.storage()
            .instance()
            .get(&DataKey::Allowance(key))
            .unwrap_or(AllowanceValue::new(0, 0))
    }

    /// Set allowance between two addresses
    pub fn set_allowance(
        env: &Env,
        from: &Address,
        spender: &Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let key = AllowanceDataKey::new(from.clone(), spender.clone());
        let value = AllowanceValue::new(amount, expiration_ledger);
        
        if amount == 0 {
            env.storage().instance().remove(&DataKey::Allowance(key));
        } else {
            env.storage()
                .instance()
                .set(&DataKey::Allowance(key), &value);
        }
    }

    /// Get total supply
    pub fn get_total_supply(env: &Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Set total supply
    pub fn set_total_supply(env: &Env, amount: i128) {
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &amount);
    }

    /// Check if contract is initialized
    pub fn is_initialized(env: &Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Initialized)
            .unwrap_or(false)
    }

    /// Mark contract as initialized
    pub fn set_initialized(env: &Env) {
        env.storage()
            .instance()
            .set(&DataKey::Initialized, &true);
    }

    /// Get admin address
    pub fn get_admin(env: &Env) -> Result<Address, TokenError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(TokenError::NotInitialized)
    }

    /// Set admin address
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::Admin, admin);
    }
}
