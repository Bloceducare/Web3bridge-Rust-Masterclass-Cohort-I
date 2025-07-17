
// Employee roles
#[derive(Debug)]
enum EmployeeType {
    MediaTeam,
    IT,
    Manager,
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}

// Employee data
#[derive(Debug)]
struct Employee {
    role: EmployeeType,
    still_employed: bool,
}

// Access errors
#[derive(Debug, PartialEq)]
enum AccessError {
    Terminated,
    RoleNotAllowed,
}

// Check access logic
fn can_enter_building(employee: &Employee) -> Result<(), AccessError> {
    if !employee.still_employed {
        return Err(AccessError::Terminated);
    }

    match employee.role {
        EmployeeType::MediaTeam |
        EmployeeType::IT |
        EmployeeType::Manager => Ok(()),
        _ => Err(AccessError::RoleNotAllowed),
    }
}

// Uses `?` operator to call access function
fn print_access(employee: &Employee) -> Result<(), AccessError> {
    can_enter_building(employee)?;
    println!("Access granted ");
    Ok(())
}


fn main() {
    let employee = Employee {
        role: EmployeeType::IT,
        still_employed: true,
    };

    let _ = print_access(&employee);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_allowed_employee() {
        let emp = Employee {
            role: EmployeeType::IT,
            still_employed: true,
        };
        assert_eq!(can_enter_building(&emp), Ok(()));
    }

    #[test]
    fn test_access_disallowed_employee() {
        let emp = Employee {
            role: EmployeeType::KitchenStaff,
            still_employed: true,
        };
        assert_eq!(can_enter_building(&emp), Err(AccessError::RoleNotAllowed));
    }

    #[test]
    fn test_access_terminated_employee() {
        let emp = Employee {
            role: EmployeeType::Manager,
            still_employed: false,
        };
        assert_eq!(can_enter_building(&emp), Err(AccessError::Terminated));
    }

    #[test]
    fn test_print_access_allowed() {
        let emp = Employee {
            role: EmployeeType::MediaTeam,
            still_employed: true,
        };
        let result = print_access(&emp);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_print_access_terminated() {
        let emp = Employee {
            role: EmployeeType::MediaTeam,
            still_employed: false,
        };
        let result = print_access(&emp);
        assert_eq!(result, Err(AccessError::Terminated));
    }
}