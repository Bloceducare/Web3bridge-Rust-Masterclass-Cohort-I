#[derive(Debug, Clone)]
pub enum EmployeeType {
    MediaTeam,
    IT,
    Manager,
    SocialMediaTeam,
    TechnicalSupervisor,
    KitchenStaff
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub employee_type: EmployeeType,
    pub is_employed: bool
}

impl Employee {
    pub fn new (employee_type : EmployeeType, is_employed: bool) -> Self {
        Employee {
            employee_type,
            is_employed,
        }
    }
}

#[derive(Debug)]
pub enum AccessError {
    NotEmployed,
    NotAuthorized,
}

pub fn check_access(employee: &Employee) -> Result<(), AccessError> {
    if !employee.is_employed {
        return Err(AccessError::NotEmployed)
    }

    match employee.employee_type {
        EmployeeType::MediaTeam | EmployeeType::IT | EmployeeType::Manager => Ok(()),
        _ => Err(AccessError::NotAuthorized)
    }
}

pub fn print_access(employee: &Employee) -> Result<(), AccessError> {
    check_access(employee)?;
    println!("Access granted to web3bridge garage.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allowed_active() {
        let employee = Employee::new(EmployeeType::IT, true);
        assert!(check_access(&employee).is_ok());
    }

    #[test]
    fn test_allowed_terminated() {
        let employee = Employee::new(EmployeeType::Manager, false);
        assert!(matches!(check_access(&employee), Err(AccessError::NotEmployed)));
    }

    #[test]
    fn test_not_allowed_active() {
        let employee = Employee::new(EmployeeType::KitchenStaff, true);
        assert!(matches!(check_access(&employee), Err(AccessError::NotAuthorized)));
    }

    #[test]
    fn test_not_allowed_terminated() {
        let employee = Employee::new(EmployeeType::SocialMediaTeam, false);
        assert!(matches!(check_access(&employee), Err(AccessError::NotEmployed)));
    }
}