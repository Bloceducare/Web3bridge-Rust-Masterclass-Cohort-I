use soroban_sdk::{Address, Env};
use crate::{traits::EmployeeManagementTrait, storage::{self, Employee}, events, token_interface::TokenClient};

pub struct EmployeeManagement;

impl EmployeeManagementTrait for EmployeeManagement {
    fn initialize(env: Env, admin: Address) {
        storage::set_admin(&env, &admin);
        storage::set_employee_count(&env, 0);
    }

    fn add_employee(env: Env, employee_id: Address, institution: Address, salary: i128, rank: u32) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        assert!(storage::get_employee(&env, &employee_id).is_none(), "Employee already exists");
        assert!(salary > 0, "Salary must be positive");

        let employee = Employee {
            id: employee_id.clone(),
            institution: institution.clone(),
            salary,
            rank,
            is_active: true,
            is_suspended: false,
        };

        storage::set_employee(&env, &employee);
        
        let count = storage::get_employee_count(&env);
        storage::set_employee_count(&env, count + 1);

        events::employee_added_event(&env, employee_id, institution, salary, rank);
    }

    fn remove_employee(env: Env, employee_id: Address) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        assert!(storage::get_employee(&env, &employee_id).is_some(), "Employee does not exist");

        storage::remove_employee(&env, &employee_id);
        
        let count = storage::get_employee_count(&env);
        storage::set_employee_count(&env, count - 1);

        events::employee_removed_event(&env, employee_id);
    }

    fn update_employee_salary(env: Env, employee_id: Address, new_salary: i128) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        let mut employee = storage::get_employee(&env, &employee_id).expect("Employee does not exist");
        assert!(new_salary > 0, "Salary must be positive");

        employee.salary = new_salary;
        storage::set_employee(&env, &employee);

        events::employee_updated_event(&env, employee_id, new_salary);
    }

    fn promote_employee(env: Env, employee_id: Address, new_rank: u32) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        let mut employee = storage::get_employee(&env, &employee_id).expect("Employee does not exist");
        assert!(new_rank > employee.rank, "New rank must be higher than current rank");

        employee.rank = new_rank;
        storage::set_employee(&env, &employee);

        events::employee_promoted_event(&env, employee_id, new_rank);
    }

    fn suspend_employee(env: Env, employee_id: Address) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        let mut employee = storage::get_employee(&env, &employee_id).expect("Employee does not exist");
        assert!(!employee.is_suspended, "Employee is already suspended");

        employee.is_suspended = true;
        storage::set_employee(&env, &employee);

        events::employee_suspended_event(&env, employee_id);
    }

    fn reinstate_employee(env: Env, employee_id: Address) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        let mut employee = storage::get_employee(&env, &employee_id).expect("Employee does not exist");
        assert!(employee.is_suspended, "Employee is not suspended");

        employee.is_suspended = false;
        storage::set_employee(&env, &employee);

        events::employee_reinstated_event(&env, employee_id);
    }

    fn get_employee(env: Env, employee_id: Address) -> Option<Employee> {
        storage::get_employee(&env, &employee_id)
    }

    fn get_employee_count(env: Env) -> u32 {
        storage::get_employee_count(&env)
    }

    fn set_token_contract(env: Env, token_contract: Address) {
        let admin = storage::get_admin(&env).expect("Contract not initialized");
        admin.require_auth();

        storage::set_token_contract(&env, &token_contract);
    }

    fn pay_salary(env: Env, from: Address, employee_id: Address) {
        from.require_auth();

        let employee = storage::get_employee(&env, &employee_id).expect("Employee does not exist");
        assert!(employee.is_active, "Employee is not active");
        assert!(!employee.is_suspended, "Employee is suspended");

        let token_contract = storage::get_token_contract(&env).expect("Token contract not set");
        
        let token_client = TokenClient::new(&env, &token_contract);
        token_client.transfer(&from, &employee_id, &employee.salary);

        events::salary_paid_event(&env, employee_id, from, employee.salary);
    }
}
