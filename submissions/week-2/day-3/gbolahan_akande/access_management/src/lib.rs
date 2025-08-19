pub mod state;

use crate::state::*;

pub fn is_employee_access(employ: &EmployeeDetails) -> Result<(), String> {
    if employ.is_employed == EmploymentStatus::Employed {
        match employ.employee_type {
            EmployeeType::ITdept => Ok(()),
            EmployeeType::Managers => Ok(()),
            EmployeeType::MediaTeam => Ok(()),
            EmployeeType::KitchenStaff => Err("You have no access".to_string()),
            EmployeeType::SocialMedia => Err("You have no access".to_string()),
            EmployeeType::Supervisors => Err("You have no access".to_string()),
        }
    } else {
        Err("You are not Employed".to_string())
    }
}

pub fn print_access(employ: &EmployeeDetails) -> Result<(), String> {
    match is_employee_access(employ) {
        Ok(pass) => {
            println!("You have access");
            Ok(())
        }
        Err(Fail) => {
            println!("You have no access");
            Ok(())
        }
    }
}
