#![no_std]
use soroban_sdk::{contract, contractimpl, contractmeta, Address, Env, String};

pub mod admin;
pub mod allowance;
pub mod balance;
pub mod metadata;
pub mod storage;
pub mod types;
pub mod traits;

pub use admin::*;
pub use allowance::*;
pub use balance::*;
pub use metadata::*;
pub use storage::*;
pub use types::*;
pub use traits::*;

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
    ) {
        if has_admin(&env) {
            panic!("Already initialized");
        }

        admin.require_auth();

        if name.len() > 32 {
            panic!("Name must be 32 characters or less");
        }
        
        if symbol.len() > 16 {
            panic!("Symbol must be 16 characters or less");
        }

        if decimals > 18 {
            panic!("Decimals must be 18 or less");
        }
        
        if total_supply < 0 {
            panic!("Total supply must be non-negative");
        }

        // Set metadata
        set_admin(&env, &admin);
        set_name(&env, &name);
        set_symbol(&env, &symbol);
        set_decimals(&env, &decimals);
        set_total_supply(&env, &total_supply);

        if total_supply > 0 {
            set_balance(&env, &admin, &total_supply);
        }
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        Self::transfer_impl(&env, &from, &to, &amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        let allowance = get_allowance(&env, &from, &spender);
        if allowance < amount {
            panic!("Insufficient allowance");
        }

        set_allowance(&env, &from, &spender, &(allowance - amount));
        Self::transfer_impl(&env, &from, &to, &amount);
    }

    fn transfer_impl(env: &Env, from: &Address, to: &Address, amount: &i128) {
        if *amount < 0 {
            panic!("Amount must be non-negative");
        }

        if *amount == 0 {
            return;
        }

        if from == to {
            return;
        }

        if get_frozen(env, from) {
            panic!("From account is frozen");
        }

        if get_frozen(env, to) {
            panic!("To account is frozen");
        }

        let from_balance = get_balance(env, from);
        if from_balance < *amount {
            panic!("Insufficient balance");
        }

        set_balance(env, from, &(from_balance - amount));
        let to_balance = get_balance(env, to);
        set_balance(env, to, &(to_balance + amount));
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
    ) {
        from.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let current_ledger = env.ledger().sequence();
        if expiration_ledger <= current_ledger {
            panic!("Expiration ledger must be in the future");
        }

        set_allowance(&env, &from, &spender, &amount);
        set_allowance_expiration(&env, &from, &spender, &expiration_ledger);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let expiration = get_allowance_expiration(&env, &from, &spender);
        let current_ledger = env.ledger().sequence();
        
        if expiration <= current_ledger {
            return 0;
        }

        get_allowance(&env, &from, &spender)
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin = get_admin(&env);
        admin.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        if amount == 0 {
            return;
        }

        if get_frozen(&env, &to) {
            panic!("To account is frozen");
        }

        let to_balance = get_balance(&env, &to);
        let new_balance = to_balance.checked_add(amount).unwrap_or_else(|| panic!("Balance overflow"));
        
        set_balance(&env, &to, &new_balance);

        let total_supply = get_total_supply(&env);
        let new_supply = total_supply.checked_add(amount).unwrap_or_else(|| panic!("Supply overflow"));
        
        set_total_supply(&env, &new_supply);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();
        Self::burn_impl(&env, &from, &amount);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        let allowance = get_allowance(&env, &from, &spender);
        if allowance < amount {
            panic!("Insufficient allowance");
        }

        set_allowance(&env, &from, &spender, &(allowance - amount));
        Self::burn_impl(&env, &from, &amount);
    }

    fn burn_impl(env: &Env, from: &Address, amount: &i128) {
        if *amount < 0 {
            panic!("Amount must be non-negative");
        }

        if *amount == 0 {
            return;
        }

        let from_balance = get_balance(env, from);
        if from_balance < *amount {
            panic!("Insufficient balance");
        }

        set_balance(env, from, &(from_balance - amount));

        let total_supply = get_total_supply(env);
        set_total_supply(env, &(total_supply - amount));
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin = get_admin(&env);
        admin.require_auth();
        set_admin(&env, &new_admin);
    }

    pub fn admin(env: Env) -> Address {
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
        let admin = get_admin(&env);
        admin.require_auth();
        set_frozen(&env, &account, &true);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        let admin = get_admin(&env);
        admin.require_auth();
        set_frozen(&env, &account, &false);
    }

    pub fn is_frozen(env: Env, account: Address) -> bool {
        get_frozen(&env, &account)
    }
}

mod test;