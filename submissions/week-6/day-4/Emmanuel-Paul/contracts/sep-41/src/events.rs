use soroban_sdk::{symbol_short, Address, Env};

pub struct Events;

impl Events {
    pub fn transfer(env: &Env, from: &Address, to: &Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from.clone(), to.clone());
        env.events().publish(topics, amount);
    }

    pub fn burn(env: &Env, from: &Address, amount: i128) {
        let topics = (symbol_short!("burn"), from.clone());
        env.events().publish(topics, amount);
    }

    pub fn approve(
        env: &Env,
        from: &Address,
        spender: &Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        let topics = (symbol_short!("approve"), from.clone(), spender.clone());
        env.events().publish(topics, (amount, expiration_ledger));
    }
}
