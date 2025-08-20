use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct FirstContract;

#[contractimpl]
impl FirstContract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
