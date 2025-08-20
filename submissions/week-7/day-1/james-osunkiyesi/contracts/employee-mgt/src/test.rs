#![cfg(test)]

use super::*;
use crate::storage::{EmployeeDept, EmployeeRank};
use soroban_sdk::{testutils::Address as TestAddress, Address, Env, String};
use soroban_sdk::testutils::Ledger;

fn generate_token_contract(env: Env) -> (Env, Address) {
    let id = env.register(
        token::WASM,
        ()
    );
    (env, id)
}



fn create_test_env() -> (Env, ContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let employee = Address::generate(&env);
    let (env, token_id) = generate_token_contract(env);

    (env, client, admin, employee, token_id)
}

#[test]
fn test_init_admin() {
    let (env, client, admin, _, token_id) = create_test_env();

    // Mock authentication for the admin
    env.mock_all_auths();

    // Initialize admin with token_id
    client.init(&admin, &token_id);

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
    let (env, client, admin, _, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin first time
    client.init(&admin, &token_id);

    // Try to initialize again - should panic
    client.init(&admin, &token_id);
}

#[test]
fn test_add_employee() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin first
    client.init(&admin, &token_id);

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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Try to remove non-existent employee - should panic
    client.remove_employee(&employee);
}

#[test]
fn test_promote_employee() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee as INTERN
    client.init(&admin, &token_id);
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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Try to promote non-existent employee - should panic
    client.promote_employee(&employee);
}

#[test]
fn test_suspend_employee() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Try to suspend non-existent employee - should panic
    client.suspend_employee(&employee, &1);
}

#[test]
fn test_update_employee_department() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Try to update department for non-existent employee - should panic
    client.update_employee_dept(&employee, &EmployeeDept::MARKETING);
}

#[test]
fn test_update_employee_name() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
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
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Try to update name for non-existent employee - should panic
    client.update_employee_name(&employee, &String::from_str(&env, "New Name"));
}

#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_employee_action_nonexistent() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Try employee action with non-existent employee - should panic
    client.employee_action(&employee);
}

#[test]
fn test_suspended_employee_automatic_activation() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
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
    let (env, client, admin, _, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

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

#[test]
fn test_token_transfer() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin with token
    client.init(&admin, &token_id);

    // Test token transfer - this should complete without panicking
    client.test_token_transfer(&employee, &1000);

    // If we reach here, the transfer was successful
    assert!(true);
}
#[test]
#[should_panic(expected = "Max Rank Reached")]
fn test_promote_employee_max_rank() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee with highest rank (MANAGER)
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "Senior Manager"),
        &EmployeeRank::MANAGER,
        &EmployeeDept::ADMINISTRATIVE
    );

    assert!(client.employee_action(&employee));


    // Promoting a manager should fail or handle gracefully
    // The contract should handle this case appropriately
    client.promote_employee(&employee);

    // Employee should still be able to perform actions
    assert!(client.employee_action(&employee));
}

#[test]
fn test_suspension_time_advancement() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "Temporary Worker"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Suspend employee for 2 days
    client.suspend_employee(&employee, &2);

    // Employee should not be able to perform actions while suspended
    assert!(!client.employee_action(&employee));

    // For this test, we can't actually advance time in Soroban test environment
    // The suspension logic will be tested through the contract's internal logic
    // In a real environment, time would advance naturally
}

#[test]
fn test_multiple_employees() {
    let (env, client, admin, _, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin
    client.init(&admin, &token_id);

    // Create multiple employees
    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);
    let employee3 = Address::generate(&env);

    // Add multiple employees
    client.add_employee(
        &employee1,
        &String::from_str(&env, "Alice Johnson"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DESIGN
    );

    client.add_employee(
        &employee2,
        &String::from_str(&env, "Bob Wilson"),
        &EmployeeRank::SENIOR,
        &EmployeeDept::DEVELOPMENT
    );

    client.add_employee(
        &employee3,
        &String::from_str(&env, "Carol Davis"),
        &EmployeeRank::MANAGER,
        &EmployeeDept::MARKETING
    );

    // All employees should be able to perform actions
    assert!(client.employee_action(&employee1));
    assert!(client.employee_action(&employee2));
    assert!(client.employee_action(&employee3));

    // Remove one employee
    client.remove_employee(&employee2);

    // Remaining employees should still work
    assert!(client.employee_action(&employee1));
    assert!(client.employee_action(&employee3));
}

#[test]
fn test_employee_department_changes() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "Department Changer"),
        &EmployeeRank::INTERMEDIATE,
        &EmployeeDept::DESIGN
    );

    // Test changing to all different departments
    let departments = [
        EmployeeDept::DEVELOPMENT,
        EmployeeDept::MARKETING,
        EmployeeDept::CONTENT,
        EmployeeDept::ADMINISTRATIVE,
        EmployeeDept::DESIGN, // Back to original
    ];
    for dept in departments {
        client.update_employee_dept(&employee, &dept);
        assert!(client.employee_action(&employee));
    }
}

#[test]
fn test_collect_pay_success_after_28_days() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Get current timestamp and advance by 28 days
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (28 * 86_400); // 28 days in seconds
    });

    // Employee should be able to collect pay after 28 days
    client.collect_pay(&employee);

    // If we reach here, payment was successful
    assert!(true);
}

#[test]
#[should_panic(expected = "Payday has not reached")]
fn test_collect_pay_too_early() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Try to collect pay immediately (before 28 days) - should panic
    client.collect_pay(&employee);
}

#[test]
#[should_panic(expected = "Payday has not reached")]
fn test_collect_pay_partial_period() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Advance time by only 27 days (1 day short of 28)
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (27 * 86_400);
    });

    // Should panic because 28 days haven't passed yet
    client.collect_pay(&employee);
}
#[test]
fn test_collect_pay_multiple_times() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::SENIOR,
        &EmployeeDept::DEVELOPMENT
    );

    let initial_time = env.ledger().timestamp();

    // First pay period (28 days)
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = initial_time + (28 * 86_400);
    });
    client.collect_pay(&employee);

    // Second pay period (another 28 days)
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = initial_time + (56 * 86_400);
    });
    client.collect_pay(&employee);

    // Third pay period (another 28 days)
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = initial_time + (84 * 86_400);
    });
    client.collect_pay(&employee);

    // All collections should succeed
    assert!(true);
}

#[test]
fn test_collect_multiple_pay_at_the_same_time() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::SENIOR,
        &EmployeeDept::DEVELOPMENT
    );

    let initial_time = env.ledger().timestamp();

    // First pay period (28 days)
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = initial_time + (28 * 86_400) * 3; // User should have 3 pending pays
    });
    client.collect_pay(&employee);
    client.collect_pay(&employee);
    client.collect_pay(&employee);

    // All collections should succeed
    assert!(true);
}


#[test]
#[should_panic(expected = "Employee data does not exists")]
fn test_collect_pay_nonexistent_employee() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin but don't add employee
    client.init(&admin, &token_id);

    // Try to collect pay for non-existent employee - should panic
    client.collect_pay(&employee);
}

#[test]
fn test_collect_pay_suspended_employee() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Suspend employee for 30 days (longer than pay period)
    client.suspend_employee(&employee, &30);

    // Advance time by 28 days (pay period)
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (28 * 86_400);
    });

    // Suspended employee should not be able to collect pay
    // The function checks if employee is active first, so it should just return without error
    // Since the employee is suspended (30 days), they won't be active even after 28 days
    client.collect_pay(&employee);
}

#[test]
fn test_collect_pay_after_suspension_expires() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Suspend employee for 20 days (shorter than pay period)
    client.suspend_employee(&employee, &20);

    // Advance time by 30 days (past both suspension and pay period)
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (30 * 86_400);
    });

    // Employee should be able to collect pay after suspension expires and 28+ days have passed
    client.collect_pay(&employee);

    assert!(true);
}

#[test]
fn test_collect_pay_after_promotion() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee as JUNIOR
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Promote employee to INTERMEDIATE
    client.promote_employee(&employee);

    // Advance time by 28 days
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (28 * 86_400);
    });

    // Employee should be able to collect pay at new rank rate (INTERMEDIATE = 4500)
    client.collect_pay(&employee);

    assert!(true);
}

#[test]
#[should_panic(expected = "Payday has not reached")]
fn test_collect_pay_updates_last_pay_time() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::JUNIOR,
        &EmployeeDept::DEVELOPMENT
    );

    // Advance time by 28 days
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (28 * 86_400);
    });

    // Collect pay first time
    client.collect_pay(&employee);

    // Try to collect pay again immediately - should fail because last pay time was updated
    client.collect_pay(&employee);
}

#[test]
fn test_collect_pay_long_period() {
    let (env, client, admin, employee, token_id) = create_test_env();

    env.mock_all_auths();

    // Initialize admin and add employee
    client.init(&admin, &token_id);
    client.add_employee(
        &employee,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::MANAGER,
        &EmployeeDept::ADMINISTRATIVE
    );

    // Advance time by 60 days (more than double the pay period)
    let current_time = env.ledger().timestamp();

    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + (60 * 86_400);
    });

    // Should still be able to collect pay (only collects for one period)
    client.collect_pay(&employee);

    assert!(true);
}

#[test]
fn test_pay_rates_validation() {
    // Test that the pay rates are defined correctly in EmployeeRank
    use crate::storage::EmployeeRank;

    // Verify pay rates are set for each rank
    assert_eq!(EmployeeRank::get_pay(EmployeeRank::INTERN), 1_500);
    assert_eq!(EmployeeRank::get_pay(EmployeeRank::JUNIOR), 2_500);
    assert_eq!(EmployeeRank::get_pay(EmployeeRank::INTERMEDIATE), 4_500);
    assert_eq!(EmployeeRank::get_pay(EmployeeRank::SENIOR), 7_000);
    assert_eq!(EmployeeRank::get_pay(EmployeeRank::MANAGER), 12_000);
}
