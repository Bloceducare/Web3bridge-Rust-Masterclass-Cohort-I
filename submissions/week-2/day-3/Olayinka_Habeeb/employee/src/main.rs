
#[derive(Debug, PartialEq)]
enum Employee {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMediaTeam,
    TechnicalSupervisor,
    KitchenStaff,
}

struct EmployeeCheck {
    role: Employee,
    is_employed: bool,
}

impl EmployeeCheck {
    fn can_access(&self, allowed_roles: &[Employee]) -> Result<(), String> {
        if !self.is_employed {
            return Err(String::from("Access denied"));
        }
        if allowed_roles.contains(&self.role) {
            Ok(())
        } else {
            Err(String::from("Access denied: Role not permitted."))
        }
    }
}


fn check_employee_access(emp: &EmployeeCheck, allowed_roles: &[Employee]) -> Result<(), String> {
    emp.can_access(allowed_roles)?;
    println!("Access granted: Welcome to the Web3Bridge garage!");
    Ok(())
}

fn main() {
    let manager = EmployeeCheck { 
        role: Employee::Manager, 
        is_employed: true };

    let kitchen_staff = EmployeeCheck{ 
        role: Employee::KitchenStaff,
         is_employed: true };

    let not_employed = EmployeeCheck { 
        role: Employee::ITDepartment, 
        is_employed: false };

   let garage_roles = &[Employee::Manager, Employee::ITDepartment, Employee::KitchenStaff, Employee::TechnicalSupervisor];
    match check_employee_access(&manager, garage_roles) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e),
    }
    match check_employee_access(&kitchen_staff, garage_roles) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e),
    }
    match check_employee_access(&not_employed, garage_roles) {
        Ok(_) => {},
        Err(e) => println!("Error: {}", e),
    }
}

