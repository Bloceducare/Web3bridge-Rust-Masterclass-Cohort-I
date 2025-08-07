#[derive(Debug, PartialEq)]
pub enum Department {
    Media,
    IT,
    Manager,
    SocialMedia,
    Technician,
    Kitchen,
}

#[derive(Debug, PartialEq)]
pub enum EmploymentStatus {
    Terminated,
    Active,
}

#[derive(Debug)]
pub enum AccessFeedback {
    AccessGranted,
    AccessDenied,
}

#[derive(Debug)]
pub struct Employee {
    pub employee_id: u32,
    pub department: Department,
    pub employment_status: EmploymentStatus,
    pub has_access: bool,
}

#[derive(Debug)]
pub struct AllEmployees {
    pub all_employees: Vec<Employee>,
    pub next_id: u32,
}

impl AllEmployees {
    pub fn new() -> AllEmployees {
        AllEmployees {
            all_employees: Vec::new(),
            next_id: 1,
        }
    }
    pub fn add_employee(&mut self, department: Department) {
        let has_access = match department {
            Department::IT => true,
            Department::Kitchen => false,
            Department::Manager => true,
            Department::Media => true,
            Department::SocialMedia => false,
            Department::Technician => false,
        };
        let new_employee = Employee {
            employee_id: self.next_id,
            department,
            employment_status: EmploymentStatus::Active,
            has_access,
        };
        self.next_id += 1;
        self.all_employees.push(new_employee);
    }
    pub fn check_employee_access(&self, employee_id: u32) -> Result<(), AccessFeedback> {
        let employee = self
            .all_employees
            .iter()
            .find(|e| e.employee_id == employee_id);
        if let Some(employee) = employee {
            if employee.employment_status == EmploymentStatus::Active && employee.has_access {
                return Ok(());
            }
        }
        Err(AccessFeedback::AccessDenied)
    }

    pub fn update_employee_status(&mut self, employee_id: u32, status: EmploymentStatus) {
        let employee = self.all_employees.iter_mut().find(|e| e.employee_id == employee_id);
        if let Some(employee) = employee {
            employee.employment_status = status;
        }
    }
}

impl Employee {
    pub fn id(&self) -> u32 {
        self.employee_id
    }
    pub fn enter_garage(&self, org: &AllEmployees) -> Result<(), AccessFeedback> {
        org.check_employee_access(self.employee_id)?;
        println!(
            "Employee allowed to enter the garage. ({:?} department).",
            self.department
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_employee() {
        let mut employees = AllEmployees::new();

        employees.add_employee(Department::SocialMedia);
        println!("{employees:?}");
        assert_eq!(employees.all_employees.len(), 1);
    }

    #[test]
    fn test_check_employee_access() {
        let mut employees = AllEmployees::new();

        employees.add_employee(Department::Media);
        employees.add_employee(Department::Kitchen);
        let feedback = match employees.check_employee_access(1) {
            Ok(_) => "Access granted!",
            Err(_) => "Access Denied!",
        };
        assert_eq!(feedback, "Access granted!");

        let feedback = match employees.check_employee_access(2) {
            Ok(_) => "Access granted!",
            Err(_) => "Access Denied!",
        };
        assert_eq!(feedback, "Access Denied!");
    }

    #[test]
    fn test_update_employee_status() {
        let mut employees = AllEmployees::new();
        employees.add_employee(Department::Media);
        employees.update_employee_status(1, EmploymentStatus::Terminated);
        assert_eq!(employees.all_employees[0].employment_status, EmploymentStatus::Terminated);
    }
}