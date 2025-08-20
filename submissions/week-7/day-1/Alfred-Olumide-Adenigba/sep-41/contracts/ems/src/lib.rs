#![no_std]

mod test;
mod traits;
mod storage;
mod events;
mod impls;
mod token_interface;

pub use crate::traits::EmployeeManagementTrait;
pub use crate::impls::EmployeeManagement;
pub use crate::storage::Employee;

use soroban_sdk::{contractimpl, contract};

#[contract]
pub struct EmployeeManagementContract;

#[contractimpl]
impl EmployeeManagementTrait for EmployeeManagementContract {
    fn initialize(env: soroban_sdk::Env, admin: soroban_sdk::Address) {
        EmployeeManagement::initialize(env, admin);
    }
    fn add_employee(env: soroban_sdk::Env, employee_id: soroban_sdk::Address, institution: soroban_sdk::Address, salary: i128, rank: u32) {
        EmployeeManagement::add_employee(env, employee_id, institution, salary, rank);
    }
    fn remove_employee(env: soroban_sdk::Env, employee_id: soroban_sdk::Address) {
        EmployeeManagement::remove_employee(env, employee_id);
    }
    fn update_employee_salary(env: soroban_sdk::Env, employee_id: soroban_sdk::Address, new_salary: i128) {
        EmployeeManagement::update_employee_salary(env, employee_id, new_salary);
    }
    fn promote_employee(env: soroban_sdk::Env, employee_id: soroban_sdk::Address, new_rank: u32) {
        EmployeeManagement::promote_employee(env, employee_id, new_rank);
    }
    fn suspend_employee(env: soroban_sdk::Env, employee_id: soroban_sdk::Address) {
        EmployeeManagement::suspend_employee(env, employee_id);
    }
    fn reinstate_employee(env: soroban_sdk::Env, employee_id: soroban_sdk::Address) {
        EmployeeManagement::reinstate_employee(env, employee_id);
    }
    fn get_employee(env: soroban_sdk::Env, employee_id: soroban_sdk::Address) -> Option<Employee> {
        EmployeeManagement::get_employee(env, employee_id)
    }
    fn get_employee_count(env: soroban_sdk::Env) -> u32 {
        EmployeeManagement::get_employee_count(env)
    }
    fn set_token_contract(env: soroban_sdk::Env, token_contract: soroban_sdk::Address) {
        EmployeeManagement::set_token_contract(env, token_contract);
    }
    fn pay_salary(env: soroban_sdk::Env, from: soroban_sdk::Address, employee_id: soroban_sdk::Address) {
        EmployeeManagement::pay_salary(env, from, employee_id);
    }
}
