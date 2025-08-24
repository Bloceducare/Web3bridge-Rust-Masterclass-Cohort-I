#![cfg(test)]

use crate::factory_contract::{FactoryContract, FactoryContractClient};

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(FactoryContract, ());
    let client = FactoryContractClient::new(&env, &contract_id);
}
