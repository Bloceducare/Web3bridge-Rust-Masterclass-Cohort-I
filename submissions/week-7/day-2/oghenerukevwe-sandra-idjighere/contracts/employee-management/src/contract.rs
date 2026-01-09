use crate::interface::{EmployeeManagementInterface, EmployeeManagementAdminInterface};
use crate::storage::{
    address_is_employee, decrement_employee_count, employee_exists, get_employee_count,
    get_employee_id_by_address, get_next_employee_id, increment_employee_count, is_initialized,
    read_admin, read_employee, read_institution, remove_employee, set_initialized, write_admin,
    write_employee, write_institution,
};
use crate::types::{
    Employee, EmployeeAddedEvent, EmployeePromotedEvent, EmployeeRank, EmployeeStatus,
    EmployeeStatusChangedEvent, Institution, SalaryPaidEvent,
};
use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};
use soroban_sdk::token::TokenClient;

#[contract]
pub struct EmployeeManagement;

#[contractimpl]
impl EmployeeManagementInterface for EmployeeManagement {
    fn initialize(
        env: Env,
        admin: Address,
        institution_name: String,
        token_contract: Address,
        base_salary_amount: i128,
        min_promotion_interval: u64,
    ) {
        if is_initialized(&env) {
            panic!("contract already initialized");
        }

        // Validate inputs
        if institution_name.len() == 0 || institution_name.len() > 64 {
            panic!("institution name must be 1-64 characters");
        }

        if base_salary_amount <= 0 {
            panic!("base salary must be positive");
        }

        if min_promotion_interval == 0 {
            panic!("promotion interval must be positive");
        }

        let institution = Institution {
            name: institution_name,
            admin: admin.clone(),
            token_contract,
            base_salary_amount,
            min_promotion_interval,
        };

        write_institution(&env, &institution);
        write_admin(&env, &admin);
        set_initialized(&env);
    }

    fn add_employee(
        env: Env,
        employee_address: Address,
        name: String,
        rank: EmployeeRank,
        department: String,
    ) -> u64 {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        // Validate inputs
        if name.len() == 0 || name.len() > 64 {
            panic!("employee name must be 1-64 characters");
        }

        if department.len() == 0 || department.len() > 32 {
            panic!("department must be 1-32 characters");
        }

        // Check if address is already an employee
        if address_is_employee(&env, &employee_address) {
            panic!("address is already an employee");
        }

        let institution = read_institution(&env);
        let employee_id = get_next_employee_id(&env);
        let current_time = env.ledger().sequence() as u64;

        let employee = Employee {
            id: employee_id,
            address: employee_address.clone(),
            name,
            rank: rank.clone(),
            status: EmployeeStatus::Active,
            base_salary: institution.base_salary_amount,
            hire_date: current_time,
            last_promotion: None,
            department,
        };

        write_employee(&env, &employee);
        increment_employee_count(&env);

        // Emit event
        let event = EmployeeAddedEvent {
            employee_id,
            employee_address,
            rank,
            salary: employee.current_salary(),
        };
        env.events().publish(("employee_added", employee_id), event);

        employee_id
    }

    fn remove_employee(env: Env, employee_id: u64) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        if !employee_exists(&env, employee_id) {
            panic!("employee not found");
        }

        remove_employee(&env, employee_id);
        decrement_employee_count(&env);

        env.events().publish(("employee_removed", employee_id), ());
    }

    fn update_employee(
        env: Env,
        employee_id: u64,
        name: Option<String>,
        department: Option<String>,
        base_salary: Option<i128>,
    ) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let mut employee = read_employee(&env, employee_id).unwrap_or_else(|| {
            panic!("employee not found");
        });

        // Update fields if provided
        if let Some(new_name) = name {
            if new_name.len() == 0 || new_name.len() > 64 {
                panic!("employee name must be 1-64 characters");
            }
            employee.name = new_name;
        }

        if let Some(new_department) = department {
            if new_department.len() == 0 || new_department.len() > 32 {
                panic!("department must be 1-32 characters");
            }
            employee.department = new_department;
        }

        if let Some(new_base_salary) = base_salary {
            if new_base_salary <= 0 {
                panic!("base salary must be positive");
            }
            employee.base_salary = new_base_salary;
        }

        write_employee(&env, &employee);

        env.events().publish(("employee_updated", employee_id), ());
    }

    fn promote_employee(env: Env, employee_id: u64) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let mut employee = read_employee(&env, employee_id).unwrap_or_else(|| {
            panic!("employee not found");
        });

        let institution = read_institution(&env);
        let current_time = env.ledger().sequence() as u64;

        // Check if employee is eligible for promotion
        if !employee.is_promotion_eligible(current_time, institution.min_promotion_interval) {
            panic!("employee not eligible for promotion");
        }

        let old_rank = employee.rank.clone();
        let new_rank = employee.rank.next_rank().unwrap_or_else(|| {
            panic!("employee is already at the highest rank");
        });

        employee.rank = new_rank.clone();
        employee.last_promotion = Some(current_time);

        write_employee(&env, &employee);

        // Emit event
        let event = EmployeePromotedEvent {
            employee_id,
            old_rank,
            new_rank,
            new_salary: employee.current_salary(),
        };
        env.events().publish(("employee_promoted", employee_id), event);
    }

    fn suspend_employee(env: Env, employee_id: u64) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let mut employee = read_employee(&env, employee_id).unwrap_or_else(|| {
            panic!("employee not found");
        });

        if employee.status == EmployeeStatus::Suspended {
            panic!("employee is already suspended");
        }

        if employee.status == EmployeeStatus::Terminated {
            panic!("cannot suspend terminated employee");
        }

        let old_status = employee.status.clone();
        employee.status = EmployeeStatus::Suspended;

        write_employee(&env, &employee);

        // Emit event
        let event = EmployeeStatusChangedEvent {
            employee_id,
            old_status,
            new_status: EmployeeStatus::Suspended,
        };
        env.events().publish(("employee_status_changed", employee_id), event);
    }

    fn reactivate_employee(env: Env, employee_id: u64) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let mut employee = read_employee(&env, employee_id).unwrap_or_else(|| {
            panic!("employee not found");
        });

        if employee.status == EmployeeStatus::Active {
            panic!("employee is already active");
        }

        if employee.status == EmployeeStatus::Terminated {
            panic!("cannot reactivate terminated employee");
        }

        let old_status = employee.status.clone();
        employee.status = EmployeeStatus::Active;

        write_employee(&env, &employee);

        // Emit event
        let event = EmployeeStatusChangedEvent {
            employee_id,
            old_status,
            new_status: EmployeeStatus::Active,
        };
        env.events().publish(("employee_status_changed", employee_id), event);
    }

    fn pay_salary(env: Env, employee_id: u64) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let employee = read_employee(&env, employee_id).unwrap_or_else(|| {
            panic!("employee not found");
        });

        if employee.status != EmployeeStatus::Active {
            panic!("can only pay salary to active employees");
        }

        let institution = read_institution(&env);
        let token_client = TokenClient::new(&env, &institution.token_contract);
        let salary_amount = employee.current_salary();

        // Transfer tokens from admin to employee
        token_client.transfer(&admin, &employee.address, &salary_amount);

        // Emit event
        let event = SalaryPaidEvent {
            employee_id,
            amount: salary_amount,
            period: env.ledger().sequence() as u64,
        };
        env.events().publish(("salary_paid", employee_id), event);
    }

    fn pay_all_salaries(env: Env) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let total_employees = get_employee_count(&env);
        let institution = read_institution(&env);
        let token_client = TokenClient::new(&env, &institution.token_contract);

        // Pay salary to all active employees
        for employee_id in 1..=total_employees {
            if let Some(employee) = read_employee(&env, employee_id) {
                if employee.status == EmployeeStatus::Active {
                    let salary_amount = employee.current_salary();

                    // Transfer tokens from admin to employee
                    token_client.transfer(&admin, &employee.address, &salary_amount);

                    // Emit event
                    let event = SalaryPaidEvent {
                        employee_id,
                        amount: salary_amount,
                        period: env.ledger().sequence() as u64,
                    };
                    env.events().publish(("salary_paid", employee_id), event);
                }
            }
        }
    }

    fn get_employee(env: Env, employee_id: u64) -> Employee {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        read_employee(&env, employee_id).unwrap_or_else(|| {
            panic!("employee not found");
        })
    }

    fn get_employee_by_address(env: Env, employee_address: Address) -> Option<u64> {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        get_employee_id_by_address(&env, &employee_address)
    }

    fn get_employees(env: Env, start_id: u64, limit: u32) -> Vec<Employee> {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let mut employees = Vec::new(&env);
        let mut count = 0u32;
        let mut current_id = start_id;

        while count < limit {
            if let Some(employee) = read_employee(&env, current_id) {
                employees.push_back(employee);
                count += 1;
            }
            current_id += 1;

            // Break if we've checked beyond reasonable bounds
            if current_id > start_id + (limit as u64) * 2 {
                break;
            }
        }

        employees
    }

    fn get_institution(env: Env) -> Institution {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        read_institution(&env)
    }

    fn get_employee_count(env: Env) -> u64 {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        get_employee_count(&env)
    }

    fn initialized(env: Env) -> bool {
        is_initialized(&env)
    }
}

#[contractimpl]
impl EmployeeManagementAdminInterface for EmployeeManagement {
    fn set_admin(env: Env, new_admin: Address) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        write_admin(&env, &new_admin);

        // Update institution admin as well
        let mut institution = read_institution(&env);
        institution.admin = new_admin.clone();
        write_institution(&env, &institution);

        env.events().publish(("admin_changed", &admin, &new_admin), ());
    }

    fn get_admin(env: Env) -> Address {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        read_admin(&env)
    }

    fn update_institution(
        env: Env,
        name: Option<String>,
        token_contract: Option<Address>,
        base_salary_amount: Option<i128>,
        min_promotion_interval: Option<u64>,
    ) {
        if !is_initialized(&env) {
            panic!("contract not initialized");
        }

        let admin = read_admin(&env);
        admin.require_auth();

        let mut institution = read_institution(&env);

        // Update fields if provided
        if let Some(new_name) = name {
            if new_name.len() == 0 || new_name.len() > 64 {
                panic!("institution name must be 1-64 characters");
            }
            institution.name = new_name;
        }

        if let Some(new_token_contract) = token_contract {
            institution.token_contract = new_token_contract;
        }

        if let Some(new_base_salary) = base_salary_amount {
            if new_base_salary <= 0 {
                panic!("base salary must be positive");
            }
            institution.base_salary_amount = new_base_salary;
        }

        if let Some(new_interval) = min_promotion_interval {
            if new_interval == 0 {
                panic!("promotion interval must be positive");
            }
            institution.min_promotion_interval = new_interval;
        }

        write_institution(&env, &institution);

        env.events().publish(("institution_updated",), ());
    }
}
