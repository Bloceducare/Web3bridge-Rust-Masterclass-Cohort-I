use Task_1::*;
struct Company {
    employees: Vec<EmployeeDetails>,
}

impl Company {
    fn new() -> Company {
        Company {
            employees: Vec::new(),
        }
    }

    fn add_employee(&mut self, employee: EmployeeDetails) {
        self.employees.push(employee);
    }

    fn remove_employee(&mut self, employee: &EmployeeDetails) {
        self.employees.retain(|e| e != employee);
    }

    fn update_employee_status(&mut self, employee: &EmployeeDetails, new_status: Status) {
        if let Some(emp) = self.employees
            .iter_mut()
            .find(|e| e.name == employee.name) {
                emp.status = new_status;
            }
    }

    fn gate_entry(&self, employee: &EmployeeDetails) -> Result<(), String> {
        match employee.status {
            Status::Employed => match employee.position {
                Employee::MediaTeam 
                | Employee::ITDepartmentEmployees 
                | Employee::Managers => Ok(()),
                _ => Err("Access denied: unauthorized position.".to_string()),
        },
            Status::Terminated => Err("Access denied: unauthorized position.".to_string()),
        }
    }

    fn may_access_building(&self, employee: &EmployeeDetails) -> Result<bool, String> {
        self.gate_entry(employee)?;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_employee() {
        let mut company = Company::new();
        let employee = EmployeeDetails {
            name: String::from("Alice"),
            position: Employee::MediaTeam,
            status: Status::Employed,
        };
        company.add_employee(employee);
        assert_eq!(company.employees.len(), 1);
    }

    #[test]
    fn test_remove_employee() {
        let mut company = Company::new();
        let employee = EmployeeDetails {
            name: String::from("Bob"),
            position: Employee::ITDepartmentEmployees,
            status: Status::Employed,
        };
        company.add_employee(employee);
        company.remove_employee(&EmployeeDetails {
            name: String::from("Bob"),
            position: Employee::ITDepartmentEmployees,
            status: Status::Employed,
        });
        assert_eq!(company.employees.len(), 0);
    }

    #[test]
    fn test_update_employee_status() {
        let mut company = Company::new();
        let mut employee = EmployeeDetails {
            name: String::from("Charlie"),
            position: Employee::Managers,
            status: Status::Employed,
        };
        company.add_employee(employee.clone());
        company.update_employee_status(&employee, Status::Terminated);
        assert_eq!(company.employees[0].status, Status::Terminated);
    }

    #[test]
    fn test_may_access_building() {
        let mut company = Company::new();
        let employee = EmployeeDetails {
            name: String::from("Dave"),
            position: Employee::SocialMediaTeam,
            status: Status::Employed,
        };
        company.add_employee(employee.clone());
        assert!(company.may_access_building(&employee).is_err());

        let employee = EmployeeDetails {
            name: String::from("Eve"),
            position: Employee::ITDepartmentEmployees,
            status: Status::Employed,
        };
        assert!(company.may_access_building(&employee).is_ok());

        let employee = EmployeeDetails {
            name: String::from("Frank"),
            position: Employee::TechnicianSupervisors,
            status: Status::Terminated,
        };
        assert!(company.may_access_building(&employee).is_err());

        let employee = EmployeeDetails {
            name: String::from("Grace"),
            position: Employee::KitchenStaff,
            status: Status::Employed,
        };
        assert!(company.may_access_building(&employee).is_err());
    }
} 

fn main() {
    println!("Hello, world!");
}
