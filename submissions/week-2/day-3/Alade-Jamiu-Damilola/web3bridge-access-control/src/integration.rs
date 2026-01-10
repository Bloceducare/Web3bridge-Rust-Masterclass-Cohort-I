use web3bridge_garage::*;

#[test]
fn it_denies_unemployed() {
    let e = Employee::new(EmployeeType::IT, false);
    assert!(matches!(check_access(&e), Err(AccessError::NotEmployed)));
}

#[test]
fn it_grants_it_access() {
    let e = Employee::new(EmployeeType::IT, true);
    assert!(check_access(&e).is_ok());
}