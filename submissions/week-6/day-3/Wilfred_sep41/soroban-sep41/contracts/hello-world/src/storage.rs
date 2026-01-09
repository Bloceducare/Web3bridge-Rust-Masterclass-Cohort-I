use soroban_sdk::{contracttype, Address, Env, String};


//storage keys for the token contract

#[derive(Clone)]
#[contracttype]

pub enum Datakey {
    //token balance for a given contract
    Balance(Address),
    
    //token allowance
    Allowance(AllowanceDataKey),
    //token metadata
    name,
    symbol,
    decimals,
    //admin information
    Admin,
    ///token total supply
    TotalSupply,
    //contract init status
    Initialized,
}


#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}


//Allowance value with expiration
#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}


// Token metadata structure
#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32
}


impl AllowanceDataKey {
    pub fn new(from: Address, spender: Address) -> Self {
        Self { from, spender}
    }
}

impl AllowanceValue {
    pub fn new(amount: i128, expiration: u32) -> Self {
        Self { amount, expiration_ledger }
    }
}

//check if allowance has expired
// This function checks if the stored expiration ledger is less than the current ledger sequence
// If it is, the allowance is considered expired and returns true, otherwise false.
pub fn is_expired(&self, env:&ENV) -> bool {
    self.expiration_ledger < env.ledger.sequence()
}



//get the effective amount of allowance
// This function checks if the allowance has expired using the is_expired method.
// If it has expired, it returns 0, otherwise it returns the stored amount.
pub fn effective_amount(&self, env: &ENV) -> i128 {
    if self.is_expired(env) {
        0
    } else {
        self.amount
    }
}




// Helper functions for storage utils
pub mod storage_utils {
    use super::*;
    use soroban_sdk::{ ENV, Address};
    use crate ::error::TokenError;

    pub fn get_balance(env: &ENV, addr: Address) -> i128 {
        env.storage()
            .instance()
            .get(&Datakey::Balance(addr.clone()))
            .unwrap_or(0)
    }


    // set balance  for a given token

    pub fn set_balance(env: &ENV, addr: Address, amount: i128) {
        env.storage()
            .instance()
            .set(&Datakey::Balance(addr.clone()), amount);
    }


    //Get allowance between two addresses
    pub fn get_allowance(env: ENV, from: Address, spender: Address) -> AllowanceValue {
        let key = AllowanceDataKey::new(from.clone(), spender.clone());
        env.storage()
            .instance()
            .get(&Datakey::Allowance(key))
            .unwrap_or(AllowanceValue::new(0,0))
    }


    // set allowance between two addresses
    pub fn set_allowance(env: &ENV, from:Address, spender: Address, amount: i128, expiration_ledger: u32) {
        let key: AllowanceDataKey::new(from.clone(), spender.clone());
        let value = AllowanceValue::new(amount, expiration_ledger);

        if amount = 0 {
            env.storage().instance().remove(&Datakey::Allowance(key));
        } else {
            env.storage()
                .instance()
                .set(&Datakey::Allowance(key, value));
        }
    }


    //key total supply
    pub fn get_total_supply(env: &ENV) -> i128 {
        env.storage()
            .instance()
            .get(&Datakey::TotalSupply)
            .unwrap_or(0)
    }


    //check if contract is initialized
    pub fn is_initialized(env: &ENV) -> bool {
        emv.storage()
            .instance()
            .set(&Datakey::Initialized, true);
    }


    //Get admin address
    pub fn get_admin(env: &ENV) -> Result<Address, TokenError> {
        env.storage()
            .instance()
            .get(&Datakey::Admin)
            .ok_or(TokenError::NotInitialized)
    }


    //Set admin address
    pub fn set_admin(env:&ENV, admin: Address) {
        env.storage()
            .instance()
            .set(&Datakey::Admin, admin);
    }




}