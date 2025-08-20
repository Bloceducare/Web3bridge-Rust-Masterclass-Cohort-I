#![no_std]
use soroban_sdk::{contract, contractimpl, contractmeta, panic_with_error, Address, Env, String};

pub mod admin;
pub mod allowance;
pub mod balance;
pub mod metadata;
pub mod helpers;
pub mod types;
pub mod traits;
pub mod errors;

pub use admin::*;
pub use allowance::*;
pub use balance::*;
pub use metadata::*;
pub use helpers::*;
pub use types::*;
pub use traits::*;
pub use errors::*;

contractmeta!(
    key = "Description",
    val = "SEP-41 Compliant Fungible Token Contract"
);

contractmeta!(
    key = "version",
    val = "1.0.0"
);

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
        total_supply: i128,
    ) -> Result<(), TokenError> {
        if has_admin(&env) {
            return Err(TokenError::AlreadyInitialized);
        }

        admin.require_auth();

        if name.len() > 32 {
            return Err(TokenError::NameTooLong);
        }
        
        if symbol.len() > 16 {
            return Err(TokenError::SymbolTooLong);
        }

        if decimals > 18 {
            return Err(TokenError::DecimalsTooHigh);
        }
        
        if total_supply < 0 {
            return Err(TokenError::InvalidTotalSupply);
        }

        set_admin(&env, &admin);
        set_name(&env, &name);
        set_symbol(&env, &symbol);
        set_decimals(&env, &decimals);
        set_total_supply(&env, &total_supply);

        if total_supply > 0 {
            set_balance(&env, &admin, &total_supply);
        }
        
        Ok(())
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        from.require_auth();
        Self::transfer_impl(&env, &from, &to, &amount)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        spender.require_auth();

        let allowance = get_allowance(&env, &from, &spender);
        if allowance < amount {
            return Err(TokenError::InsufficientAllowance);
        }

        set_allowance(&env, &from, &spender, &(allowance - amount));
        Self::transfer_impl(&env, &from, &to, &amount)
    }

    fn transfer_impl(env: &Env, from: &Address, to: &Address, amount: &i128) -> Result<(), TokenError> {
        if *amount < 0 {
            return Err(TokenError::InvalidAmount);
        }

        if *amount == 0 {
            return Ok(());
        }

        if from == to {
            return Ok(());
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
        let new_to_balance = to_balance.checked_add(*amount)
            .ok_or(TokenError::BalanceOverflow)?;
        set_balance(env, to, &new_to_balance);
        
        Ok(())
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        get_balance(&env, &id)
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> Result<(), TokenError> {
        from.require_auth();

        if amount < 0 {
            return Err(TokenError::InvalidAmount);
        }

        let current_ledger = env.ledger().sequence();
        if expiration_ledger <= current_ledger {
            return Err(TokenError::InvalidExpiration);
        }

        set_allowance(&env, &from, &spender, &amount);
        set_allowance_expiration(&env, &from, &spender, &expiration_ledger);
        
        Ok(())
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let expiration = get_allowance_expiration(&env, &from, &spender);
        let current_ledger = env.ledger().sequence();
        
        if expiration <= current_ledger {
            return 0;
        }

        get_allowance(&env, &from, &spender)
    }

    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        if !has_admin(&env) {
            return Err(TokenError::NotInitialized);
        }
        
        let admin = get_admin(&env);
        admin.require_auth();

        if amount < 0 {
            return Err(TokenError::InvalidAmount);
        }

        if amount == 0 {
            return Ok(());
        }

        if get_frozen(&env, &to) {
            return Err(TokenError::AccountFrozen);
        }

        let to_balance = get_balance(&env, &to);
        let new_balance = to_balance.checked_add(amount)
            .ok_or(TokenError::BalanceOverflow)?;
        
        set_balance(&env, &to, &new_balance);

        let total_supply = get_total_supply(&env);
        let new_supply = total_supply.checked_add(amount)
            .ok_or(TokenError::SupplyOverflow)?;
        
        set_total_supply(&env, &new_supply);
        
        Ok(())
    }

    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), TokenError> {
        from.require_auth();
        Self::burn_impl(&env, &from, &amount)
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) -> Result<(), TokenError> {
        spender.require_auth();

        let allowance = get_allowance(&env, &from, &spender);
        if allowance < amount {
            return Err(TokenError::InsufficientAllowance);
        }

        set_allowance(&env, &from, &spender, &(allowance - amount));
        Self::burn_impl(&env, &from, &amount)
    }

    fn burn_impl(env: &Env, from: &Address, amount: &i128) -> Result<(), TokenError> {
        if *amount < 0 {
            return Err(TokenError::InvalidAmount);
        }

        if *amount == 0 {
            return Ok(());
        }

        let from_balance = get_balance(env, from);
        if from_balance < *amount {
            return Err(TokenError::InsufficientBalance);
        }

        set_balance(env, from, &(from_balance - amount));

        let total_supply = get_total_supply(env);
        set_total_supply(env, &(total_supply - amount));
        
        Ok(())
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), TokenError> {
        if !has_admin(&env) {
            return Err(TokenError::NotInitialized);
        }
        
        let admin = get_admin(&env);
        admin.require_auth();
        set_admin(&env, &new_admin);
        
        Ok(())
    }

    pub fn admin(env: Env) -> Address {
        if !has_admin(&env) {
            panic_with_error!(&env, TokenError::NotInitialized);
        }
        
        get_admin(&env)
    }

    pub fn name(env: Env) -> String {
        get_name(&env)
    }

    pub fn symbol(env: Env) -> String {
        get_symbol(&env)
    }

    pub fn decimals(env: Env) -> u32 {
        get_decimals(&env)
    }

    pub fn total_supply(env: Env) -> i128 {
        get_total_supply(&env)
    }

    pub fn freeze_account(env: Env, account: Address) {
        if !has_admin(&env) {
            panic_with_error!(&env, TokenError::NotInitialized);
        }
        
        let admin = get_admin(&env);
        admin.require_auth();
        set_frozen(&env, &account, &true);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        if !has_admin(&env) {
            panic_with_error!(&env, TokenError::NotInitialized);
        }
        
        let admin = get_admin(&env);
        admin.require_auth();
        set_frozen(&env, &account, &false);
    }

    pub fn is_frozen(env: Env, account: Address) -> bool {
        get_frozen(&env, &account)
    }
}

