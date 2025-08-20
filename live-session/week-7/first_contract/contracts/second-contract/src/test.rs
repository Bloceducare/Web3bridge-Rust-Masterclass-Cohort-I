#![cfg(test)]

use crate::second_contract::{SecondContract, SecondContractClient};

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(SecondContract, ());
    let client = SecondContractClient::new(&env, &contract_id);
}
