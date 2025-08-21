use soroban_sdk::{contract, contractimpl, Env, Address, String, Symbol, log};
use crate::interface::{TokenInterface, AdminInterface};
// use sep_41::helpers;
use crate::datatypes::*;
use crate::errors::TokenError;


#[contract]
pub struct EmarcToken;

#[contractimpl]
impl EmarcToken {
    // pub fn __constructor(e: Env, owner: Address) {
    //     e.storage().persistent().set(&DataKey::Owner, &owner);
    // }

    // Helper function to read balance
    fn read_balance(env: &Env, addr: Address) -> i128 {
        let key = DataKey::Balance(addr);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
    
    // Helper function to write balance
    fn write_balance(env: &Env, addr: Address, amount: i128) {
        let key = DataKey::Balance(addr);
        if amount > 0 {
            env.storage().persistent().set(&key, &amount);
        } else {
            env.storage().persistent().remove(&key);
        }
    }
    
    // Helper function to read allowance
    fn read_allowance(env: &Env, from: Address, spender: Address) -> AllowanceValue {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        env.storage().temporary()
            .get(&key)
            .unwrap_or(AllowanceValue {
                amount: 0,
                expiration_ledger: 0,
            })
    }
    
    // Helper function to write allowance
    fn write_allowance(
        env: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let key = DataKey::Allowance(AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone(),
        });
        
        if amount > 0 && expiration_ledger > env.ledger().sequence() {
            let allowance = AllowanceValue {
                amount,
                expiration_ledger,
            };
            let live_for = (expiration_ledger - env.ledger().sequence()) as u32;
            env.storage().temporary().set(&key, &allowance);
            env.storage().temporary().extend_ttl(&key, live_for, live_for);
        } else {
            env.storage().temporary().remove(&key);
        }
    }
    
    // Helper function to spend allowance
    fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) -> Result<(), TokenError> {
        let allowance = Self::read_allowance(env, from.clone(), spender.clone());
        
        if allowance.expiration_ledger < env.ledger().sequence() {
            return Err(TokenError::InsufficientAllowance);
        }
        
        if allowance.amount < amount {
            return Err(TokenError::InsufficientAllowance);
        }
        
        let new_allowance = allowance.amount.checked_sub(amount)
            .ok_or(TokenError::Overflow)?;
        
        Self::write_allowance(
            env,
            from,
            spender,
            new_allowance,
            allowance.expiration_ledger,
        );
        
        Ok(())
    }
    
    // Helper function to check non-negative amount
    fn check_nonnegative_amount(amount: i128) -> Result<(), TokenError> {
        if amount < 0 {
            return Err(TokenError::InvalidAmount);
        }
        Ok(())
    }
    
    // Helper function to get admin
    fn get_admin(env: &Env) -> Result<Address, TokenError> {
        env.storage().instance()
            .get(&DataKey::Admin)
            .ok_or(TokenError::AdminNotSet)
    }
    
    // Helper function to get total supply
    fn read_total_supply(env: &Env) -> i128 {
        env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }
    
    // Helper function to write total supply
    fn write_total_supply(env: &Env, amount: i128) {
        env.storage().instance().set(&DataKey::TotalSupply, &amount);
    }
}


#[contractimpl]
impl AdminInterface for EmarcToken {
    fn initialize(
        env: Env,
        admin: Address,
        initial_supply: i128,
    ) -> Result<(), TokenError> {

        if env.storage().persistent().has(&DataKey::Admin) {
            return Err(TokenError::AlreadyInitialized);
        }

        admin.require_auth();

        // Set the admin
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Set initial total supply
        Self::write_total_supply(&env, initial_supply);
        
        // If initial supply > 0, mint to admin
        if initial_supply > 0 {
            Self::write_balance(&env, admin.clone(), initial_supply);
            
            // Emit mint event
            env.events().publish(
                (Symbol::new(&env, "mint"), admin.clone()),
                initial_supply
            );
        }
        
        log!(&env, "EmarcToken initialized with admin: {}", admin);

        Ok(())
    }

    fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        let admin = Self::get_admin(&env).expect("Admin not set");
        admin.require_auth();
        
        Self::check_nonnegative_amount(amount)?;
        
        let to_balance = Self::read_balance(&env, to.clone());
        let new_to_balance = to_balance.checked_add(amount)
            .expect("Balance overflow");
        
        Self::write_balance(&env, to.clone(), new_to_balance);
        
        // Update total supply
        let total = Self::read_total_supply(&env);
        let new_total = total.checked_add(amount)
            .expect("Total supply overflow");
        Self::write_total_supply(&env, new_total);
        
        // Emit mint event
        env.events().publish(
            (Symbol::new(&env, "mint"), to),
            amount
        );

        Ok(())
    }
    
    fn set_admin(env: Env, new_admin: Address) {
        let admin = Self::get_admin(&env).expect("Admin not set");
        admin.require_auth();
        
        new_admin.require_auth();
        
        env.storage().instance().set(&DataKey::Admin, &new_admin);
        
        log!(&env, "Admin changed from {} to {}", admin, new_admin);
    }
    
    fn admin(env: Env) -> Address {
        Self::get_admin(&env).expect("Admin not set")
    }

    fn total_supply(env: Env) -> i128 {
        Self::read_total_supply(&env)
    }

}

#[contractimpl]
impl TokenInterface for EmarcToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance = Self::read_allowance(&env, from, spender);
        if allowance.expiration_ledger < env.ledger().sequence() {
            0
        } else {
            allowance.amount
        }
    }
    
    fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) -> Result<(), TokenError> {
        from.require_auth();
        
        if let Err(e) = Self::check_nonnegative_amount(amount) {
            return Err(e);
        }
        
        // Check expiration ledger validity
        if amount > 0 && live_until_ledger < env.ledger().sequence() {
            return Err(TokenError::InvalidExpirationLedger);
        }
        
        Self::write_allowance(&env, from.clone(), spender.clone(), amount, live_until_ledger);
        
        // Emit approval event
        env.events().publish(
            (Symbol::new(&env, "approve"), from, spender),
            (amount, live_until_ledger)
        );

        Ok(())
    }
    
    fn balance(env: Env, id: Address) -> i128 {
        Self::read_balance(&env, id)
    }
    
    fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        from.require_auth();
        
        if let Err(e) = Self::check_nonnegative_amount(amount) {
            return Err(e);
        }
        
        let from_balance = Self::read_balance(&env, from.clone());
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        let to_balance = Self::read_balance(&env, to.clone());
        let new_to_balance = to_balance.checked_add(amount)
            .expect("Balance overflow");
        
        Self::write_balance(&env, from.clone(), from_balance - amount);
        Self::write_balance(&env, to.clone(), new_to_balance);
        
        // Emit transfer event
        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            amount
        );

        Ok(())
    }
    
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        spender.require_auth();
        
        if let Err(e) = Self::check_nonnegative_amount(amount) {
            return Err(e);
        }
        
        // Check and spend allowance
        if let Err(e) = Self::spend_allowance(&env, from.clone(), spender.clone(), amount) {
            return Err(e);
        }
        
        // Perform the transfer
        let from_balance = Self::read_balance(&env, from.clone());
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        let to_balance = Self::read_balance(&env, to.clone());
        let new_to_balance = to_balance.checked_add(amount)
            .expect("Balance overflow");
        
        Self::write_balance(&env, from.clone(), from_balance - amount);
        Self::write_balance(&env, to.clone(), new_to_balance);
        
        // Emit transfer event
        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            amount
        );

        Ok(())
    }
    
    fn burn(env: Env, from: Address, amount: i128) -> Result<(), TokenError> {
        from.require_auth();
        
        if let Err(e) = Self::check_nonnegative_amount(amount) {
            return Err(e);
        }
        
        let from_balance = Self::read_balance(&env, from.clone());
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        // Update balance
        Self::write_balance(&env, from.clone(), from_balance - amount);
        
        // Update total supply
        let total = Self::read_total_supply(&env);
        let new_total = total.checked_sub(amount)
            .expect("Total supply underflow");
        Self::write_total_supply(&env, new_total);
        
        // Emit burn event
        env.events().publish(
            (Symbol::new(&env, "burn"), from),
            amount
        );

        Ok(())
    }
    
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) -> Result<(), TokenError> {
        spender.require_auth();
        
        if let Err(e) = Self::check_nonnegative_amount(amount) {
            return Err(e);
        }
        
        // Check and spend allowance
        if let Err(e) = Self::spend_allowance(&env, from.clone(), spender.clone(), amount) {
            return Err(e);
        }
        
        let from_balance = Self::read_balance(&env, from.clone());
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        
        // Update balance
        Self::write_balance(&env, from.clone(), from_balance - amount);
        
        // Update total supply
        let total = Self::read_total_supply(&env);
        let new_total = total.checked_sub(amount)
            .expect("Total supply underflow");
        Self::write_total_supply(&env, new_total);
        
        // Emit burn event
        env.events().publish(
            (Symbol::new(&env, "burn"), from),
            amount
        );

        Ok(())
    }
    
    fn decimals(_env: Env) -> u32 {
        DECIMAL
    }
    
    fn name(env: Env) -> String {
        String::from_str(&env, NAME)
    }
    
    fn symbol(env: Env) -> String {
        String::from_str(&env, SYMBOL)
    }

}
