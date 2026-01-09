use soroban_sdk::{contract, contractimpl, Address, Env, String};
use crate::interface::{TokenInterface, TokenAdminInterface};
use crate::storage_utils::{storage_utils, AllowanceValue}
use crate::events::events;
use crate:metadata::metadata;
use crate::admin::admin;
use crate::error::TokenError;


#[contract]

pub struct Token;

#[contracimpl]

impl TokenInterface for Token {
    fn allowance(env: &ENV, from: Address, spender: Address) -> i128 {
        let allowance = storage_utils::get_allowance(env, from, spender);
        allowance.effective_amount(&env)
    }


    fn approve(env: &ENV, from: Address, spender:Address, amount: i128, expiration_ledger:u32) {
        //req auth from owner
        from.require_auth();

        //check if amount is valid
        if amount < o {
            panic!(TokenError::InvalidAmount);
        }

        //check if expiration is valid
        if expiration_ledger < env.ledger.sequence();
        if amount > 0 && expiration_ledger < env.ledger.sequence(){
            panic!(TokenError::InvalidExpiration)
        }


        //set allowance
        storage_utils::set_allowance(&env, &from, amount, &spender, expiration_ledger);

        //emit event
        events::emit_approval(&env, &from, &spender, amount, expiration_ledger)
    }


    fn balance(env: Env, id: Address) -> i128 {
        storage_utils::get_balance(&env, id)
    }


    fn transfer(env: Env, from: Address, to: Address, amount: i128){
        //req authentication from sender
        from.require_auth();


        //Make the transfer
        Self::internal_transfer(&env, &from, &to, amount)
    }

    fn burn(env: Env, from: Address, amount: i128){
        //require auth from the from address
        from.require_auth();

        //check and consume allowance
        Self::consume_allowance(&env, &from, amount);

        //perform burn
        Self::internal_burn(&env, &from, amount);
    }

    fn decimals(env: Env) -> u32 {
        metadata::get_decimals(&env).unwrap_or_else(|_| {
            panic!(TokenError::NotInitialized)
        })
    }

    fn name(env: Env) -> String {
        metadata::get_name(&env).unwrap_or_else(|_|) {
            panic!(TokenError::NotInitialized)
        }
    }

    fn symbol(env: Env) -> String {
        metadata::get_symbol(&env).unwrap_or_else(|_| {
            panic!(TokenError::NotInitialized)
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
