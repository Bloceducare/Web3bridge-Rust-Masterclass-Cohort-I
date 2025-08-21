// writing sep41 contract 

#![no_std]

// use core::ops::Add;

mod admin;
mod allowance;
mod balance;
mod metadata;
mod storage_types;
mod contract;

mod test;
// pub use crate::contract;


// use soroban_sdk::{contract, contractclient, contractimpl, Address, Env, String};

// #[contractclient( name = "TokenClient")]
// pub trait  Token {
//         fn allowance(env: Env, from: Address, to: Address) -> i128;
//         fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32);
//         fn balance(env: Env, id: Address) -> i128;
//         fn transfer(env: Env, from: Address, to: Address, amount: i128);
//         fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);
//         fn burn(env: Env, from: Address, amount: i128);
//         fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

//         fn decimals(env: Env) -> u32;
//         fn name(env: Env) -> String;
//         fn symbol(env: Env) -> String;

//         fn mint(env: Env, to: Address, amount: i128, live_until_ledger: u32);
// }

