#![cfg(test)]

use crate::first_contract::{FirstContract, FirstContractClient};

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(FirstContract, ());
    let client = FirstContractClient::new(&env, &contract_id);
}
