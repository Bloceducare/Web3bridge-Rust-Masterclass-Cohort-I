use soroban_sdk::{Env, Vec, Symbol, Address, Val, IntoVal};

pub fn emit_approve_event(env: &Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {
    let topics: Vec<Val> = Vec::from_array(
        env,
        [
            Symbol::new(env, "approve").to_val(),
            from.to_val(),
            spender.to_val(),
        ],
    );
    let data: Vec<Val> = Vec::from_array(env, [amount.into_val(env), live_until_ledger.into_val(env)]);
    env.events().publish(topics, data);
}

pub fn emit_transfer_event(env: &Env, from: Address, to: Address, amount: i128) {
    let topics: Vec<Val> = Vec::from_array(
        env,
        [
            Symbol::new(env, "transfer").to_val(),
            from.to_val(),
            to.to_val(),
        ],
    );
    env.events().publish(topics, amount);
}

pub fn emit_burn_event(env: &Env, from: Address, amount: i128) {
    let topics: Vec<Val> = Vec::from_array(
        env,
        [
            Symbol::new(env, "burn").to_val(),
            from.to_val(),
        ],
    );
    env.events().publish(topics, amount);
}