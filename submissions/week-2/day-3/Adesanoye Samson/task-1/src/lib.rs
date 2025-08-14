#[derive(Debug, Clone, PartialEq)]
pub enum EmployeeType {
    MEDIATEAM,
    ITDEPARTMENT,
    MANAGER,
    SOCIAMEDIATEAM,
    TECHNICALSUPERVISOR,
    KITCHENSTAFF
} 

#[derive(Debug, Clone, PartialEq)]
pub enum EmployeeStatus {
    ACTIVE,
    TERMINATED
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub id: u32,
    pub name: String, 
    pub role: EmployeeType,
    pub employee_status: EmployeeStatus
}

pub struct Garage {
    pub employees: Vec<Employee>,
    pub next_id: u32,
}

impl Garage {
    pub fn new() -> Garage {
        Garage {
            employees: Vec::new(),
            next_id: 1
        }
    }

    pub fn create_employee(&mut self, name: String, role: EmployeeType) -> u32 {
        let id = self.next_id;
        self.employees.push(Employee {
            id,
            name,
            role,
            employee_status: EmployeeStatus::ACTIVE
        });
        self.next_id += 1;
        id
    }

    pub fn get_employee(&mut self, id: u32) -> Result<&mut Employee, String> {
        self.employees.iter_mut().find(|employee| employee.id == id).ok_or("Employee not found".to_string())
    }

    pub fn get_employees(&self) ->Vec<Employee> {
        self.employees.to_vec()
    }

    pub fn change_employee_status(&mut self, id: u32, employee_status: Option<EmployeeStatus>) -> Result<(), String> {
        let employee: &mut Employee = self.get_employee(id)?;
        if let Some(s) = employee_status {
            employee.employee_status = s;
            Ok(())
        } else {
            println!("Employee status not provided");
            Err("Employee status not provided".to_string())
        }
    }

    pub fn can_access_garage(&mut self, id: u32) -> Result<(), String> {
        let employee: &mut Employee = self.get_employee(id)?;
        if employee.employee_status == EmployeeStatus::TERMINATED {
            return Err("Employee is inactive".to_string());
        }

        match employee.role {
            EmployeeType::MEDIATEAM | EmployeeType::ITDEPARTMENT | EmployeeType::MANAGER => Ok(()),
            _ => Err("Cannot access the garage".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_employee() {
        let mut garage = Garage::new();
        garage.create_employee(
            "peace".to_string(),
            EmployeeType::MEDIATEAM
        );
        assert_eq!(garage.employees.len(), 1);
        assert_eq!(garage.get_employees().len(), 1);
    }

     #[test]
    fn test_change_employee_status() {
        let mut garage = Garage::new();
        garage.create_employee("Peace".to_string(), EmployeeType::MEDIATEAM);
        garage.change_employee_status(1, Some(EmployeeStatus::TERMINATED));
        assert_eq!(garage.get_employee(1).unwrap().employee_status, EmployeeStatus::TERMINATED);
    }

    #[test]
    fn test_can_access_garage() {
        let mut garage = Garage::new();
        garage.create_employee("Peace".to_string(), EmployeeType::MEDIATEAM);
        assert_eq!(garage.can_access_garage(1), Ok(()));
    }

    #[test]
    fn test_can_not_access_garage() {
        let mut garage = Garage::new();
        garage.create_employee("John".to_string(), EmployeeType::KITCHENSTAFF);
        assert_eq!(garage.can_access_garage(1), Err("Cannot access the garage".to_string()));
    }

}