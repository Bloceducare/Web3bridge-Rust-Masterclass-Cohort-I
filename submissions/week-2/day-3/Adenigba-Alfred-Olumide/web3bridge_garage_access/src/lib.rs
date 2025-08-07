
#[derive(Debug, Clone, PartialEq)]
pub enum EmployeeStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmployeeType {
    MediaTeam,
    ITTeam,
    Manager,
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub employee_type: EmployeeType,
    pub employee_status: EmployeeStatus,
}

pub struct Garage {
    pub employees: Vec<Employee>,
    pub next_id: u32,
}

impl Garage {
    pub fn new() -> Garage {
        Garage { employees: Vec::new(), next_id: 1 }
    }

    pub fn create_employee(&mut self, name: String, employee_type: EmployeeType) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.employees.push(Employee { id, name, employee_type, employee_status: EmployeeStatus::Active });
        id
    }

    pub fn get_employee(&mut self, id: u32) -> Result<&mut Employee, String> {
        self.employees.iter_mut().find(|employee| employee.id == id).ok_or("Employee not found".to_string())
    }

    pub fn get_employees(&self) -> Vec<Employee> {
        self.employees.to_vec()
    }

    pub fn change_employee_status(&mut self, id:u32, employee_status: Option<EmployeeStatus>) -> Result<(), String> {
        let employee: &mut Employee  = self.get_employee(id)?;
        if let Some(s) = employee_status {
            employee.employee_status = s;
            Ok(())
        } else {
            println!("Employee status not provided");
            Err("Employee status not provided".to_string())
        }
    }

    
    pub fn can_access_garage(&mut self, id: u32) -> Result<(), String> {
        let employee:&mut Employee = self.get_employee(id)?;
        if employee.employee_status == EmployeeStatus::Inactive {
            return Err("Employee is inactive".to_string());
        }
        match employee.employee_type {
            EmployeeType::MediaTeam => Ok(()),
            EmployeeType::ITTeam => Ok(()),
            EmployeeType::Manager => Ok(()),
            EmployeeType::SocialMediaTeam => Err("Cannot access the garage".to_string()),
            EmployeeType::TechnicianSupervisor => Err("Cannot access the garage".to_string()),
            EmployeeType::KitchenStaff => Err("Cannot access the garage".to_string()),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_employee() {
        let mut garage = Garage::new();
        garage.create_employee("Peace".to_string(), EmployeeType::MediaTeam);
        assert_eq!(garage.employees.len(), 1);
    }

    #[test]
    fn test_get_employees() {
        let mut garage = Garage::new();
        garage.create_employee("Peace".to_string(), EmployeeType::MediaTeam);
        garage.create_employee("Ola".to_string(), EmployeeType::KitchenStaff);
        garage.create_employee("Timidan".to_string(), EmployeeType::Manager);
        assert_eq!(garage.get_employees().len(), 3);
    }

    #[test]
    fn test_change_employee_status() {
        let mut garage = Garage::new();
        garage.create_employee("Peace".to_string(), EmployeeType::MediaTeam);
        garage.change_employee_status(1, Some(EmployeeStatus::Inactive));
        assert_eq!(garage.get_employee(1).unwrap().employee_status, EmployeeStatus::Inactive);
    }

    #[test]
    fn test_can_access_garage() {
        let mut garage = Garage::new();
        garage.create_employee("Peace".to_string(), EmployeeType::MediaTeam);
        assert_eq!(garage.can_access_garage(1), Ok(()));
    }

}
