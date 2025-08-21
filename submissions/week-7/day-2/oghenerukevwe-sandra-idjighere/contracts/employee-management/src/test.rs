#![cfg(test)]

use crate::EmployeeManagement;
use crate::types::{EmployeeRank, EmployeeStatus};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, Env, String,
};

// Import the auto-generated client
use crate::contract::EmployeeManagementClient;

fn create_employee_management_contract(env: &Env) -> EmployeeManagementClient<'_> {
    let contract_address = env.register(EmployeeManagement {}, ());
    EmployeeManagementClient::new(env, &contract_address)
}

fn create_mock_token_contract(env: &Env) -> Address {
    // For testing purposes, we'll use a mock address
    // In real tests, you'd deploy an actual SEP-41 token contract
    Address::generate(env)
}

fn advance_ledger(env: &Env, delta: u32) {
    env.ledger().with_mut(|li| {
        li.sequence_number += delta;
    });
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000, // 1000 tokens with 7 decimals
        &100, // 100 ledger sequences between promotions
    );

    assert_eq!(contract.initialized(), true);
    assert_eq!(contract.get_admin(), admin);
    
    let institution = contract.get_institution();
    assert_eq!(institution.name, String::from_str(&env, "Tech Corp"));
    assert_eq!(institution.admin, admin);
    assert_eq!(institution.token_contract, token_contract);
    assert_eq!(institution.base_salary_amount, 1000_0000000);
    assert_eq!(institution.min_promotion_interval, 100);
}

#[test]
#[should_panic(expected = "contract already initialized")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    // Should panic on second initialization
    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp 2"),
        &token_contract,
        &2000_0000000,
        &200,
    );
}

#[test]
fn test_add_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    assert_eq!(employee_id, 1);
    assert_eq!(contract.get_employee_count(), 1);

    let employee = contract.get_employee(&employee_id);
    assert_eq!(employee.id, 1);
    assert_eq!(employee.address, employee_addr);
    assert_eq!(employee.name, String::from_str(&env, "John Doe"));
    assert_eq!(employee.rank, EmployeeRank::Junior);
    assert_eq!(employee.status, EmployeeStatus::Active);
    assert_eq!(employee.department, String::from_str(&env, "Engineering"));
    assert_eq!(employee.current_salary(), 2000_0000000); // 1000 * 2 (Junior multiplier)

    // Test reverse lookup
    assert_eq!(contract.get_employee_by_address(&employee_addr), Some(1));
}

#[test]
#[should_panic(expected = "address is already an employee")]
fn test_add_duplicate_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    // Should panic when adding the same address again
    contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "Jane Doe"),
        &EmployeeRank::Senior,
        &String::from_str(&env, "Marketing"),
    );
}

#[test]
fn test_update_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    // Update employee information
    contract.update_employee(
        &employee_id,
        &Some(String::from_str(&env, "John Smith")),
        &Some(String::from_str(&env, "DevOps")),
        &Some(1500_0000000),
    );

    let employee = contract.get_employee(&employee_id);
    assert_eq!(employee.name, String::from_str(&env, "John Smith"));
    assert_eq!(employee.department, String::from_str(&env, "DevOps"));
    assert_eq!(employee.base_salary, 1500_0000000);
    assert_eq!(employee.current_salary(), 3000_0000000); // 1500 * 2 (Junior multiplier)
}

#[test]
fn test_promote_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    // Advance time to make employee eligible for promotion
    advance_ledger(&env, 101);

    contract.promote_employee(&employee_id);

    let employee = contract.get_employee(&employee_id);
    assert_eq!(employee.rank, EmployeeRank::Mid);
    assert_eq!(employee.current_salary(), 3000_0000000); // 1000 * 3 (Mid multiplier)
    assert!(employee.last_promotion.is_some());
}

#[test]
#[should_panic(expected = "employee not eligible for promotion")]
fn test_promote_employee_too_early() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    // Try to promote immediately (should fail)
    contract.promote_employee(&employee_id);
}

#[test]
fn test_suspend_and_reactivate_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    // Suspend employee
    contract.suspend_employee(&employee_id);
    let employee = contract.get_employee(&employee_id);
    assert_eq!(employee.status, EmployeeStatus::Suspended);

    // Reactivate employee
    contract.reactivate_employee(&employee_id);
    let employee = contract.get_employee(&employee_id);
    assert_eq!(employee.status, EmployeeStatus::Active);
}

#[test]
fn test_remove_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    assert_eq!(contract.get_employee_count(), 1);

    contract.remove_employee(&employee_id);

    assert_eq!(contract.get_employee_count(), 0);
    assert_eq!(contract.get_employee_by_address(&employee_addr), None);
}

#[test]
fn test_rank_system() {
    // Test rank multipliers
    assert_eq!(EmployeeRank::Intern.salary_multiplier(), 1);
    assert_eq!(EmployeeRank::Junior.salary_multiplier(), 2);
    assert_eq!(EmployeeRank::Mid.salary_multiplier(), 3);
    assert_eq!(EmployeeRank::Senior.salary_multiplier(), 5);
    assert_eq!(EmployeeRank::Lead.salary_multiplier(), 7);
    assert_eq!(EmployeeRank::Manager.salary_multiplier(), 10);
    assert_eq!(EmployeeRank::Director.salary_multiplier(), 15);
    assert_eq!(EmployeeRank::VP.salary_multiplier(), 20);
    assert_eq!(EmployeeRank::CEO.salary_multiplier(), 30);

    // Test rank progression
    assert_eq!(EmployeeRank::Intern.next_rank(), Some(EmployeeRank::Junior));
    assert_eq!(EmployeeRank::Junior.next_rank(), Some(EmployeeRank::Mid));
    assert_eq!(EmployeeRank::Mid.next_rank(), Some(EmployeeRank::Senior));
    assert_eq!(EmployeeRank::Senior.next_rank(), Some(EmployeeRank::Lead));
    assert_eq!(EmployeeRank::Lead.next_rank(), Some(EmployeeRank::Manager));
    assert_eq!(EmployeeRank::Manager.next_rank(), Some(EmployeeRank::Director));
    assert_eq!(EmployeeRank::Director.next_rank(), Some(EmployeeRank::VP));
    assert_eq!(EmployeeRank::VP.next_rank(), Some(EmployeeRank::CEO));
    assert_eq!(EmployeeRank::CEO.next_rank(), None);
}

#[test]
fn test_get_employees_pagination() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    // Add multiple employees
    let employee_names = ["Employee 1", "Employee 2", "Employee 3", "Employee 4", "Employee 5"];
    for name in employee_names.iter() {
        let employee_addr = Address::generate(&env);
        contract.add_employee(
            &employee_addr,
            &String::from_str(&env, name),
            &EmployeeRank::Junior,
            &String::from_str(&env, "Engineering"),
        );
    }

    // Test pagination
    let employees = contract.get_employees(&1, &3);
    assert_eq!(employees.len(), 3);

    let employees = contract.get_employees(&4, &3);
    assert_eq!(employees.len(), 2); // Only 2 remaining employees
}

#[test]
fn test_admin_functions() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    // Test set admin
    contract.set_admin(&new_admin);
    assert_eq!(contract.get_admin(), new_admin);

    // Test update institution
    let new_token_contract = create_mock_token_contract(&env);
    contract.update_institution(
        &Some(String::from_str(&env, "New Tech Corp")),
        &Some(new_token_contract.clone()),
        &Some(2000_0000000),
        &Some(200),
    );

    let institution = contract.get_institution();
    assert_eq!(institution.name, String::from_str(&env, "New Tech Corp"));
    assert_eq!(institution.token_contract, new_token_contract);
    assert_eq!(institution.base_salary_amount, 2000_0000000);
    assert_eq!(institution.min_promotion_interval, 200);
}

#[test]
#[should_panic(expected = "employee not found")]
fn test_operations_on_nonexistent_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    // Should panic when trying to get non-existent employee
    contract.get_employee(&999);
}

#[test]
#[should_panic(expected = "contract not initialized")]
fn test_operations_before_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    // Should panic when trying to add employee before initialization
    contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );
}

#[test]
#[should_panic(expected = "institution name must be 1-64 characters")]
fn test_initialize_invalid_institution_name() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, ""), // Empty name
        &token_contract,
        &1000_0000000,
        &100,
    );
}

#[test]
#[should_panic(expected = "base salary must be positive")]
fn test_initialize_invalid_base_salary() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &0, // Invalid salary
        &100,
    );
}

#[test]
#[should_panic(expected = "employee is already suspended")]
fn test_suspend_already_suspended_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    contract.suspend_employee(&employee_id);
    // Should panic when trying to suspend again
    contract.suspend_employee(&employee_id);
}

#[test]
#[should_panic(expected = "employee is already active")]
fn test_reactivate_already_active_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_contract = create_mock_token_contract(&env);
    let employee_addr = Address::generate(&env);
    let contract = create_employee_management_contract(&env);

    contract.initialize(
        &admin,
        &String::from_str(&env, "Tech Corp"),
        &token_contract,
        &1000_0000000,
        &100,
    );

    let employee_id = contract.add_employee(
        &employee_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &String::from_str(&env, "Engineering"),
    );

    // Should panic when trying to reactivate an already active employee
    contract.reactivate_employee(&employee_id);
}
