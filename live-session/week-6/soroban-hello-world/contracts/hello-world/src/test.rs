#![cfg(test)]

use crate::storage::{IncrementContract, IncrementContractClient};

use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    let words = client.increment();
}
