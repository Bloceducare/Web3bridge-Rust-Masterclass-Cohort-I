use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, panic_with_error, symbol_short, contracttype, Bytes};
use soroban_sdk::token::TokenClient;
use crate::storage::{get_employee, set_employee, get_all_employees, remove_employee};
use crate::events::{emit_employee_added, emit_employee_removed, emit_employee_updated, emit_employee_promoted, emit_employee_suspended};
use crate::errors::EmployeeError;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Rank {
    Junior,
    Senior,
    Manager,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Active,
    Suspended,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub rank: Rank,
    pub salary: i128, // Salary in SEP-41 tokens
    pub status: Status,
    pub institution: Address,
}

const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct EmployeeManager;

#[contractimpl]
impl EmployeeManager {
    pub fn initialize(env: Env, token_contract: Address) {
        if env.storage().instance().has(&symbol_short!("TOKEN")) {
            panic_with_error!(env, EmployeeError::AlreadyInitialized);
        }
        env.storage().instance().set(&symbol_short!("TOKEN"), &token_contract);
    }

    pub fn add_employee(
        env: Env,
        name: String,
        rank: Rank,
        salary: i128,
        institution: Address,
        admin: Address,
    ) -> u32 {
        institution.require_auth();
        admin.require_auth();

        let token_client = Self::get_token_client(&env);
        let mut current_id = env.storage().persistent().get(&NEXT_ID).unwrap_or(1u32);

        let employee = Employee {
            id: current_id,
            name,
            rank,
            salary,
            status: Status::Active,
            institution: institution.clone(),
        };

        let employee_address = employee.id.to_address(&env);
        token_client.transfer(&institution, &employee_address, &salary);

        set_employee(&env, current_id, &employee);
        emit_employee_added(&env, current_id, &employee.name, &employee.rank, salary, &institution);

        current_id += 1;
        env.storage().persistent().set(&NEXT_ID, &current_id);

        current_id - 1
    }

    pub fn remove_employee(env: Env, id: u32, admin: Address) {
        admin.require_auth();
        let employee = get_employee(&env, id).unwrap_or_else(|| panic_with_error!(env, EmployeeError::EmployeeNotFound));
        if employee.status == Status::Suspended {
            panic_with_error!(env, EmployeeError::EmployeeSuspended);
        }
        remove_employee(&env, id);
        emit_employee_removed(&env, id, &employee.name, &employee.institution);
    }

    pub fn update_employee(env: Env, id: u32, name: String, salary: i128, admin: Address) {
        admin.require_auth();
        let mut employee = get_employee(&env, id).unwrap_or_else(|| panic_with_error!(env, EmployeeError::EmployeeNotFound));
        if employee.status == Status::Suspended {
            panic_with_error!(env, EmployeeError::EmployeeSuspended);
        }
        employee.name = name;
        employee.salary = salary;
        set_employee(&env, id, &employee);
        emit_employee_updated(&env, id, &employee.name, salary, &employee.institution);
    }

    pub fn promote_employee(env: Env, id: u32, new_rank: Rank, new_salary: i128, admin: Address) {
        admin.require_auth();
        let mut employee = get_employee(&env, id).unwrap_or_else(|| panic_with_error!(env, EmployeeError::EmployeeNotFound));
        if employee.status == Status::Suspended {
            panic_with_error!(env, EmployeeError::EmployeeSuspended);
        }
        if employee.rank == new_rank {
            panic_with_error!(env, EmployeeError::InvalidRank);
        }
        let token_client = Self::get_token_client(&env);
        let additional_salary = new_salary - employee.salary;
        if additional_salary > 0 {
            // Transfer additional salary - mint may not be available
            let employee_address = employee.id.to_address(&env);
            token_client.transfer(&employee.institution, &employee_address, &additional_salary);
        }
        employee.rank = new_rank;
        employee.salary = new_salary;
        set_employee(&env, id, &employee);
        emit_employee_promoted(&env, id, &employee.name, &employee.rank, new_salary, &employee.institution);
    }

    pub fn suspend_employee(env: Env, id: u32, admin: Address) {
        admin.require_auth();
        let mut employee = get_employee(&env, id).unwrap_or_else(|| panic_with_error!(env, EmployeeError::EmployeeNotFound));
        if employee.status == Status::Suspended {
            panic_with_error!(env, EmployeeError::EmployeeAlreadySuspended);
        }
        employee.status = Status::Suspended;
        set_employee(&env, id, &employee);
        emit_employee_suspended(&env, id, &employee.name, &employee.institution);
    }

    pub fn get_employee(env: Env, id: u32) -> Employee {
        get_employee(&env, id).unwrap_or_else(|| panic_with_error!(env, EmployeeError::EmployeeNotFound))
    }

    pub fn get_all_employees(env: Env) -> soroban_sdk::Vec<(u32, Employee)> {
        get_all_employees(&env)
    }

    fn get_token_client(env: &Env) -> TokenClient {
        let token_contract: Address = env.storage().instance().get(&symbol_short!("TOKEN")).unwrap_or_else(|| panic_with_error!(env, EmployeeError::NotInitialized));
        TokenClient::new(env, &token_contract)
    }
}

pub trait ToAddress {
    fn to_address(&self, env: &Env) -> Address;
}

impl ToAddress for u32 {
    fn to_address(&self, env: &Env) -> Address {
        let mut bytes = [0u8; 32];
        bytes[28..].copy_from_slice(&self.to_be_bytes());
        let bytes_obj = Bytes::from_array(env, &bytes);
        Address::from_string_bytes(&bytes_obj)
    }
}