#![cfg(test)]
extern crate std;

use crate::{Ems, EmsClient};
use crate::types::EmployeeRank;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

// Import the SepToken from the external crate for testing
#[cfg(test)]
use sep41_token::{SepToken, SepTokenClient};

fn create_token<'a>(env: &Env, admin: &Address) -> SepTokenClient<'a> {
    let token = SepTokenClient::new(env, &env.register(SepToken, ()));
    token.initialize(
        admin,
        &7,
        &String::from_str(env, "Company Token"),
        &String::from_str(env, "COMP"),
    );
    token
}

fn create_ems<'a>(env: &Env) -> EmsClient<'a> {
    EmsClient::new(env, &env.register(Ems, ()))
}

fn setup_test_environment() -> (Env, Address, EmsClient<'static>, SepTokenClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token = create_token(&env, &admin);
    let ems = create_ems(&env);
    let token_address = token.address.clone();
    
    // Initialize EMS with the token
    ems.initialize(&admin, &token_address);
    
    // Mint some tokens to admin for payroll
    token.mint(&admin, &100_000_000_000_000i128); // 100B tokens with 7 decimals
    
    (env, admin, ems, token, token_address)
}

#[test]
fn test_initialize_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token = create_token(&env, &admin);
    let ems = create_ems(&env);
    
    // Should initialize successfully
    ems.initialize(&admin, &token.address);
    
    // Verify admin is set
    assert_eq!(ems.get_admin(), admin);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_initialize_already_initialized() {
    let (_, admin, ems, _, token_address) = setup_test_environment();
    
    ems.initialize(&admin, &token_address);
}

#[test]
#[should_panic(expected = "HostError: Error(Storage, MissingValue)")]
fn test_initialize_token_not_initialized() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let ems = create_ems(&env);
    let fake_token_address = Address::generate(&env);
    
    ems.initialize(&admin, &fake_token_address);
}

#[test]
fn test_add_employee_success() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64; // 1000 tokens with 7 decimals
    
    // Add employee
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    
    // Verify employee was added
    assert!(ems.employee_exists(&employee_address));
    
    let employee = ems.get_employee(&employee_address).unwrap();
    assert_eq!(employee.address, employee_address);
    assert_eq!(employee.rank, EmployeeRank::Junior);
    assert_eq!(employee.weekly_salary, weekly_salary);
    assert_eq!(employee.is_active, true);
    assert_eq!(employee.is_suspended, false);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_add_employee_already_exists() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee first time
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    
    ems.add_employee(&employee_address, &EmployeeRank::Senior, &weekly_salary);
}

#[test]
fn test_remove_employee_success() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add then remove employee
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    assert!(ems.employee_exists(&employee_address));
    
    ems.remove_employee(&employee_address);
    assert!(!ems.employee_exists(&employee_address));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_remove_employee_not_exists() {
    let (env, _, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    
    ems.remove_employee(&employee_address);
}

#[test]
fn test_update_salary_success() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let initial_salary = 1000_0000000u64;
    let new_salary = 1500_0000000u64;
    
    // Add employee and update salary
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &initial_salary);
    ems.update_salary(&employee_address, &new_salary);
    
    let employee = ems.get_employee(&employee_address).unwrap();
    assert_eq!(employee.weekly_salary, new_salary);
}

#[test]
fn test_promote_employee_success() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee and promote
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    ems.promote_employee(&employee_address, &EmployeeRank::Senior);
    
    let employee = ems.get_employee(&employee_address).unwrap();
    assert_eq!(employee.rank, EmployeeRank::Senior);
}

#[test]
fn test_suspend_employee_success() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee and suspend
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    ems.suspend_employee(&employee_address);
    
    let employee = ems.get_employee(&employee_address).unwrap();
    assert!(employee.is_suspended);
}

#[test]
fn test_unsuspend_employee_success() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee, suspend, then unsuspend
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    ems.suspend_employee(&employee_address);
    
    let employee = ems.get_employee(&employee_address).unwrap();
    assert!(employee.is_suspended);
    
    ems.unsuspend_employee(&employee_address);
    
    let employee = ems.get_employee(&employee_address).unwrap();
    assert!(!employee.is_suspended);
}

#[test]
fn test_is_payment_due_new_employee() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    
    assert!(ems.is_payment_due(&employee_address));
}

#[test]
fn test_payment_info() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    
    let (salary, last_paid, is_due) = ems.get_payment_info(&employee_address);
    assert_eq!(salary, weekly_salary);
    assert_eq!(last_paid, None); // Never been paid
    assert!(is_due);
}

#[test]
fn test_pay_employee_success() {
    let (env, admin, ems, token, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    
    // Check initial balances
    let admin_initial_balance = token.balance(&admin);
    let employee_initial_balance = token.balance(&employee_address);
    
    // Pay employee
    let result = ems.pay_employee(&employee_address);
    assert!(result);
    
    // Check balances after payment
    let admin_final_balance = token.balance(&admin);
    let employee_final_balance = token.balance(&employee_address);
    
    assert_eq!(admin_final_balance, admin_initial_balance - weekly_salary as i128);
    assert_eq!(employee_final_balance, employee_initial_balance + weekly_salary as i128);
    
    // Employee should no longer be due for payment
    assert!(!ems.is_payment_due(&employee_address));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_pay_employee_not_exists() {
    let (env, _, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    
    // Should fail when employee doesn't exist
    ems.pay_employee(&employee_address);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")]
fn test_pay_employee_suspended() {
    let (env, _admin, ems, _, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee and suspend
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    ems.suspend_employee(&employee_address);
    
    // Should fail when trying to pay suspended employee
    ems.pay_employee(&employee_address);
}

#[test]
fn test_pay_employee_timing_logic() {
    let (env, admin, ems, token, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let weekly_salary = 1000_0000000u64;
    
    // Add employee and pay once
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &weekly_salary);
    
    // Get initial balances
    let admin_initial_balance = token.balance(&admin);
    let employee_initial_balance = token.balance(&employee_address);
    
    // First payment should succeed
    let result = ems.pay_employee(&employee_address);
    assert!(result);
    
    // Verify balances changed
    let admin_after_first = token.balance(&admin);
    let employee_after_first = token.balance(&employee_address);
    assert_eq!(admin_after_first, admin_initial_balance - weekly_salary as i128);
    assert_eq!(employee_after_first, employee_initial_balance + weekly_salary as i128);
    
    // Should not be due for payment immediately after being paid
    assert!(!ems.is_payment_due(&employee_address));
    
    // Test that we can query payment info correctly
    let (salary, last_paid, is_due) = ems.get_payment_info(&employee_address);
    assert_eq!(salary, weekly_salary);
    assert!(last_paid.is_some()); // Should have a last paid timestamp
    assert!(!is_due); // Should not be due immediately after payment
    
    // Verify that trying to pay again immediately would fail by checking is_due
    assert!(!ems.is_payment_due(&employee_address)); // Not due for payment yet
}

#[test]
fn test_multiple_employees_payment() {
    let (env, admin, ems, token, _) = setup_test_environment();
    
    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);
    let employee3 = Address::generate(&env);
    
    let salary1 = 1000_0000000u64;
    let salary2 = 1500_0000000u64;
    let salary3 = 2000_0000000u64;
    
    // Add multiple employees
    ems.add_employee(&employee1, &EmployeeRank::Junior, &salary1);
    ems.add_employee(&employee2, &EmployeeRank::Mid, &salary2);
    ems.add_employee(&employee3, &EmployeeRank::Senior, &salary3);
    
    // Pay all employees
    let admin_initial_balance = token.balance(&admin);
    
    ems.pay_employee(&employee1);
    ems.pay_employee(&employee2);
    ems.pay_employee(&employee3);
    
    let total_paid = (salary1 + salary2 + salary3) as i128;
    let admin_final_balance = token.balance(&admin);
    
    assert_eq!(admin_final_balance, admin_initial_balance - total_paid);
    
    // Verify individual balances
    assert_eq!(token.balance(&employee1), salary1 as i128);
    assert_eq!(token.balance(&employee2), salary2 as i128);
    assert_eq!(token.balance(&employee3), salary3 as i128);
}

#[test]
fn test_employee_full_lifecycle() {
    let (env, _admin, ems, token, _) = setup_test_environment();
    
    let employee_address = Address::generate(&env);
    let initial_salary = 1000_0000000u64;
    let new_salary = 1500_0000000u64;
    
    // 1. Add employee
    ems.add_employee(&employee_address, &EmployeeRank::Junior, &initial_salary);
    assert!(ems.employee_exists(&employee_address));
    
    // 2. Pay employee
    ems.pay_employee(&employee_address);
    assert_eq!(token.balance(&employee_address), initial_salary as i128);
    
    // 3. Promote employee
    ems.promote_employee(&employee_address, &EmployeeRank::Mid);
    let employee = ems.get_employee(&employee_address).unwrap();
    assert_eq!(employee.rank, EmployeeRank::Mid);
    
    // 4. Update salary
    ems.update_salary(&employee_address, &new_salary);
    let employee = ems.get_employee(&employee_address).unwrap();
    assert_eq!(employee.weekly_salary, new_salary);
    
    // 5. Remove employee
    ems.remove_employee(&employee_address);
    assert!(!ems.employee_exists(&employee_address));
}
