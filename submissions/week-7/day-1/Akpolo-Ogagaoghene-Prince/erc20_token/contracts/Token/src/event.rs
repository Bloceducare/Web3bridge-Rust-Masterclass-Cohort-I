
use soroban_sdk::{Address, Env};

pub(crate) fn approve(env: &Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
    let topics = ("approve", from, spender);
    env.events().publish(topics, (amount, expiration_ledger));
}

pub(crate) fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
    let topics = ("transfer", from, to);
    env.events().publish(topics, amount);
}

pub(crate) fn mint(env: &Env, admin: Address, to: Address, amount: i128) {
    let topics = ("mint", admin, to);
    env.events().publish(topics, amount);
}

pub(crate) fn burn(env: &Env, from: Address, amount: i128) {
    let topics = ("burn", from);
    env.events().publish(topics, amount);
}

pub(crate) fn set_admin(env: &Env, admin: Address, new_admin: Address) {
    let topics = ("set_admin", admin);
    env.events().publish(topics, new_admin);
}