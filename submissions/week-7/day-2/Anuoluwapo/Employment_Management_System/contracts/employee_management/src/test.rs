#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::{
    EmployeeManagement, 
    EmployeeManagementClient, 
    EmployeeRank, 
    EmployeeStatus
};

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);

    let result = client.try_initialize(&admin, &token_contract);
    assert!(result.is_ok());

    let stored_admin = client.get_admin();
    assert_eq!(stored_admin, admin);
}

#[test]
fn test_register_institution() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);

    client.initialize(&admin, &token_contract);

    let result = client.try_register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );
    assert!(result.is_ok());

    let institution = client.view_institution(&institution_addr);
    assert_eq!(institution.name, String::from_str(&env, "Tech Corp"));
    assert_eq!(institution.admin, inst_admin);
}

#[test]
fn test_add_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);
    let employee_addr = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );

    let employee_id = client.add_employee(
        &employee_addr,
        &institution_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000i128
    );
    assert!(employee_id > 0);

    let employee = client.view_employee(&employee_addr);
    assert_eq!(employee.name, String::from_str(&env, "John Doe"));
    assert_eq!(employee.rank, EmployeeRank::Junior);
    assert_eq!(employee.salary, 50000i128);
}

#[test]
fn test_promote_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);
    let employee_addr = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );
    client.add_employee(
        &employee_addr,
        &institution_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &40000i128
    );

    let result = client.try_promote_employee(
        &employee_addr,
        &EmployeeRank::Mid,
        &55000i128
    );
    assert!(result.is_ok());

    let employee = client.view_employee(&employee_addr);
    assert_eq!(employee.rank, EmployeeRank::Mid);
    assert_eq!(employee.salary, 55000i128);
}

#[test]
fn test_suspend_and_reactivate_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);
    let employee_addr = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );
    client.add_employee(
        &employee_addr,
        &institution_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &40000i128
    );

    let result = client.try_suspend_employee(&employee_addr);
    assert!(result.is_ok());

    let employee = client.view_employee(&employee_addr);
    assert_eq!(employee.status, EmployeeStatus::Suspended);

    let result = client.try_reactivate_employee(&employee_addr);
    assert!(result.is_ok());

    let employee = client.view_employee(&employee_addr);
    assert_eq!(employee.status, EmployeeStatus::Active);
}

#[test]
fn test_update_employee() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);
    let employee_addr = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );
    client.add_employee(
        &employee_addr,
        &institution_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &40000i128
    );

    let result = client.try_update_employee(
        &employee_addr,
        &Some(String::from_str(&env, "John Smith")),
        &Some(45000i128)
    );
    assert!(result.is_ok());

    let employee = client.view_employee(&employee_addr);
    assert_eq!(employee.name, String::from_str(&env, "John Smith"));
    assert_eq!(employee.salary, 45000i128);
}

#[test]
fn test_employee_count() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );

    assert_eq!(client.get_employee_count(), 0);

    let employee1 = Address::generate(&env);
    let employee2 = Address::generate(&env);

    client.add_employee(
        &employee1,
        &institution_addr,
        &String::from_str(&env, "Employee 1"),
        &EmployeeRank::Junior,
        &40000i128
    );

    client.add_employee(
        &employee2,
        &institution_addr,
        &String::from_str(&env, "Employee 2"),
        &EmployeeRank::Mid,
        &50000i128
    );

    assert_eq!(client.get_employee_count(), 2);
}

#[test]
fn test_unauthorized_access() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);
    let _unauthorized_user = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );


}

#[test]
fn test_invalid_salary() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(EmployeeManagement, ());
    let client = EmployeeManagementClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let token_contract = Address::generate(&env);
    let institution_addr = Address::generate(&env);
    let inst_admin = Address::generate(&env);
    let employee_addr = Address::generate(&env);

    client.initialize(&admin, &token_contract);
    client.register_institution(
        &institution_addr, 
        &String::from_str(&env, "Tech Corp"), 
        &inst_admin
    );

 
    let employee_id = client.add_employee(
        &employee_addr,
        &institution_addr,
        &String::from_str(&env, "John Doe"),
        &EmployeeRank::Junior,
        &50000i128  
    );
    
    assert!(employee_id > 0);
    
    
}