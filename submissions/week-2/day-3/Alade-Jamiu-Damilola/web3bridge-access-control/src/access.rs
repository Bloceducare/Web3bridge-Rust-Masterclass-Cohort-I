use crate::employee::{Employee, EmployeeType};

#[derive(Debug)]
pub enum AccessError {
    NotEmployed,
    NotAuthorized,
}

pub fn check_access(employee: &Employee) -> Result<(), AccessError> {
    if !employee.is_employed {
        return Err(AccessError::NotEmployed);
    }
    match employee.employee_type {
        EmployeeType::MediaTeam | EmployeeType::IT | EmployeeType::Manager => Ok(()),
        _ => Err(AccessError::NotAuthorized),
    }
}

pub fn print_access(employee: &Employee) -> Result<(), AccessError> {
    check_access(employee)?;
    println!("Access granted to web3bridge garage.");
    Ok(())
}