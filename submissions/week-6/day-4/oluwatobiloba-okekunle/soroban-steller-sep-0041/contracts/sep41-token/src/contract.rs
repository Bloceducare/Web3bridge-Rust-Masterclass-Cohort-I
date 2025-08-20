use soroban_sdk::{contract, contractimpl, contractmeta, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;

use crate::{admin, allowance, balance, error::TokenError, events, metadata, storage};

// Contract metadata
contractmeta!(key = "Description", val = "SEP-41 compliant token contract");

#[contract]
pub struct SepToken;

#[contractimpl]
impl SepToken {
    /// Initialize the token with metadata and admin
    pub fn initialize(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
    ) -> Result<(), TokenError> {
        if admin::has_administrator(&env) {
            return Err(TokenError::AlreadyInitialized);
        }

        admin.require_auth();

        // Write metadata using token SDK
        let metadata = TokenMetadata {
            decimal,
            name: name.clone(),
            symbol: symbol.clone(),
        };

        metadata::write_metadata(&env, metadata);
        admin::write_administrator(&env, &admin);
        storage::set_total_supply(&env, 0);
        storage::extend_instance(&env);

        Ok(())
    }

    /// Check if the contract has an administrator (useful for frontends)
    pub fn has_admin(env: Env) -> bool {
        admin::has_administrator(&env)
    }

    /// Get current administrator (only if one exists)
    pub fn admin(env: Env) -> Result<Address, TokenError> {
        if !admin::has_administrator(&env) {
            return Err(TokenError::NotInitialized);
        }
        Ok(admin::read_administrator(&env))
    }

    /// Mint new tokens (admin only)
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        let admin = admin::read_administrator(&env);
        admin.require_auth();
        admin::check_admin(&env, &admin)?;

        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        let total_supply = storage::get_total_supply(&env);
        let new_total_supply = total_supply
            .checked_add(amount)
            .ok_or(TokenError::SupplyOverflow)?;

        balance::receive_balance(&env, &to, amount)?;
        storage::set_total_supply(&env, new_total_supply);
        events::mint_event(&env, admin, to, amount);
        storage::extend_instance(&env);

        Ok(())
    }

    /// Clawback tokens (admin only)
    pub fn clawback(env: Env, from: Address, amount: i128) -> Result<(), TokenError> {
        let admin = admin::read_administrator(&env);
        admin.require_auth();
        admin::check_admin(&env, &admin)?;

        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }

        balance::spend_balance(&env, &from, amount)?;

        let total_supply = storage::get_total_supply(&env);
        storage::set_total_supply(&env, total_supply - amount);

        events::clawback_event(&env, admin, from, amount);
        storage::extend_instance(&env);

        Ok(())
    }

    /// Set a new administrator (admin only)
    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), TokenError> {
        admin::set_admin(&env, &new_admin)
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        storage::get_total_supply(&env)
    }
}

// SEP-41 Token Interface Implementation
use soroban_sdk::token::Interface as TokenInterface;

#[contractimpl]
impl TokenInterface for SepToken {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        storage::extend_instance(&env);
        allowance::read_allowance(&env, &from, &spender)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        allowance::write_allowance(&env, &from, &spender, amount, expiration_ledger)
            .expect("Failed to write allowance");

        events::approve_event(&env, from, spender, amount, expiration_ledger);
        storage::extend_instance(&env);
    }

    fn balance(env: Env, id: Address) -> i128 {
        storage::extend_instance(&env);
        balance::read_balance(&env, &id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        balance::transfer_balance(&env, &from, &to, amount).expect("Transfer failed");

        events::transfer_event(&env, from, to, amount);
        storage::extend_instance(&env);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        allowance::spend_allowance(&env, &from, &spender, amount).expect("Insufficient allowance");
        balance::transfer_balance(&env, &from, &to, amount).expect("Transfer failed");

        events::transfer_event(&env, from, to, amount);
        storage::extend_instance(&env);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        balance::spend_balance(&env, &from, amount).expect("Insufficient balance");

        let total_supply = storage::get_total_supply(&env);
        storage::set_total_supply(&env, total_supply - amount);

        events::burn_event(&env, from, amount);
        storage::extend_instance(&env);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        allowance::spend_allowance(&env, &from, &spender, amount).expect("Insufficient allowance");
        balance::spend_balance(&env, &from, amount).expect("Insufficient balance");

        let total_supply = storage::get_total_supply(&env);
        storage::set_total_supply(&env, total_supply - amount);

        events::burn_event(&env, from, amount);
        storage::extend_instance(&env);
    }

    fn decimals(env: Env) -> u32 {
        storage::extend_instance(&env);
        metadata::read_decimal(&env)
    }

    fn name(env: Env) -> String {
        storage::extend_instance(&env);
        metadata::read_name(&env)
    }

    fn symbol(env: Env) -> String {
        storage::extend_instance(&env);
        metadata::read_symbol(&env)
    }
}
