#![no_std]

mod sep41;
mod traits;

pub use sep41::*;
pub use traits::*;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct EmployeeSystem;

#[contractimpl]
impl EmployeeManagement for EmployeeSystem {
    fn initialize(env: Env, admin: Address, token_contract: Address) {
        if env.storage().instance().has(&"admin") {
            panic!("already initialized");
        }
        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"token", &token_contract);
    }

    fn add_employee(env: Env, employee: Address, rank: String, salary: i128) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        if salary < 0 {
            panic!("salary cannot be negative");
        }

        if env.storage().persistent().has(&employee) {
            panic!("employee already exists");
        }

        let employee_info = EmployeeInfo {
            rank,
            salary,
            is_active: true,
            last_paid: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&employee, &employee_info);
    }

    fn remove_employee(env: Env, employee: Address) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        if !env.storage().persistent().has(&employee) {
            panic!("employee not found");
        }

        env.storage().persistent().remove(&employee);
    }

    fn update_salary(env: Env, employee: Address, new_salary: i128) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        if new_salary < 0 {
            panic!("salary cannot be negative");
        }

        let mut employee_info: EmployeeInfo = env
            .storage()
            .persistent()
            .get(&employee)
            .expect("employee not found");

        employee_info.salary = new_salary;
        env.storage().persistent().set(&employee, &employee_info);
    }

    fn promote_employee(env: Env, employee: Address, new_rank: String, new_salary: i128) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        if new_salary < 0 {
            panic!("salary cannot be negative");
        }

        let mut employee_info: EmployeeInfo = env
            .storage()
            .persistent()
            .get(&employee)
            .expect("employee not found");

        employee_info.rank = new_rank;
        employee_info.salary = new_salary;
        env.storage().persistent().set(&employee, &employee_info);
    }

    fn suspend_employee(env: Env, employee: Address) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        let mut employee_info: EmployeeInfo = env
            .storage()
            .persistent()
            .get(&employee)
            .expect("employee not found");

        employee_info.is_active = false;
        env.storage().persistent().set(&employee, &employee_info);
    }

    fn reactivate_employee(env: Env, employee: Address) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        let mut employee_info: EmployeeInfo = env
            .storage()
            .persistent()
            .get(&employee)
            .expect("employee not found");

        employee_info.is_active = true;
        env.storage().persistent().set(&employee, &employee_info);
    }

    fn pay_salary(env: Env, employee: Address) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();

        let mut employee_info: EmployeeInfo = env
            .storage()
            .persistent()
            .get(&employee)
            .expect("employee not found");

        if !employee_info.is_active {
            panic!("employee is suspended");
        }

        let token_contract: Address = env.storage().instance().get(&"token").expect("token not set");
        let token_client = Sep41Client::new(&env, &token_contract);

        token_client.transfer(&admin, &employee, &employee_info.salary);

        employee_info.last_paid = env.ledger().timestamp();
        env.storage().persistent().set(&employee, &employee_info);
    }

    fn get_employee(env: Env, employee: Address) -> Option<EmployeeInfo> {
        env.storage().persistent().get(&employee)
    }

    fn is_active(env: Env, employee: Address) -> bool {
        if let Some(employee_info) = env.storage().persistent().get::<Address, EmployeeInfo>(&employee) {
            employee_info.is_active
        } else {
            false
        }
    }
}

#[contractimpl]
impl EmployeeSystem {
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&"admin").expect("not initialized")
    }

    pub fn get_token_contract(env: Env) -> Address {
        env.storage().instance().get(&"token").expect("not initialized")
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&"admin").expect("not initialized");
        admin.require_auth();
        env.storage().instance().set(&"admin", &new_admin);
    }
}

#[cfg(test)]
mod test;
