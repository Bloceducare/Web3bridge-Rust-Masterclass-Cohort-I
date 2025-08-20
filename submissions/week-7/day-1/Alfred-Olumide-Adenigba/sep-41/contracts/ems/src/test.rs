#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{EmployeeManagementContract, EmployeeManagementContractClient, Employee};

fn setup() -> (Env, EmployeeManagementContractClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagementContract, ());
    let client = EmployeeManagementContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let employee1 = Address::generate(&env);
    let institution1 = Address::generate(&env);

    client.initialize(&admin);

    (env, client, admin, employee1, institution1)
}

#[test]
fn test_initialize() {
    let (env, client, admin, _employee1, _institution1) = setup();

    let count = client.get_employee_count();
    assert_eq!(count, 0);
}

#[test]
fn test_add_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);

    let employee = client.get_employee(&employee1).unwrap();
    assert_eq!(employee.id, employee1);
    assert_eq!(employee.institution, institution1);
    assert_eq!(employee.salary, 50000);
    assert_eq!(employee.rank, 1);
    assert_eq!(employee.is_active, true);
    assert_eq!(employee.is_suspended, false);

    let count = client.get_employee_count();
    assert_eq!(count, 1);
}

#[test]
fn test_remove_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    assert_eq!(client.get_employee_count(), 1);

    client.remove_employee(&employee1);
    assert!(client.get_employee(&employee1).is_none());
    assert_eq!(client.get_employee_count(), 0);
}

#[test]
fn test_update_employee_salary() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    
    client.update_employee_salary(&employee1, &60000);

    let employee = client.get_employee(&employee1).unwrap();
    assert_eq!(employee.salary, 60000);
}

#[test]
fn test_promote_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    
    client.promote_employee(&employee1, &2);

    let employee = client.get_employee(&employee1).unwrap();
    assert_eq!(employee.rank, 2);
}

#[test]
fn test_suspend_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    
    client.suspend_employee(&employee1);

    let employee = client.get_employee(&employee1).unwrap();
    assert_eq!(employee.is_suspended, true);
}

#[test]
fn test_reinstate_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    client.suspend_employee(&employee1);
    
    client.reinstate_employee(&employee1);

    let employee = client.get_employee(&employee1).unwrap();
    assert_eq!(employee.is_suspended, false);
}

#[test]
fn test_multiple_employees() {
    let (env, client, admin, employee1, institution1) = setup();
    let employee2 = Address::generate(&env);
    let institution2 = Address::generate(&env);

    client.add_employee(&employee1, &institution1, &50000, &1);
    client.add_employee(&employee2, &institution2, &75000, &2);

    assert_eq!(client.get_employee_count(), 2);

    let emp1 = client.get_employee(&employee1).unwrap();
    let emp2 = client.get_employee(&employee2).unwrap();

    assert_eq!(emp1.salary, 50000);
    assert_eq!(emp1.rank, 1);
    assert_eq!(emp2.salary, 75000);
    assert_eq!(emp2.rank, 2);
}

#[test]
#[should_panic(expected = "Employee already exists")]
fn test_add_duplicate_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    client.add_employee(&employee1, &institution1, &60000, &2);
}

#[test]
#[should_panic(expected = "Employee does not exist")]
fn test_remove_nonexistent_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.remove_employee(&employee1);
}

#[test]
#[should_panic(expected = "Salary must be positive")]
fn test_add_employee_negative_salary() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &-1000, &1);
}

#[test]
#[should_panic(expected = "New rank must be higher than current rank")]
fn test_promote_employee_lower_rank() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &3);
    client.promote_employee(&employee1, &2);
}

#[test]
#[should_panic(expected = "Employee is already suspended")]
fn test_suspend_already_suspended_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    client.suspend_employee(&employee1);
    client.suspend_employee(&employee1);
}

#[test]
#[should_panic(expected = "Employee is not suspended")]
fn test_reinstate_active_employee() {
    let (env, client, admin, employee1, institution1) = setup();

    client.add_employee(&employee1, &institution1, &50000, &1);
    client.reinstate_employee(&employee1);
}

#[test]
fn test_set_token_contract() {
    let (env, client, admin, _employee1, _institution1) = setup();
    
    let token_contract = Address::generate(&env);
    client.set_token_contract(&token_contract);
}

#[test]
fn test_pay_salary() {
    let (env, client, admin, employee1, institution1) = setup();
    
    // Setup token contract
    let token_contract_id = env.register(sep_41_token::TokenContract, ());
    let token_client = sep_41_token::TokenContractClient::new(&env, &token_contract_id);
    
    // Initialize token with total supply
    let total_supply = 1000000_i128;
    token_client.initialize(&admin, &total_supply);
    
    // Set token contract in EMS
    client.set_token_contract(&token_contract_id);
    
    // Add employee
    let salary = 5000_i128;
    client.add_employee(&employee1, &institution1, &salary, &1);
    
    // Transfer tokens to admin for salary payment
    let payer = Address::generate(&env);
    token_client.transfer(&admin, &payer, &salary);
    
    // Pay salary
    client.pay_salary(&payer, &employee1);
    
    // Verify employee received tokens
    let employee_balance = token_client.balance_of(&employee1);
    assert_eq!(employee_balance, salary);
}

#[test]
#[should_panic(expected = "Employee does not exist")]
fn test_pay_salary_nonexistent_employee() {
    let (env, client, admin, _employee1, _institution1) = setup();
    
    let token_contract = Address::generate(&env);
    client.set_token_contract(&token_contract);
    
    let fake_employee = Address::generate(&env);
    let payer = Address::generate(&env);
    client.pay_salary(&payer, &fake_employee);
}

#[test]
#[should_panic(expected = "Employee is suspended")]
fn test_pay_salary_suspended_employee() {
    let (env, client, admin, employee1, institution1) = setup();
    
    let token_contract = Address::generate(&env);
    client.set_token_contract(&token_contract);
    
    client.add_employee(&employee1, &institution1, &50000, &1);
    client.suspend_employee(&employee1);
    
    let payer = Address::generate(&env);
    client.pay_salary(&payer, &employee1);
}
