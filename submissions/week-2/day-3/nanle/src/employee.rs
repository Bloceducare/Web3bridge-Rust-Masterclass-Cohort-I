#[derive(Debug)]
pub enum EmployeeType {
    // Employees that can access the building
    MediaTeam,
    ITDepartment,
    Manager,
    // Employees that cannot access the building
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
pub struct Employee {
    employee_type: EmployeeType,
    is_employed: bool,
}

impl Employee {
    pub fn new(employee_type: EmployeeType, is_employed: bool) -> Self {
        Employee {
            employee_type,
            is_employed,
        }
    }

    pub fn can_access_building(&self) -> Result<(), String> {
        // Check if the employee is employed
        if !self.is_employed {
            return Err("Access denied: Employee is terminated.".to_string());
        }

        // Check if the employee type has access
        match self.employee_type {
            EmployeeType::MediaTeam
            | EmployeeType::ITDepartment
            | EmployeeType::Manager => Ok(()),
            EmployeeType::SocialMediaTeam
            | EmployeeType::TechnicianSupervisor
            | EmployeeType::KitchenStaff => Err("Access denied: Role does not have access.".to_string()),
        }
    }

    pub fn print_access_status(&self) {
        match self.can_access_building() {
            Ok(_) => println!("Access granted: Employee can access the building."),
            Err(err) => println!("{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_team_access() {
        let employee = Employee::new(EmployeeType::MediaTeam, true);
        assert_eq!(employee.can_access_building(), Ok(()));
    }

    #[test]
    fn test_it_department_access() {
        let employee = Employee::new(EmployeeType::ITDepartment, true);
        assert_eq!(employee.can_access_building(), Ok(()));
    }

    #[test]
    fn test_manager_access() {
        let employee = Employee::new(EmployeeType::Manager, true);
        assert_eq!(employee.can_access_building(), Ok(()));
    }

    #[test]
    fn test_social_media_team_denied() {
        let employee = Employee::new(EmployeeType::SocialMediaTeam, true);
        assert_eq!(
            employee.can_access_building(),
            Err("Access denied: Role does not have access.".to_string())
        );
    }

    #[test]
    fn test_terminated_employee_denied() {
        let employee = Employee::new(EmployeeType::Manager, false);
        assert_eq!(
            employee.can_access_building(),
            Err("Access denied: Employee is terminated.".to_string())
        );
    }
}