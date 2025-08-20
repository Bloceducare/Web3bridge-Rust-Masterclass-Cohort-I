use::create::mod::o::client;
use soroban_sdk::{contract, contractimpl, };


#[contract]
pub struct second_contract;
#[contractimpl]
impl second_contract {
    pub fn add(env: &Env, a: i32, b: i32) -> i32 {
        pub fn add_first_contract(env: &Env, a: i32, b: i32) -> i32 {
           env:soroban_sdk::env,
           contract_address: soroban_sdk::Address,
           a:i32,
            b:i32,
        }-> {
            let new = contract_a::client::new($env,$contract_address);
            new.add(a, b)
            
        }
        pub fn sub_first_contract(env: &Env, a: i32, b: i32) -> i32 {
           env:soroban_sdk::env,
           contract_address: soroban_sdk::Address,
           a:i32,
            b:i32,
        }-> {
            let new = contract_a::client::new($env,$contract_address);
            new.sub(a, b)
            
        }
    }
}