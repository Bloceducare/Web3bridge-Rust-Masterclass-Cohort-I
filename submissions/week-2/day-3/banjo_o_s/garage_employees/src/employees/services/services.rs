use crate::employees::constants::constants::{EmployeeRole, EmployeeStatus, OtherEmployeeRole};
use crate::employees::models::models::{Employee, Garage, OtherRoles};
use std::collections::HashMap;

impl EmployeeRole {
    fn check_role(&self) -> Option<OtherEmployeeRole> {
        match self {
            EmployeeRole::OtherEmployeeRole(s) => match s {
                1 => Some(OtherEmployeeRole::Kitchen),
                2 => Some(OtherEmployeeRole::SocialMedia),
                3 => Some(OtherEmployeeRole::Technician),
                _ => None,
            },
            _ => None,
        }
    }
}

impl EmployeeStatus {
    pub fn is_terminated(&self) -> bool {
        match self {
            EmployeeStatus::Terminated => true,
            EmployeeStatus::Engaged => false,
        }
    }
}

impl Employee {
    fn new(name: String, role: EmployeeRole) -> Self {
        Self {
            name,
            role,
            status: EmployeeStatus::Engaged,
        }
    }
    fn change_employee_role(&mut self, role: EmployeeRole) -> bool {
        self.role = role;
        true
    }
    fn terminate_employee(&mut self) -> bool {
        self.status = EmployeeStatus::Terminated;
        true
    }
}
impl Garage {
    pub fn new(name: String) -> Self {
        Self {
            name,
            employees: HashMap::new(),
            other_roles: HashMap::new(),
            id: 1,
        }
    }
    pub fn add_employee(&mut self, name: String, role: EmployeeRole) -> bool {
        let id: u128 = self.id;
        let other_role = role.check_role();
        self.employees.insert(id, Employee::new(name, role));
        if other_role.is_some() {
            self.other_roles.insert(
                id,
                OtherRoles {
                    id,
                    role: other_role.unwrap(),
                },
            );
        }
        self.id += 1;
        true
    }
    pub fn get_employee(&self, id: u128) -> Option<&Employee> {
        let employee = self.employees.get(&id)?;
        Some(employee)
    }

    pub fn terminate_employee(&mut self, id: u128) -> Result<bool, String> {
        let optional_employee = self.employees.get_mut(&id);
        let result: bool = Some(optional_employee)
            .unwrap()
            .expect("e didn't dey")
            .terminate_employee();
        Ok(result)
    }

    pub fn change_employee_role(
        &mut self,
        id: u128,
        new_role: EmployeeRole,
    ) -> Result<bool, String> {
        let optional_employee = self.employees.get_mut(&id);
        let other_role = new_role.check_role();
        if other_role.is_some() {
            self.other_roles.insert(
                id,
                OtherRoles {
                    id,
                    role: other_role.unwrap(),
                },
            );
        } else {
            self.other_roles.remove(&id);
        }
        let result: bool = Some(optional_employee)
            .unwrap()
            .expect("e didn't dey")
            .change_employee_role(new_role);
        Ok(result)
    }

    pub fn is_employed_terminated(&self, id: u128) -> Result<bool, String> {
        let result: Option<&Employee> = self.get_employee(id);
        if result.is_none() {
            return Err("e didn't dey".to_string());
        }
        Ok(result.unwrap().status.is_terminated())
    }

    pub fn has_access(&self, id: u128) -> bool {
        let e = self.get_employee(id).unwrap();
        if e.status.is_terminated() {
            return false;
        } else if e.role == EmployeeRole::Manager
            || e.role == EmployeeRole::IT
            || e.role == EmployeeRole::Media
        {
            return true;
        }
        return false;
    }
}
