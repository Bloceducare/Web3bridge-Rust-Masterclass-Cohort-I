#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, symbol_short, Symbol,
};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum EmployeeRank {
    Intern = 1,
    Junior = 2,
    Senior = 3,
    Lead = 4,
    Manager = 5,
    Director = 6,
}

impl EmployeeRank {
    pub fn from_u32(value: u32) -> Option<EmployeeRank> {
        match value {
            1 => Some(EmployeeRank::Intern),
            2 => Some(EmployeeRank::Junior),
            3 => Some(EmployeeRank::Senior),
            4 => Some(EmployeeRank::Lead),
            5 => Some(EmployeeRank::Manager),
            6 => Some(EmployeeRank::Director),
            _ => None,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            EmployeeRank::Intern => 1,
            EmployeeRank::Junior => 2,
            EmployeeRank::Senior => 3,
            EmployeeRank::Lead => 4,
            EmployeeRank::Manager => 5,
            EmployeeRank::Director => 6,
        }
    }

    pub fn get_base_salary(&self) -> i128 {
        match self {
            EmployeeRank::Intern => 50_000,
            EmployeeRank::Junior => 80_000,
            EmployeeRank::Senior => 120_000,
            EmployeeRank::Lead => 150_000,
            EmployeeRank::Manager => 200_000,
            EmployeeRank::Director => 300_000,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum EmployeeStatus {
    Active,
    Suspended(u64),
    Terminated,
}

#[derive(Clone)]
#[contracttype]
pub struct Employee {
    pub name: String,
    pub rank: EmployeeRank,
    pub salary: i128,
    pub hire_date: u64,
    pub status: EmployeeStatus,
    pub last_payment: u64,
}

const ADMIN: Symbol = symbol_short!("ADMIN");
const TOKEN: Symbol = symbol_short!("TOKEN");
const EMPLOYEE: Symbol = symbol_short!("EMPLOYEE");

#[contract]
pub struct EmployeeManagement;

#[contractimpl]
impl EmployeeManagement {
    pub fn initialize(env: Env, admin: Address, token_contract: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract already initialized");
        }
        
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token_contract);
    }

    pub fn add_employee(
        env: Env,
        employee_address: Address,
        name: String,
        rank: EmployeeRank,
        custom_salary: Option<i128>,
    ) {
        Self::require_admin(&env);
        
        if env.storage().persistent().has(&(EMPLOYEE, employee_address.clone())) {
            panic!("Employee already exists");
        }

        let salary = custom_salary.unwrap_or_else(|| rank.get_base_salary());
        
        if salary <= 0 {
            panic!("Salary must be positive");
        }

        let employee = Employee {
            name,
            rank,
            salary,
            hire_date: env.ledger().timestamp(),
            status: EmployeeStatus::Active,
            last_payment: 0,
        };

        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "employee_added", &employee_address);
    }

    pub fn remove_employee(env: Env, employee_address: Address) {
        Self::require_admin(&env);
        
        if !env.storage().persistent().has(&(EMPLOYEE, employee_address.clone())) {
            panic!("Employee not found");
        }

        env.storage().persistent().remove(&(EMPLOYEE, employee_address.clone()));
        Self::publish_event(&env, "employee_removed", &employee_address);
    }

    pub fn update_employee_salary(env: Env, employee_address: Address, new_salary: i128) {
        Self::require_admin(&env);
        
        if new_salary <= 0 {
            panic!("Salary must be positive");
        }

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&(EMPLOYEE, employee_address.clone()))
            .expect("Employee not found");

        employee.salary = new_salary;
        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "salary_updated", &employee_address);
    }

    pub fn update_employee_name(env: Env, employee_address: Address, new_name: String) {
        Self::require_admin(&env);

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&(EMPLOYEE, employee_address.clone()))
            .expect("Employee not found");

        employee.name = new_name;
        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "name_updated", &employee_address);
    }

    pub fn promote_employee(env: Env, employee_address: Address, new_rank: EmployeeRank) {
        Self::require_admin(&env);

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&(EMPLOYEE, employee_address.clone()))
            .expect("Employee not found");

        if new_rank.to_u32() <= employee.rank.to_u32() {
            panic!("New rank must be higher than current rank");
        }

        employee.rank = new_rank.clone();
        employee.salary = new_rank.get_base_salary();
        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "employee_promoted", &employee_address);
    }

    pub fn suspend_employee(env: Env, employee_address: Address, duration_days: u32) {
        Self::require_admin(&env);

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&(EMPLOYEE, employee_address.clone()))
            .expect("Employee not found");

        let suspension_end = env.ledger().timestamp() + (duration_days as u64 * 24 * 60 * 60);
        employee.status = EmployeeStatus::Suspended(suspension_end);

        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "employee_suspended", &employee_address);
    }

    pub fn reactivate_employee(env: Env, employee_address: Address) {
        Self::require_admin(&env);

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&(EMPLOYEE, employee_address.clone()))
            .expect("Employee not found");

        employee.status = EmployeeStatus::Active;
        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "employee_reactivated", &employee_address);
    }

    pub fn pay_salary(env: Env, employee_address: Address) {
        Self::require_admin(&env);

        let mut employee: Employee = env
            .storage()
            .persistent()
            .get(&(EMPLOYEE, employee_address.clone()))
            .expect("Employee not found");

        match employee.status {
            EmployeeStatus::Active => {},
            EmployeeStatus::Suspended(until_timestamp) => {
                if env.ledger().timestamp() < until_timestamp {
                    panic!("Employee is currently suspended");
                } else {
                    employee.status = EmployeeStatus::Active;
                }
            },
            EmployeeStatus::Terminated => panic!("Cannot pay terminated employee"),
        }

        let current_time = env.ledger().timestamp();
        let one_month = 30 * 24 * 60 * 60; // 30 days in seconds
        
        if employee.last_payment > 0 && current_time - employee.last_payment < one_month {
            panic!("Employee was paid less than a month ago");
        }

        let token_contract: Address = env.storage().instance().get(&TOKEN).expect("Token contract not set");
        let admin: Address = env.storage().instance().get(&ADMIN).expect("Admin not set");
        
        // Note: In production, this would call the token contract for transfer
        // For now, we'll just record the payment without actual token transfer
        // env.invoke_contract(&token_contract, &Symbol::new(&env, "transfer"), args);

        employee.last_payment = current_time;
        env.storage().persistent().set(&(EMPLOYEE, employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&(EMPLOYEE, employee_address.clone()), 1000, 1000);
        
        Self::publish_event(&env, "salary_paid", &employee_address);
    }

    pub fn get_employee(env: Env, employee_address: Address) -> Option<Employee> {
        env.storage().persistent().get(&(EMPLOYEE, employee_address))
    }

    pub fn is_employee_active(env: Env, employee_address: Address) -> bool {
        if let Some(employee) = Self::get_employee(env.clone(), employee_address) {
            match employee.status {
                EmployeeStatus::Active => true,
                EmployeeStatus::Suspended(until_timestamp) => {
                    env.ledger().timestamp() >= until_timestamp
                },
                EmployeeStatus::Terminated => false,
            }
        } else {
            false
        }
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN).expect("Admin not set")
    }

    pub fn get_token_contract(env: Env) -> Address {
        env.storage().instance().get(&TOKEN).expect("Token contract not set")
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        Self::require_admin(&env);
        env.storage().instance().set(&ADMIN, &new_admin);
        Self::publish_event(&env, "admin_changed", &new_admin);
    }

    pub fn set_token_contract(env: Env, new_token_contract: Address) {
        Self::require_admin(&env);
        env.storage().instance().set(&TOKEN, &new_token_contract);
        Self::publish_event(&env, "token_contract_changed", &new_token_contract);
    }

    fn require_admin(env: &Env) {
        let admin: Address = env.storage().instance().get(&ADMIN).expect("Admin not set");
        admin.require_auth();
    }

    fn publish_event(env: &Env, event_type: &str, employee_address: &Address) {
        let topics = (symbol_short!("EMS"), Symbol::new(env, event_type));
        env.events().publish(topics, employee_address);
    }
}

#[cfg(test)]
mod test;