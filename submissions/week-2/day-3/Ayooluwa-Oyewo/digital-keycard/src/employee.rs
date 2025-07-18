use crate::error::EmployeeError;

#[derive(Debug, PartialEq)]
pub enum EmployeeType {
    Media,
    IT,
    Manager,
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
pub struct EmployeeDetails {
    pub id: u32,
    pub name: String,
    pub employee_type: EmployeeType,
    pub is_employed: bool,
}

#[derive(Debug)]
pub struct Employee {
    pub data: Vec<EmployeeDetails>,
    pub next_id: u32,
}

impl Employee {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_employee(&mut self, name: String, employee_type: EmployeeType, is_employed: bool) {
        let employee = EmployeeDetails {
            id: self.next_id,
            name,
            employee_type,
            is_employed,
        };
        self.data.push(employee);
        self.next_id += 1;
    }

    pub fn check_access(&self, employee_id: u32) -> Result<String, EmployeeError> {
        let employee = self
            .data
            .iter()
            .find(|e| e.id == employee_id)
            .ok_or(EmployeeError::EmployeeNotFound)?;

        if !employee.is_employed {
            return Err(EmployeeError::EmployeeTerminated);
        }

        let message = match employee.employee_type {
            EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager => Ok(format!(
                "Employee {} can access the building",
                employee.name
            )),

            EmployeeType::SocialMedia
            | EmployeeType::TechnicianSupervisor
            | EmployeeType::KitchenStaff => Err(EmployeeError::EmployeeNotAllowed),
        };
        message
    }

    pub fn can_access_building(&self, employee_id: u32) {
        match self.check_access(employee_id) {
            Ok(message) => println!("{}", message),
            Err(error) => println!(
                "{}",
                match error {
                    EmployeeError::EmployeeNotAllowed => {
                        format!("Employee {} cannot access the building", employee_id)
                    }
                    EmployeeError::EmployeeTerminated => {
                        format!("Employee {} has been terminated", employee_id)
                    }
                    EmployeeError::EmployeeNotFound => {
                        format!("Employee {} cannot be found", employee_id)
                    }
                }
            ),
        }
    }
}
