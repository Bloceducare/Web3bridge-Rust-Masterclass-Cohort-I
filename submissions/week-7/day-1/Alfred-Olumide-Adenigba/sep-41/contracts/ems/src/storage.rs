use soroban_sdk::{Address, Env, contracttype};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    EmployeeCount,
    Employee(Address),
    TokenContract,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Employee {
    pub id: Address,
    pub institution: Address,
    pub salary: i128,
    pub rank: u32,
    pub is_active: bool,
    pub is_suspended: bool,
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::Admin)
}

pub fn set_employee(env: &Env, employee: &Employee) {
    env.storage().persistent().set(&DataKey::Employee(employee.id.clone()), employee);
}

pub fn get_employee(env: &Env, employee_id: &Address) -> Option<Employee> {
    env.storage().persistent().get(&DataKey::Employee(employee_id.clone()))
}

pub fn remove_employee(env: &Env, employee_id: &Address) {
    env.storage().persistent().remove(&DataKey::Employee(employee_id.clone()));
}

pub fn set_employee_count(env: &Env, count: u32) {
    env.storage().persistent().set(&DataKey::EmployeeCount, &count);
}

pub fn get_employee_count(env: &Env) -> u32 {
    env.storage().persistent().get(&DataKey::EmployeeCount).unwrap_or(0)
}

pub fn set_token_contract(env: &Env, token_contract: &Address) {
    env.storage().persistent().set(&DataKey::TokenContract, token_contract);
}

pub fn get_token_contract(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&DataKey::TokenContract)
}
