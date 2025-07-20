use crate::employee_struct::{Employee, EmployeeDB};
use crate::employee_enum::{EmployeeType, EmploymentStatus, AccessError};

impl EmployeeDB {
   pub fn new() -> Self {
        Self{ 
            data: Vec::new(),
            next_id: 1,
         }
    }

   pub fn new_employee(&mut self, employee: Employee) -> u32 {
        let present_id = self.next_id;

        let new_employee = Employee {
            id: present_id,
            name: employee.name,
            age: employee.age,
            dept: employee.dept,
            status: EmploymentStatus::Employed,
        };

        self.next_id += 1;
        self.data.push(new_employee);
        present_id

    }

   pub fn fire_employee(&mut self, id:u32, new_status: EmploymentStatus) -> bool {
        if let Some(employee) = self.data.iter_mut().find(|employee_id| employee_id.id == id) {
            employee.status = new_status;
            true
        } else {
            false
        }

    }

   pub fn can_access_building(&self, id: u32) -> Result<(), AccessError> {
        let employee = self.data.iter().find(|e| e.id == id)
            .ok_or(AccessError::NotAuthorized)?;

        if employee.status != EmploymentStatus::Employed {
            return Err(AccessError::NotEmployed);
        }

        match employee.dept {
            EmployeeType::MultiMedia | EmployeeType::Ict | EmployeeType::Manager => Ok(()),
            _ => Err(AccessError::NotAuthorized),
        }
    }

  pub fn enter_building(&self, id: u32) -> Result<(), AccessError> {
        self.can_access_building(id)?; // ? will return early if Err
        println!("Access granted ✅");
        Ok(())
    }
}