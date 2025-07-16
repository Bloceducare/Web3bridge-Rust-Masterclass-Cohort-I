// struct Employee

use std::collections::HashMap;

#[derive(Debug)]
pub enum EmployeeStatus {
    Terminated,
    Engaged,
}

#[derive(Debug)]
pub enum EmployeeRole {
    Media,
    IT,
    Manager,
    OtherEmployeeRole,
}

#[derive(Debug)]
pub enum OtherEmployeeRole {
    Social,
    Technician,
    Kitchen,
}

pub struct Employee {
    pub name: String,
    pub status: EmployeeStatus,
    pub role: EmployeeRole,
}

impl EmployeeStatus {
    pub fn is_terminated(&self) -> bool {
        match self {
            EmployeeStatus::Terminated => true,
            EmployeeStatus::Engaged => false,
        }
    }
}

pub struct Garage {
    pub name: String,
    pub employees: HashMap<u128, Employee>,
    pub id: u128,
}

impl Employee {
    pub fn new(name: String, role: EmployeeRole) -> Self {
        Self {
            name,
            role,
            status: EmployeeStatus::Engaged,
        }
    }
    pub fn change_employee_role(&mut self, role: EmployeeRole) -> bool {
        self.role = role;
        true
    }
    pub fn terminate_employee(&mut self) -> bool {
        self.status = EmployeeStatus::Terminated;
        true
    }
}
impl Garage {
    pub fn new(name: String) -> Self {
        Self {
            name,
            employees: HashMap::new(),
            id: 1,
        }
    }
    pub fn add_employee(&mut self, name: String, role: EmployeeRole) -> bool {
        let id: u128 = self.id;
        self.employees.insert(id, Employee::new(name, role));
        self.id += 1;
        true
    }
    pub fn get_employee(&self, id: u128) -> Option<&Employee> {
        let employee = self.employees.get(&id)?;
        Some(employee)
    }
    
    pub fn terminate_employee(&mut self, id: u128, ) -> Result<bool, String> {
        let optional_employee = self.employees.get_mut(&id);
        let result: bool = Some(optional_employee).unwrap().expect("e didn't dey").terminate_employee();
        Ok(result)
    }

    pub fn change_employee_role(&mut self, id: u128, new_role: EmployeeRole) -> Result<bool, String> {
        let optional_employee = self.employees.get_mut(&id);
        let result: bool = Some(optional_employee).unwrap().expect("e didn't dey").change_employee_role(new_role);
        Ok(result)
    }
}
