use soroban_sdk::{Address, Env};
use crate::storage::{get_allowance, set_allowance, get_allowance_expiration, set_allowance_expiration};

pub fn approve_allowance(
    env: &Env,
    owner: &Address,
    spender: &Address,
    amount: &i128,
    expiration_ledger: &u32,
) {
    owner.require_auth();
    
    if *amount < 0 {
        panic!("Amount must be non-negative");
    }

    let current_ledger = env.ledger().sequence();
    if *expiration_ledger <= current_ledger {
        panic!("Expiration must be in the future");
    }

    set_allowance(env, owner, spender, amount);
    set_allowance_expiration(env, owner, spender, expiration_ledger);
}

pub fn check_allowance(env: &Env, owner: &Address, spender: &Address) -> i128 {
    let expiration = get_allowance_expiration(env, owner, spender);
    let current_ledger = env.ledger().sequence();
    
    if expiration <= current_ledger {
        return 0;
    }

    get_allowance(env, owner, spender)
}