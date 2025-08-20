use soroban_sdk::{symbol_short, Address, Env};

pub fn transfer_event(env: &Env, from: Address, to: Address, amount: i128) {
    let topics = (symbol_short!("transfer"), from, to);
    env.events().publish(topics, amount);
}

pub fn approve_event(
    env: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let topics = (symbol_short!("approve"), from, spender);
    env.events().publish(topics, (amount, expiration_ledger));
}

pub fn mint_event(env: &Env, admin: Address, to: Address, amount: i128) {
    let topics = (symbol_short!("mint"), admin, to);
    env.events().publish(topics, amount);
}

pub fn clawback_event(env: &Env, admin: Address, from: Address, amount: i128) {
    let topics = (symbol_short!("clawback"), admin, from);
    env.events().publish(topics, amount);
}

pub fn burn_event(env: &Env, from: Address, amount: i128) {
    let topics = (symbol_short!("burn"), from);
    env.events().publish(topics, amount);
}

pub fn set_admin_event(env: &Env, admin: Address, new_admin: Address) {
    let topics = (symbol_short!("set_admin"), admin);
    env.events().publish(topics, new_admin);
}
