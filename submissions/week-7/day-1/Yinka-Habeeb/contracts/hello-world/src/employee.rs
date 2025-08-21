#[no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};
use crate::token::TokenClient;

#[contracttype]
#[derive(Clone, Debug)]
pub struct Employee {
    pub name: String,
    pub address: Address,
    pub pay: i128,
}

#[contracttype]
pub enum DataKey {
    Owner,
    Token,
    Employee(Address),
}

#[contract]
pub struct EmployeeContract;

#[contractimpl]
impl EmployeeContract {
    pub fn init(env: &Env, admin: Address, token: Address) {
        env.storage().instance().set(&DataKey::Owner, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    pub fn add_employee(env: &Env, admin: Address, name: String, emp: Address, pay: i128) {
        admin.require_auth();
        let employee = Employee { name, address: emp.clone(), pay };
        env.storage().instance().set(&DataKey::Employee(emp), &employee);
    }

    pub fn pay_employee(env: &Env, admin: Address, emp: Address) {
        admin.require_auth();
        let employee: Employee = env.storage().instance().get(&DataKey::Employee(emp.clone())).unwrap();
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();

        let client = TokenClient::new(env, &token);
        let current = env.current_contract_address();
        client.transfer_from(&current, &admin, &employee.address, &employee.pay);
    }
}
