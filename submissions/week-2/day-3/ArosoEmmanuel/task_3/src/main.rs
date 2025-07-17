#[derive(Debug)]
enum EmployeeType {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
struct Employee {
    emp_type: EmployeeType,
    is_employed: bool,
}

// Error type to represent access denial reasons
#[derive(Debug, PartialEq)]
enum AccessError {
    Terminated,
    NotAuthorized,
}

// Function that returns a Result indicating if access is allowed
fn can_access(employee: &Employee) -> Result<(), AccessError> {
    if !employee.is_employed {
        return Err(AccessError::Terminated);
    }

    match employee.emp_type {
        EmployeeType::MediaTeam | EmployeeType::ITDepartment | EmployeeType::Manager => Ok(()),
        _ => Err(AccessError::NotAuthorized),
    }
}

// Function using the `?` operator to print access status
fn print_access_status(employee: &Employee) {
    match try_access(employee) {
        Ok(_) => println!("Access granted for {:?}", employee.emp_type),
        Err(e) => println!("Access denied for {:?}: {:?}", employee.emp_type, e),
    }
}

// Helper function that uses `?` operator internally
fn try_access(employee: &Employee) -> Result<(), AccessError> {
    can_access(employee)?; // uses ? operator to propagate error
    Ok(())
}

fn main() {
    let employees = vec![
        Employee {
            emp_type: EmployeeType::MediaTeam,
            is_employed: true,
        },
        Employee {
            emp_type: EmployeeType::SocialMediaTeam,
            is_employed: true,
        },
        Employee {
            emp_type: EmployeeType::Manager,
            is_employed: false,
        },
        Employee {
            emp_type: EmployeeType::KitchenStaff,
            is_employed: true,
        },
        Employee {
            emp_type: EmployeeType::ITDepartment,
            is_employed: true,
        },
    ];

    for emp in employees {
        print_access_status(&emp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorized_and_employed() {
        let emp = Employee {
            emp_type: EmployeeType::ITDepartment,
            is_employed: true,
        };
        assert_eq!(can_access(&emp), Ok(()));
    }

    #[test]
    fn test_authorized_but_terminated() {
        let emp = Employee {
            emp_type: EmployeeType::Manager,
            is_employed: false,
        };
        assert_eq!(can_access(&emp), Err(AccessError::Terminated));
    }

    #[test]
    fn test_unauthorized_employee() {
        let emp = Employee {
            emp_type: EmployeeType::KitchenStaff,
            is_employed: true,
        };
        assert_eq!(can_access(&emp), Err(AccessError::NotAuthorized));
    }

    #[test]
    fn test_unauthorized_and_terminated() {
        let emp = Employee {
            emp_type: EmployeeType::TechnicianSupervisor,
            is_employed: false,
        };
        assert_eq!(can_access(&emp), Err(AccessError::Terminated));
    }
}
