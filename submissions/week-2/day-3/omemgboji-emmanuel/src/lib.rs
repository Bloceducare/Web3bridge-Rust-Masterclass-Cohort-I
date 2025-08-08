pub mod enums;
pub mod structs;

use enums::EmployeeRoles;
use structs::Employee;

impl Employee {
    pub fn check_access(&self) -> Result<(), String> {
        if !self.is_employed {
            return Err("Access denied: You are not employed.".to_string());
        }

        match self.role {
            EmployeeRoles::MediaTeam | EmployeeRoles::ITDepartment | EmployeeRoles::Manager => {
                Ok(())
            }
            _ => Err("Access denied: You do not have the required role.".to_string()),
        }
    }

    pub fn access_garage(&self) -> Result<(), String> {
        self.check_access()?;
        println!("Welcome back! Access granted to the Web3Bridge Garage.");
        Ok(())
    }
}
