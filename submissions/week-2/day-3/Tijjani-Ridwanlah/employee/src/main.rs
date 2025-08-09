#[derive(Debug)]
pub enum EmployeeType {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
pub enum EmployeeStatus {
    Active,
    Terminated,
}

#[derive(Debug)]
pub struct Employee {
    role: EmployeeType,
    status: EmployeeStatus,
}

pub fn check_access(employee: &Employee) -> Result<(), String> {
    if let EmployeeStatus::Terminated = employee.status {
        return Err("Access denied: Employee is terminated".to_string());
    }

    match employee.role {
        EmployeeType::MediaTeam | EmployeeType::ITDepartment | EmployeeType::Manager => Ok(()),
        _ => Err("Access denied: Employee type not authorized".to_string()),
    }
}

pub fn print_access_status(employee: &Employee) -> Result<(), String> {
    check_access(&employee)?;
    println!("Access granted for {:?}", employee.role);
    Ok(())
}

fn main() {
    let employees = vec![
        Employee {
            role: EmployeeType::MediaTeam,
            status: EmployeeStatus::Active,
        },
        Employee {
            role: EmployeeType::TechnicianSupervisor,
            status: EmployeeStatus::Active,
        },
        Employee {
            role: EmployeeType::Manager,
            status: EmployeeStatus::Terminated,
        },
        Employee {
            role: EmployeeType::KitchenStaff,
            status: EmployeeStatus::Active,
        },
        Employee {
            role: EmployeeType::ITDepartment,
            status: EmployeeStatus::Active,
        },
    ];

    for employee in employees {
        match print_access_status(&employee) {
            Ok(msg) => println!("{:?}", msg),
            Err(err) => println!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_access() {
        let active_manager = Employee {
            role: EmployeeType::Manager,
            status: EmployeeStatus::Active,
        };
        assert!(check_access(&active_manager).is_ok());

        let terminated_it = Employee {
            role: EmployeeType::ITDepartment,
            status: EmployeeStatus::Terminated,
        };
        assert!(check_access(&terminated_it).is_err());

        let active_kitchen = Employee {
            role: EmployeeType::KitchenStaff,
            status: EmployeeStatus::Active,
        };
        assert!(check_access(&active_kitchen).is_err());
    }
}
