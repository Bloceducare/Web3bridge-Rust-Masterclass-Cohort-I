use soroban_sdk::{contract, contractimpl, token::Interface as TokenInterface, Address, Env};
#[contract]
pub struct first_contract;
#[contractimpl]
impl first_contract {
    pub fn add(env: &Env, a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn sub(env: &Env, a: i32, b: i32) -> i32 {
        a + b
    }
}