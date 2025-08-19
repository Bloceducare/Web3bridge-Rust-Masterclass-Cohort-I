use soroban_sdk::{Address, Env};
use crate::storage::Employee;

pub trait EmployeeManagementTrait {
    fn initialize(env: Env, admin: Address);
    fn set_token_contract(env: Env, token_contract: Address);
    fn add_employee(env: Env, employee_id: Address, institution: Address, salary: i128, rank: u32);
    fn remove_employee(env: Env, employee_id: Address);
    fn update_employee_salary(env: Env, employee_id: Address, new_salary: i128);
    fn promote_employee(env: Env, employee_id: Address, new_rank: u32);
    fn suspend_employee(env: Env, employee_id: Address);
    fn reinstate_employee(env: Env, employee_id: Address);
    fn pay_salary(env: Env, from: Address, employee_id: Address);
    fn get_employee(env: Env, employee_id: Address) -> Option<Employee>;
    fn get_employee_count(env: Env) -> u32;
}
