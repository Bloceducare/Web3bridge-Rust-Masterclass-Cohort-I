#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror, Address, Env, String, symbol_short,
    token::Client as Sep41TokenClient, // Use SDK's token client
};
use core::result::Result;
use core::clone::Clone;
use core::marker::Copy;
use core::fmt::Debug;
use core::cmp::{Eq, PartialEq};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EmployeeError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    EmployeeNotFound = 3,
    EmployeeAlreadyExists = 4,
    Unauthorized = 5,
    InvalidRank = 6,
    InvalidSalary = 7,
    InsufficientBalance = 8,
    EmployeeSuspended = 9,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Employee {
    pub address: Address,
    pub name: String,
    pub rank: u32,
    pub salary: i128,
    pub is_active: bool,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Employees(Address),
    TokenContract,
    InstitutionName,
}

pub struct EmployeeEvents;

impl EmployeeEvents {
    pub fn employee_added(env: &Env, employee: &Address, name: &String, rank: u32) {
        let topics = (symbol_short!("emp_add"), employee.clone());
        env.events().publish(topics, (name.clone(), rank));
    }
    pub fn employee_removed(env: &Env, employee: &Address) {
        let topics = (symbol_short!("emp_rem"), employee.clone());
        env.events().publish(topics, ());
    }
    pub fn employee_updated(env: &Env, employee: &Address, name: &String) {
        let topics = (symbol_short!("emp_upd"), employee.clone());
        env.events().publish(topics, name.clone());
    }
    pub fn employee_promoted(env: &Env, employee: &Address, new_rank: u32) {
        let topics = (symbol_short!("emp_prom"), employee.clone());
        env.events().publish(topics, new_rank);
    }
    pub fn employee_suspended(env: &Env, employee: &Address, is_active: bool) {
        let topics = (symbol_short!("emp_susp"), employee.clone());
        env.events().publish(topics, is_active);
    }
}

#[contract]
pub struct EmployeeManagement;

#[contractimpl]
impl EmployeeManagement {
    pub fn initialize(env: Env, admin: Address, token_contract: Address, institution_name: String) -> Result<(), EmployeeError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(EmployeeError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenContract, &token_contract);
        env.storage().instance().set(&DataKey::InstitutionName, &institution_name);
        Ok(())
    }

    pub fn add_employee(
        env: Env,
        employee_addr: Address,
        name: String,
        rank: u32,
        salary: i128
    ) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if rank == 0 {
            return Err(EmployeeError::InvalidRank);
        }
        if salary <= 0 {
            return Err(EmployeeError::InvalidSalary);
        }
        if env.storage().persistent().has(&DataKey::Employees(employee_addr.clone())) {
            return Err(EmployeeError::EmployeeAlreadyExists);
        }
        let employee = Employee {
            address: employee_addr.clone(),
            name,
            rank,
            salary,
            is_active: true,
        };
        env.storage().persistent().set(&DataKey::Employees(employee_addr.clone()), &employee);
        EmployeeEvents::employee_added(&env, &employee_addr, &employee.name, employee.rank);
        Ok(())
    }

    pub fn remove_employee(env: Env, employee_addr: Address) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if !env.storage().persistent().has(&DataKey::Employees(employee_addr.clone())) {
            return Err(EmployeeError::EmployeeNotFound);
        }
        env.storage().persistent().remove(&DataKey::Employees(employee_addr.clone()));
        EmployeeEvents::employee_removed(&env, &employee_addr);
        Ok(())
    }

    pub fn update_employee(
        env: Env,
        employee_addr: Address,
        name: String,
        salary: i128
    ) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if salary <= 0 {
            return Err(EmployeeError::InvalidSalary);
        }
        let mut employee = Self::get_employee(&env, employee_addr.clone())?;
        employee.name = name.clone();
        employee.salary = salary;
        env.storage().persistent().set(&DataKey::Employees(employee_addr.clone()), &employee);
        EmployeeEvents::employee_updated(&env, &employee_addr, &name);
        Ok(())
    }

    pub fn promote_employee(env: Env, employee_addr: Address, new_rank: u32) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        if new_rank == 0 {
            return Err(EmployeeError::InvalidRank);
        }
        let mut employee = Self::get_employee(&env, employee_addr.clone())?;
        employee.rank = new_rank;
        env.storage().persistent().set(&DataKey::Employees(employee_addr.clone()), &employee);
        EmployeeEvents::employee_promoted(&env, &employee_addr, new_rank);
        Ok(())
    }

    pub fn suspend_employee(env: Env, employee_addr: Address, suspend: bool) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        let mut employee = Self::get_employee(&env, employee_addr.clone())?;
        employee.is_active = !suspend;
        env.storage().persistent().set(&DataKey::Employees(employee_addr.clone()), &employee);
        EmployeeEvents::employee_suspended(&env, &employee_addr, employee.is_active);
        Ok(())
    }

    pub fn pay_salary(env: Env, employee_addr: Address) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(&env)?;
        admin.require_auth();
        let employee = Self::get_employee(&env, employee_addr.clone())?;
        if !employee.is_active {
            return Err(EmployeeError::EmployeeSuspended);
        }
        let token_contract = Self::get_token_contract(&env)?;
        let client = Sep41TokenClient::new(&env, &token_contract);
        if client.balance(&admin) < employee.salary {
            return Err(EmployeeError::InsufficientBalance);
        }
        client.transfer(&admin, &employee_addr, &employee.salary);
        Ok(())
    }

    pub fn get_employee(env: &Env, employee_addr: Address) -> Result<Employee, EmployeeError> {
        env.storage()
            .persistent()
            .get(&DataKey::Employees(employee_addr))
            .ok_or(EmployeeError::EmployeeNotFound)
    }

    pub fn get_admin(env: &Env) -> Result<Address, EmployeeError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(EmployeeError::NotInitialized)
    }

    pub fn get_token_contract(env: &Env) -> Result<Address, EmployeeError> {
        env.storage()
            .instance()
            .get(&DataKey::TokenContract)
            .ok_or(EmployeeError::NotInitialized)
    }

    pub fn get_institution_name(env: Env) -> Result<String, EmployeeError> {
        env.storage()
            .instance()
            .get(&DataKey::InstitutionName)
            .ok_or(EmployeeError::NotInitialized)
    }
}

// #[cfg(test)]
// mod test;