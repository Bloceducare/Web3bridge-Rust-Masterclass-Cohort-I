#![cfg(test)]

use crate::child::{ChildContract, ChildContractClient};

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(ChildContract, ());
    let client = ChildContractClient::new(&env, &contract_id);
}
