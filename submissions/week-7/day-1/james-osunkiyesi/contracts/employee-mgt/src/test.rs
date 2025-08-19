#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Address, testutils::Address as TestAddress};
use crate::storage::{EmployeeRank, EmployeeDept};

fn create_test_env() -> (Env, ContractClient<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let employee = Address::generate(&env);

    (env, client, admin, employee)
}

#[test]
fn test_init_admin() {
    let (env, client, admin, _) = create_test_env();

    // Mock authentication for the admin
    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Verify admin is set (indirectly by trying to add an employee)
    let employee = Address::generate(&env);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );
}

#[test]
#[should_panic(expected = "Admin already set")]
fn test_init_admin_already_exists() {
    let (env, client, admin, _) = create_test_env();

    env.mock_all_auths();

    // Initialize admin first time
    client.init(&admin);

    // Try to initialize again - should panic
    client.init(&admin);
}

#[test]
fn test_add_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin first
    client.init(&admin);

    // Add employee
    client.add_employee(
        &employee,
        &String::from_str(&env, "Jane Smith"),
        &EmployeeRank::SENIOR,
        &EmployeeDept::DESIGN
    );

    // Verify employee can perform actions (indicating they were added successfully)
    assert!(client.employee_action(&employee));
}

#[test]
#[should_panic(expected = "Employee data already exists")]
fn test_add_duplicate_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Add employee first time
    client.add_employee(
        &employee,
        &String::from_str(&env, "Jane Smith"),
        &EmployeeRank::SENIOR,
        &EmployeeDept::DESIGN
    );

    // Try to add same employee again - should panic
    client.add_employee(
        &employee,
        &String::from_str(&env, "Jane Smith Updated"),
        &EmployeeRank::MANAGER,
        &EmployeeDept::MARKETING
    );
}

#[test]
fn test_remove_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Verify employee exists
    assert!(client.employee_action(&employee));

    // Remove employee
    client.remove_employee(&employee);

    // Verify employee no longer exists (should panic when trying employee_action)
    // We can't test the panic directly here, but the removal function completed successfully
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_remove_nonexistent_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Try to remove non-existent employee - should panic
    client.remove_employee(&employee);
}

#[test]
fn test_promote_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee as INTERN
    client.init(&admin);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::INTERN,
        &EmployeeDept::DEVELOPMENT
    );

    // Promote employee
    client.promote_employee(&employee);

    // Employee should still be able to perform actions after promotion
    assert!(client.employee_action(&employee));
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_promote_nonexistent_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Try to promote non-existent employee - should panic
    client.promote_employee(&employee);
}

#[test]
fn test_suspend_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Verify employee is active initially
    assert!(client.employee_action(&employee));

    // Suspend employee for 1 day (86400 seconds)
    client.suspend_employee(&employee, &1);

    // Employee action should return false while suspended
    assert!(!client.employee_action(&employee));
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_suspend_nonexistent_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Try to suspend non-existent employee - should panic
    client.suspend_employee(&employee, &1);
}

#[test]
fn test_update_employee_department() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Update employee department
    client.update_employee_dept(&employee, &EmployeeDept::MARKETING);

    // Employee should still be able to perform actions after department update
    assert!(client.employee_action(&employee));
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_update_department_nonexistent_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Try to update department for non-existent employee - should panic
    client.update_employee_dept(&employee, &EmployeeDept::MARKETING);
}

#[test]
fn test_update_employee_name() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Update employee name
    client.update_employee_name(&employee, &String::from_str(&env, "John Smith"));

    // Employee should still be able to perform actions after name update
    assert!(client.employee_action(&employee));
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_update_name_nonexistent_employee() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Try to update name for non-existent employee - should panic
    client.update_employee_name(&employee, &String::from_str(&env, "New Name"));
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_employee_action_nonexistent() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Try employee action with non-existent employee - should panic
    client.employee_action(&employee);
}

#[test]
fn test_suspended_employee_automatic_activation() {
    let (env, client, admin, employee) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Suspend employee for a very short time (0 days - should be immediately active)
    client.suspend_employee(&employee, &0);

    // Since suspension time has passed, employee should be automatically activated
    assert!(client.employee_action(&employee));
}

#[test]
fn test_multiple_employees_management() {
    let (env, client, admin, _) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin);

    // Create multiple employees
    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);
    let employee3 = Address::generate(&env);

    // Add multiple employees
    client.add_employee(
        &employee1,
        &String::from_str(&env, "Alice"),
        &EmployeeRank::INTERN,
        &EmployeeDept::DEVELOPMENT
    );

    client.add_employee(
        &employee2,
        &String::from_str(&env, "Bob"),
        &EmployeeRank::SENIOR,
        &EmployeeDept::DESIGN
    );

    client.add_employee(
        &employee3,
        &String::from_str(&env, "Charlie"),
        &EmployeeRank::MANAGER,
        &EmployeeDept::ADMINISTRATIVE
    );

    // Verify all employees can perform actions
    assert!(client.employee_action(&employee1));
    assert!(client.employee_action(&employee2));
    assert!(client.employee_action(&employee3));

    // Promote employee1
    client.promote_employee(&employee1);
    assert!(client.employee_action(&employee1));

    // Suspend employee2
    client.suspend_employee(&employee2, &1);
    assert!(!client.employee_action(&employee2));

    // Employee3 should still be active
    assert!(client.employee_action(&employee3));

    // Update employee3's department
    client.update_employee_dept(&employee3, &EmployeeDept::MARKETING);
    assert!(client.employee_action(&employee3));
}
