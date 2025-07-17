pub mod state;

use crate::state::{Employee, EmployeeType};


impl Employee {
    pub fn new(employee_type: EmployeeType, is_employed: bool) -> Self {
        Employee {
            employee_type,
            is_employed,
        }
    }

    pub fn check_access(&self) -> Result<(), String> {
        if !self.is_employed {
            return Err("Access denied: Employee is terminated.".to_string());
        }

        match self.employee_type {
            EmployeeType::MediaTeam | EmployeeType::IT | EmployeeType::Manager => Ok(()),
            _ => Err("Access denied: Employee does not have clearance.".to_string()),
        }
    }

    pub fn print_access_status(&self) -> Result<(), String> {
        self.check_access()?;
        println!("Access granted for {:?}.", self.employee_type);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_access_granted() {
        let manager = Employee::new(EmployeeType::Manager, true);
        assert!(manager.check_access().is_ok());
    }

    #[test]
    fn test_it_employee_access_granted() {
        let it_employee = Employee::new(EmployeeType::IT, true);
        assert!(it_employee.check_access().is_ok());
    }

    #[test]
    fn test_kitchen_staff_access_denied() {
        let kitchen_staff = Employee::new(EmployeeType::KitchenStaff, true);
        let result = kitchen_staff.check_access();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Access denied: Employee does not have clearance."
        );
    }

    #[test]
    fn test_terminated_manager_access_denied() {
        let terminated_manager = Employee::new(EmployeeType::Manager, false);
        let result = terminated_manager.check_access();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Access denied: Employee is terminated."
        );
    }

    #[test]
    fn test_social_media_access_denied() {
        let social_media_employee = Employee::new(EmployeeType::SocialMedia, true);
        assert!(social_media_employee.check_access().is_err());
    }
}
