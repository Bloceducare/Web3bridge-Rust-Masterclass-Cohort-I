use crate::error::ContractErrors;
use crate::import::contract_a::Client;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct SecondContract;

#[contractimpl]
impl SecondContract {
    pub fn add_from_first_contract(
        env: soroban_sdk::Env,
        contract_address: soroban_sdk::Address,
        a: i32,
        b: i32,
    ) -> Result<(), ContractErrors> {
        if a < 50 {
            return Err(ContractErrors::LessThan);
        }

        let new = Client::new(&env, &contract_address);

        new.add(&a, &b);
        Ok(())
    }

    pub fn sub_from_first_contract(
        env: soroban_sdk::Env,
        contract_address: soroban_sdk::Address,
        a: i32,
        b: i32,
    ) -> Result<i32, ContractErrors> {
        if a < 50 {
            return Err(ContractErrors::LessThan);
        }

        let new = Client::new(&env, &contract_address);

        Ok(new.sub(&a, &b))
    }
}
