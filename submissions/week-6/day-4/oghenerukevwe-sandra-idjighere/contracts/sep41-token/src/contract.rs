use crate::interface::{TokenInterface, TokenAdminInterface};
use crate::storage::{
    is_initialized, read_admin, read_allowance, read_balance, read_token_info, set_initialized,
    spend_allowance, transfer_balance, write_admin, write_allowance, write_balance, write_token_info,
    TokenMetadata,
};
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct Token;

#[contractimpl]
impl TokenInterface for Token {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }
        read_allowance(&env, &from, &spender).amount
    }

    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        // Require authorization from the `from` address
        from.require_auth();

        // Validate expiration ledger
        let current_ledger = env.ledger().sequence();
        if amount > 0 && expiration_ledger < current_ledger {
            panic!("expiration_ledger is in the past");
        }

        // Write the allowance
        write_allowance(&env, &from, &spender, amount, expiration_ledger);

        // Emit approval event
        env.events().publish(
            ("approve", &from, &spender),
            (amount, expiration_ledger),
        );
    }

    fn balance(env: Env, id: Address) -> i128 {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }
        read_balance(&env, &id)
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        // Require authorization from the `from` address
        from.require_auth();

        // Validate amount
        if amount < 0 {
            panic!("negative amount");
        }

        if amount == 0 {
            return; // No-op for zero transfers
        }

        // Perform the transfer
        transfer_balance(&env, &from, &to, amount);

        // Emit transfer event
        env.events().publish(("transfer", &from, &to), amount);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        // Require authorization from the spender
        spender.require_auth();

        // Validate amount
        if amount < 0 {
            panic!("negative amount");
        }

        if amount == 0 {
            return; // No-op for zero transfers
        }

        // Spend the allowance
        spend_allowance(&env, &from, &spender, amount);

        // Perform the transfer
        transfer_balance(&env, &from, &to, amount);

        // Emit transfer event
        env.events().publish(("transfer", &from, &to), amount);
    }

    fn burn(env: Env, from: Address, amount: i128) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        // Require authorization from the `from` address
        from.require_auth();

        // Validate amount
        if amount < 0 {
            panic!("negative amount");
        }

        if amount == 0 {
            return; // No-op for zero burns
        }

        // Check balance and burn
        let balance = read_balance(&env, &from);
        if balance < amount {
            panic!("insufficient balance");
        }

        write_balance(&env, &from, balance - amount);

        // Emit burn event
        env.events().publish(("burn", &from), amount);
    }

    fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        // Require authorization from the spender
        spender.require_auth();

        // Validate amount
        if amount < 0 {
            panic!("negative amount");
        }

        if amount == 0 {
            return; // No-op for zero burns
        }

        // Spend the allowance
        spend_allowance(&env, &from, &spender, amount);

        // Check balance and burn
        let balance = read_balance(&env, &from);
        if balance < amount {
            panic!("insufficient balance");
        }

        write_balance(&env, &from, balance - amount);

        // Emit burn event
        env.events().publish(("burn", &from), amount);
    }

    fn decimals(env: Env) -> u32 {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }
        read_token_info(&env).decimal
    }

    fn name(env: Env) -> String {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }
        read_token_info(&env).name
    }

    fn symbol(env: Env) -> String {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }
        read_token_info(&env).symbol
    }
}

#[contractimpl]
impl TokenAdminInterface for Token {
    fn initialize(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if is_initialized(&env) {
            panic!("token already initialized");
        }

        // Validate inputs
        if decimal > 18 {
            panic!("decimal must be <= 18");
        }

        if name.len() == 0 || name.len() > 32 {
            panic!("name must be 1-32 characters");
        }

        if symbol.len() == 0 || symbol.len() > 12 {
            panic!("symbol must be 1-12 characters");
        }

        // Set token metadata
        let metadata = TokenMetadata {
            decimal,
            name,
            symbol,
        };
        write_token_info(&env, &metadata);

        // Set admin
        write_admin(&env, &admin);

        // Mark as initialized
        set_initialized(&env);
    }

    fn mint(env: Env, to: Address, amount: i128) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        if amount < 0 {
            panic!("negative amount");
        }

        if amount == 0 {
            return; // No-op for zero mints
        }

        let balance = read_balance(&env, &to);
        write_balance(&env, &to, balance + amount);

        // Emit mint event
        env.events().publish(("mint", &to), amount);
    }

    fn set_admin(env: Env, new_admin: Address) {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        write_admin(&env, &new_admin);

        // Emit admin change event
        env.events().publish(("set_admin", &admin, &new_admin), ());
    }

    fn admin(env: Env) -> Address {
        if !is_initialized(&env) {
            panic!("token not initialized");
        }
        read_admin(&env)
    }

    fn initialized(env: Env) -> bool {
        is_initialized(&env)
    }
}
