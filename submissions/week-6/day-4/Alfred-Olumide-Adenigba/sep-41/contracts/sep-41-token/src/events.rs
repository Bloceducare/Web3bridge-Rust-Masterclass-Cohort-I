use soroban_sdk::{Address, Env, symbol_short};

pub fn transfer_event(env: &Env, from: Address, to: Address, amount: i128) {
    let topics = (symbol_short!("transfer"), from.clone(), to.clone());
    env.events().publish(topics, amount);
}

pub fn approve_event(env: &Env, owner: Address, spender: Address, amount: i128) {
    let topics = (symbol_short!("approve"), owner.clone(), spender.clone());
    env.events().publish(topics, amount);
}
