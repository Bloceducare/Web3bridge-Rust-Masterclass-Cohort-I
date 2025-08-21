use crate::datatypes::ZERO_ADDRESS;
use soroban_sdk::{contracttype, Address, Env, Symbol};

/// Event topics
/// These are the topics used to identify events in the Soroban blockchain.
// pub const APPROVAL_EVENT: Symbol = Symbol::new(&env, "approve");

/// Event data structures
/// These structures define the data that will be emitted in the events.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transfer {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Approve {
    pub owner: Address,
    pub spender: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mint {
    pub to: Address,
    pub from: Address,
    pub amount: i128,
}

pub fn emit_transfer(env: &Env, from: Address, to: Address, amount: i128) {
    let topics = (Symbol::new(&env, "transfer"),);
    let transfer_data = Transfer { from, to, amount };
    env.events().publish(topics, transfer_data.clone());
}

pub fn emit_approve(env: &Env, owner: Address, spender: Address, amount: i128) {
    let topics = (Symbol::new(&env, "approve"),);
    let approval_data = Approve {
        owner,
        spender,
        amount,
    };
    env.events().publish(topics, approval_data.clone());
}

pub fn emit_mint(env: &Env, to: Address, amount: i128) {
    let topics = (Symbol::new(&env, "mint"),);
    let zero_addr = Address::from_str(&env, ZERO_ADDRESS);
    let mint_data = Mint {
        to,
        from: zero_addr,
        amount,
    };
    env.events().publish(topics, mint_data.clone());
}

pub fn emit_burn(env: &Env, from: Address, amount: i128) {
    let topics = (Symbol::new(&env, "burn"),);
    let zero_addr = Address::from_str(&env, ZERO_ADDRESS);
    let burn_data = Transfer {
        from,
        to: zero_addr,
        amount,
    };
    env.events().publish(topics, burn_data.clone());
}
