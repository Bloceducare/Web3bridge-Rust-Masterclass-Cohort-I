// - Create an employee management system where:
// - A user is registered to an institution and a price is agreed upon employment.
// - Create logic to basically:
// - add, remove, update, promote, and suspend an employee.
// - Add ranks to the employee upon registration
// - Write proper tests for all functions.
// - Try to deploy on your own
// - Enjoy :)\
// use error::ContractErrors;

use soroban_sdk::{ Address, Env };
use crate::errors::ContractErrors;
use crate::types::{ Employee, EmployeeRank };
use soroban_sdk::String;

pub trait EmployeeManagement {
    fn initialize(env: Env, admin: Address) -> Result<(), ContractErrors>;
    fn set_token_contract(env: Env, token_contract: Address);
    fn add_employee(
        env: Env,
        name: String,
        price: i128,
        rank: EmployeeRank,
        institution_name: String
    ) -> Result<Address, ContractErrors>;
    fn remove_employee(env: Env, id: Address) -> Result<(), ContractErrors>;
    fn update_employee(
        env: Env,
        id: Address,
        name: String,
        price: i128
    ) -> Result<(), ContractErrors>;
    fn promote_employee(env: Env, id: Address) -> Result<(), ContractErrors>;
    fn suspend_employee(env: Env, id: Address) -> Result<(), ContractErrors>;
    fn reinstate_employee(env: Env, id: Address) -> Result<(), ContractErrors>;
    fn pay_salary(env: Env, from: Address, id: Address) -> Result<(), ContractErrors>;
    fn get_employee(env: Env, id: Address) -> Result<Employee, ContractErrors>;
    fn get_employee_count(env: Env) -> Result<u32, ContractErrors>;
    fn get_token_contract(env: Env) -> Result<Address, ContractErrors>;
}
