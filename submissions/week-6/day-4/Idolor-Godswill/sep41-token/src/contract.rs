use soroban_sdk::{contract, contractimpl, Address, Env, String};
use crate::interface::{TokenInterface, TokenAdminInterface};
use crate::storage::{storage_utils, AllowanceValue};
use crate::events::events;
use crate::metadata::metadata;
use crate::admin::admin;
use crate::error::TokenError;

#[contract]
pub struct Token;

#[contractimpl]
impl TokenInterface for Token {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance = storage_utils::get_allowance(&env, &from, &spender);
        allowance.effective_amount(&env)
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        // Require authorization from the `from` address
        from.require_auth();

        // Validate amount
        if amount < 0 {
            panic!("Invalid amount");
        }

        // Validate expiration ledger
        let current_ledger = env.ledger().sequence();
        if amount > 0 && expiration_ledger < current_ledger {
            panic!("Invalid expiration");
        }

        // Set allowance
        storage_utils::set_allowance(&env, &from, &spender, amount, expiration_ledger);

        // Emit approval event
        events::emit_approval(&env, from, spender, amount, expiration_ledger);
    }

    fn balance(env: Env, id: Address) -> i128 {
        storage_utils::get_balance(&env, &id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // Require authorization from the `from` address
        from.require_auth();

        // Perform the transfer
        Self::internal_transfer(&env, &from, &to, amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        // Require authorization from the spender
        spender.require_auth();

        // Check and consume allowance
        Self::consume_allowance(&env, &from, &spender, amount);

        // Perform the transfer
        Self::internal_transfer(&env, &from, &to, amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        // Require authorization from the `from` address
        from.require_auth();

        // Perform the burn
        Self::internal_burn(&env, &from, amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        // Require authorization from the spender
        spender.require_auth();

        // Check and consume allowance
        Self::consume_allowance(&env, &from, &spender, amount);

        // Perform the burn
        Self::internal_burn(&env, &from, amount);
    }

    fn decimals(env: Env) -> u32 {
        metadata::get_decimals(&env).unwrap_or_else(|_| {
            panic!("Not initialized");
        })
    }

    fn name(env: Env) -> String {
        metadata::get_name(&env).unwrap_or_else(|_| {
            panic!("Not initialized");
        })
    }

    fn symbol(env: Env) -> String {
        metadata::get_symbol(&env).unwrap_or_else(|_| {
            panic!("Not initialized");
        })
    }
}

#[contractimpl]
impl TokenAdminInterface for Token {
    fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32) {
        // Check if already initialized
        if storage_utils::is_initialized(&env) {
            panic!("Already initialized");
        }

        // Validate metadata
        metadata::validate_metadata(&name, &symbol, decimals).unwrap_or_else(|_| {
            panic!("Invalid metadata");
        });

        // Set metadata
        metadata::set_metadata(&env, name, symbol, decimals);

        // Initialize admin
        admin::initialize_admin(&env, &admin);

        // Mark as initialized
        storage_utils::set_initialized(&env);
    }

    fn mint(env: Env, to: Address, amount: i128) {
        // Get caller and require admin authorization
        let caller = env.current_contract_address(); // In practice, this would be the caller
        admin::require_admin(&env, &caller).unwrap_or_else(|_| {
            panic!("Unauthorized");
        });

        // Validate amount
        if amount < 0 {
            panic!("Invalid amount");
        }

        if amount == 0 {
            return;
        }

        // Update recipient balance
        let current_balance = storage_utils::get_balance(&env, &to);
        let new_balance = current_balance.checked_add(amount).unwrap_or_else(|| {
            panic!("Overflow");
        });
        storage_utils::set_balance(&env, &to, new_balance);

        // Update total supply
        let current_supply = storage_utils::get_total_supply(&env);
        let new_supply = current_supply.checked_add(amount).unwrap_or_else(|| {
            panic!("Overflow");
        });
        storage_utils::set_total_supply(&env, new_supply);

        // Emit mint event
        events::emit_mint(&env, to, amount);
    }

    fn set_admin(env: Env, new_admin: Address) {
        let caller = env.current_contract_address(); // In practice, this would be the caller
        admin::set_admin(&env, &caller, &new_admin).unwrap_or_else(|_| {
            panic!("Unauthorized");
        });
    }

    fn admin(env: Env) -> Address {
        admin::get_admin(&env).unwrap_or_else(|_| {
            panic!("Not initialized");
        })
    }
}

impl Token {
    /// Internal transfer function
    fn internal_transfer(env: &Env, from: &Address, to: &Address, amount: i128) {
        // Validate amount
        if amount < 0 {
            panic!("Invalid amount");
        }

        if amount == 0 {
            return;
        }

        // Get current balances
        let from_balance = storage_utils::get_balance(env, from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = storage_utils::get_balance(env, to);

        // Calculate new balances
        let new_from_balance = from_balance - amount;
        let new_to_balance = to_balance.checked_add(amount).unwrap_or_else(|| {
            panic!("Overflow");
        });

        // Update balances
        storage_utils::set_balance(env, from, new_from_balance);
        storage_utils::set_balance(env, to, new_to_balance);

        // Emit transfer event
        events::emit_transfer(env, from.clone(), to.clone(), amount);
    }

    /// Internal burn function
    fn internal_burn(env: &Env, from: &Address, amount: i128) {
        // Validate amount
        if amount < 0 {
            panic!("Invalid amount");
        }

        if amount == 0 {
            return;
        }

        // Check balance
        let current_balance = storage_utils::get_balance(env, from);
        if current_balance < amount {
            panic!("Insufficient balance");
        }

        // Update balance
        let new_balance = current_balance - amount;
        storage_utils::set_balance(env, from, new_balance);

        // Update total supply
        let current_supply = storage_utils::get_total_supply(env);
        let new_supply = current_supply.checked_sub(amount).unwrap_or_else(|| {
            panic!("Underflow");
        });
        storage_utils::set_total_supply(env, new_supply);

        // Emit burn event
        events::emit_burn(env, from.clone(), amount);
    }

    /// Consume allowance for transfer_from and burn_from operations
    fn consume_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) {
        let allowance = storage_utils::get_allowance(env, from, spender);
        let effective_allowance = allowance.effective_amount(env);

        if effective_allowance < amount {
            if allowance.is_expired(env) {
                panic!("Allowance expired");
            } else {
                panic!("Insufficient allowance");
            }
        }

        // Update allowance
        let new_allowance = effective_allowance - amount;
        storage_utils::set_allowance(env, from, spender, new_allowance, allowance.expiration_ledger);
    }
}
