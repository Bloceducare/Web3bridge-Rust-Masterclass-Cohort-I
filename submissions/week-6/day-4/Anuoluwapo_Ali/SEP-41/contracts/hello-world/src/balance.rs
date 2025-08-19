use soroban_sdk::{Address, Env};
use crate::storage::{get_balance, set_balance, get_frozen};

pub fn transfer_balance(env: &Env, from: &Address, to: &Address, amount: &i128) {
    if *amount <= 0 {
        panic!("Amount must be positive");
    }

    if get_frozen(env, from) {
        panic!("Sender account is frozen");
    }

    if get_frozen(env, to) {
        panic!("Recipient account is frozen");
    }

    let from_balance = get_balance(env, from);
    if from_balance < *amount {
        panic!("Insufficient balance");
    }

    set_balance(env, from, &(from_balance - amount));
    
    let to_balance = get_balance(env, to);
    set_balance(env, to, &(to_balance + amount));
}
