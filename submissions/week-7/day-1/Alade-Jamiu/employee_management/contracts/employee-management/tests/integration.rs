use super::EmployeeManagementContractClient;
use soroban_sdk::{testutils::Address, Env, IntoVal};

#[test]
fn test_add_employee() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract_wasm(None, EmployeeManagementContractClient);
    let client = EmployeeManagementContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    let result = client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("John Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1000,
        &env.string_new_from_slice("Developer"),
    );
    assert!(result.is_ok());

    let employee = client.get_employee(&env.string_new_from_slice("EMP001")).unwrap();
    assert_eq!(employee.name, env.string_new_from_slice("John Doe"));
    assert_eq!(client.get_employee_count(), 1);
}

#[test]
fn test_add_duplicate_employee() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract_wasm(None, EmployeeManagementContractClient);
    let client = EmployeeManagementContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("John Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1000,
        &env.string_new_from_slice("Developer"),
    ).unwrap();

    let result = client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("Jane Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1200,
        &env.string_new_from_slice("Senior Developer"),
    );
    assert!(result.is_err());
}

#[test]
fn test_remove_employee() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract_wasm(None, EmployeeManagementContractClient);
    let client = EmployeeManagementContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("John Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1000,
        &env.string_new_from_slice("Developer"),
    ).unwrap();

    let result = client.remove_employee(&env.string_new_from_slice("EMP001"));
    assert!(result.is_ok());
    assert_eq!(client.get_employee_count(), 0);
    assert!(client.get_employee(&env.string_new_from_slice("EMP001")).is_err());
}

#[test]
fn test_update_employee() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract_wasm(None, EmployeeManagementContractClient);
    let client = EmployeeManagementContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("John Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1000,
        &env.string_new_from_slice("Developer"),
    ).unwrap();

    let result = client.update_employee(
        &env.string_new_from_slice("EMP001"),
        &Some(env.string_new_from_slice("John Smith")),
        &Some(1500),
        &None,
    );
    assert!(result.is_ok());

    let employee = client.get_employee(&env.string_new_from_slice("EMP001")).unwrap();
    assert_eq!(employee.name, env.string_new_from_slice("John Smith"));
    assert_eq!(employee.salary, 1500);
}

#[test]
fn test_promote_employee() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract_wasm(None, EmployeeManagementContractClient);
    let client = EmployeeManagementContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("John Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1000,
        &env.string_new_from_slice("Developer"),
    ).unwrap();

    let result = client.promote_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("Senior Developer"),
    );
    assert!(result.is_ok());

    let employee = client.get_employee(&env.string_new_from_slice("EMP001")).unwrap();
    assert_eq!(employee.rank, env.string_new_from_slice("Senior Developer"));
}

#[test]
fn test_suspend_employee() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register_contract_wasm(None, EmployeeManagementContractClient);
    let client = EmployeeManagementContractClient::new(&env, &contract_id);
    client.initialize(&admin);

    client.add_employee(
        &env.string_new_from_slice("EMP001"),
        &env.string_new_from_slice("John Doe"),
        &env.string_new_from_slice("Tech Corp"),
        &1000,
        &env.string_new_from_slice("Developer"),
    ).unwrap();

    let result = client.suspend_employee(&env.string_new_from_slice("EMP001"));
    assert!(result.is_ok());

    let employee = client.get_employee(&env.string_new_from_slice("EMP001")).unwrap();
    assert_eq!(employee.status, env.string_new_from_slice("Suspended"));
}