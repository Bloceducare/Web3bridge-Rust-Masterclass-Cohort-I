fn main() {
    let employees = vec![
        Employee { role: EmployeeRole::MediaTeam, status: EmploymentStatus::Active },
        Employee { role: EmployeeRole::TechnicianSupervisor, status: EmploymentStatus::Active },
        Employee { role: EmployeeRole::Manager, status: EmploymentStatus::Terminated },
        Employee { role: EmployeeRole::KitchenStaff, status: EmploymentStatus::Active },
        Employee { role: EmployeeRole::ITDepartment, status: EmploymentStatus::Active },
    ];

    for employee in employees {
        match employee.check_access() {
            Ok(msg) => println!("{}", msg),
            Err(err) => println!("{}", err),
        }
    }
}

#[derive(Debug)]
enum EmployeeRole {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
enum EmploymentStatus {
    Active,
    Terminated,
}

#[derive(Debug)]
struct Employee {
    role: EmployeeRole,
    status: EmploymentStatus,
}

impl Employee {
    fn can_access(&self) -> Result<(), String> {
        match self.status {
            EmploymentStatus::Terminated => Err("Access denied: Employee is terminated".to_string()),
            EmploymentStatus::Active => match self.role {
                EmployeeRole::MediaTeam | EmployeeRole::ITDepartment | EmployeeRole::Manager => Ok(()),
                _ => Err("Access denied: Role not permitted".to_string()),
            },
        }
    }

    fn check_access(&self) -> Result<String, String> {
        self.can_access()?;
        Ok(format!("Access granted for {:?}", self.role))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access() {
        let active_manager = Employee {
            role: EmployeeRole::Manager,
            status: EmploymentStatus::Active,
        };
        assert!(active_manager.can_access().is_ok());

        let terminated_it = Employee {
            role: EmployeeRole::ITDepartment,
            status: EmploymentStatus::Terminated,
        };
        assert!(terminated_it.can_access().is_err());

        let active_kitchen = Employee {
            role: EmployeeRole::KitchenStaff,
            status: EmploymentStatus::Active,
        };
        assert!(active_kitchen.can_access().is_err());
    }
}
