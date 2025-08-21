#[no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Balance(Address),
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn transfer_from(env: &Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let mut bal_from: i128 = env.storage().instance().get(&DataKey::Balance(from.clone())).unwrap_or(0);
        let mut bal_to: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);

        if bal_from < amount {
            panic!("insufficient balance");
        }

        bal_from -= amount;
        bal_to += amount;

        env.storage().instance().set(&DataKey::Balance(from), &bal_from);
        env.storage().instance().set(&DataKey::Balance(to), &bal_to);
    }

    pub fn mint(env: &Env, to: Address, amount: i128) {
        let mut bal: i128 = env.storage().instance().get(&DataKey::Balance(to.clone())).unwrap_or(0);
        bal += amount;
        env.storage().instance().set(&DataKey::Balance(to), &bal);
    }

    pub fn balance_of(env: &Env, addr: Address) -> i128 {
        env.storage().instance().get(&DataKey::Balance(addr)).unwrap_or(0)
    }
}
