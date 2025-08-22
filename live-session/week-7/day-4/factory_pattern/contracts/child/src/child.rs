use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ChildContract;

#[contractimpl]
impl ChildContract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
