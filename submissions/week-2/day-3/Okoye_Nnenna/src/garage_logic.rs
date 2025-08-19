use crate::employees::{Employee, EmployeeType};

pub fn can_access(employee: &Employee) -> Result<(), String> {
    if !employee.is_employed {
        return Err("Access denied: Employee is terminated.".into());
    }

    match employee.role {
        EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager => Ok(()),
        _ => Err("Access denied: Not authorized for garage access.".into()),
    }
}

pub fn check_access(employee: &Employee) -> Result<(), String> {
    can_access(employee)?;
    println!("Access granted: Welcome to the garage.");
    Ok(())
}
