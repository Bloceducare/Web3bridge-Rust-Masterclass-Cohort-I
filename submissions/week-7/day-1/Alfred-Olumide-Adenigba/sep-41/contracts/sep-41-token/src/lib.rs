#![no_std]

mod test;
mod token;
mod storage;
mod events;
mod impls;

pub use crate::token::TokenTrait;
pub use crate::impls::Token;

use soroban_sdk::{contractimpl, contract, String};

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenTrait for TokenContract {
    fn initialize(env: soroban_sdk::Env, admin: soroban_sdk::Address, total_supply: i128) {
        Token::initialize(env, admin, total_supply);
    }
    fn name(env: soroban_sdk::Env) -> String {
        Token::name(env)
    }
    fn symbol(env: soroban_sdk::Env) -> String {
        Token::symbol(env)
    }
    fn decimals(env: soroban_sdk::Env) -> u32 {
        Token::decimals(env)
    }
    fn total_supply(env: soroban_sdk::Env) -> i128 {
        Token::total_supply(env)
    }
    fn balance_of(env: soroban_sdk::Env, owner: soroban_sdk::Address) -> i128 {
        Token::balance_of(env, owner)
    }
    fn allowance(env: soroban_sdk::Env, owner: soroban_sdk::Address, spender: soroban_sdk::Address) -> i128 {
        Token::allowance(env, owner, spender)
    }
    fn approve(env: soroban_sdk::Env, from: soroban_sdk::Address, spender: soroban_sdk::Address, amount: i128) {
        Token::approve(env, from, spender, amount)
    }
    fn transfer(env: soroban_sdk::Env, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        Token::transfer(env, from, to, amount)
    }
    fn transfer_from(env: soroban_sdk::Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, to: soroban_sdk::Address, amount: i128) {
        Token::transfer_from(env, spender, from, to, amount)
    }
    fn mint(env: soroban_sdk::Env, to: soroban_sdk::Address, amount: i128) {
        Token::mint(env, to, amount);
    }
    fn burn(env: soroban_sdk::Env, from: soroban_sdk::Address, amount: i128) {
        Token::burn(env, from, amount);
    }
    fn burn_from(env: soroban_sdk::Env, spender: soroban_sdk::Address, from: soroban_sdk::Address, amount: i128) {
        Token::burn_from(env, spender, from, amount);
    }
}
