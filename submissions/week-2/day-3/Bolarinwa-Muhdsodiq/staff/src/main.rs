#[derive(Debug, Clone, PartialEq)]
enum EmployeeRole {
    MEDIATEAM,
    ITDEPARTMENT,
    MANAGER,
    SOCIALMEDIATEAM,
    TECHNICIANSUPERVISOR,
    KITCHENSTAFF,
}

#[derive(Debug, Clone, PartialEq)]
enum EmployeeStatus {
    ACTIVE,
    INACTIVE,
    ONLEAVE,
    TERMINATED,
}

struct Employee {
    id: u32,
    name: String,
    status: EmployeeStatus,
    role: EmployeeRole,
    employed: bool,
}

struct EmployeeData {
    data: Vec<Employee>,
    next_id: u32,
}

impl EmployeeData {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    fn register(&mut self, name: String, role: EmployeeRole) -> u32 {
        let present_id = self.next_id;
        let employee = Employee {
            id: present_id,
            name,
            status: EmployeeStatus::ACTIVE,
            role,
            employed: true,
        };
        self.next_id += 1;
        self.data.push(employee);
        present_id
    }

    fn update_employ(&mut self, id: u32, new_name: String) -> bool {
        if let Some(employee) = self.data.iter_mut().find(|emp| emp.id == id) {
            employee.name = new_name;
            true
        } else {
            false
        }
    }

    fn update_employee_status(&mut self, id: u32, status: EmployeeStatus) -> bool {
        if let Some(employee) = self.data.iter_mut().find(|emp| emp.id == id) {
            employee.status = status;
            true
        } else {
            false
        }
    }

    fn evit_employee(&mut self, id: u32) -> bool {
        if let Some(employee_index) = self.data.iter().position(|employee| employee.id == id) {
            self.data.remove(employee_index);
            true
        } else {
            false
        }
    }

    fn get_all_employee(&self) -> &Vec<Employee> {
        &self.data
    }
}

fn can_access_garage(employee: &Employee) -> Result<(), String> {
    if !employee.employed {
        return Err("Access denied: Employee is terminated".to_string());
    }

    match employee.role {
        EmployeeRole::MEDIATEAM | EmployeeRole::ITDEPARTMENT | EmployeeRole::MANAGER => Ok(()),
        EmployeeRole::SOCIALMEDIATEAM
        | EmployeeRole::TECHNICIANSUPERVISOR
        | EmployeeRole::KITCHENSTAFF => {
            Err("Access denied: This role does not have garage access".to_string())
        }
    }
}

fn print_access_result(employee: &Employee) -> Result<(), String> {
    can_access_garage(employee)?; // This uses the ? operator
    println!("Access granted to garage");
    Ok(())
}

fn main() {
    let media_employee = Employee {
        id: 1,
        name: "John Media".to_string(),
        status: EmployeeStatus::ACTIVE,
        role: EmployeeRole::MEDIATEAM,
        employed: true,
    };

    let it_employee = Employee {
        id: 2,
        name: "Jane IT".to_string(),
        status: EmployeeStatus::ACTIVE,
        role: EmployeeRole::ITDEPARTMENT,
        employed: true,
    };

    let terminated_manager = Employee {
        id: 3,
        name: "Bob Manager".to_string(),
        status: EmployeeStatus::TERMINATED,
        role: EmployeeRole::MANAGER,
        employed: false,
    };

    let kitchen_staff = Employee {
        id: 4,
        name: "Alice Kitchen".to_string(),
        status: EmployeeStatus::ACTIVE,
        role: EmployeeRole::KITCHENSTAFF,
        employed: true,
    };

    println!("Testing garage access:");
    println!("--------------");

    // Media team (should have access)
    println!("1. {} (Media Team):", media_employee.name);
    match print_access_result(&media_employee) {
        Ok(_) => println!("   ✓ Success"),
        Err(e) => println!("   ✗ Error: {}", e),
    }

    // IT department (should have access)
    println!("\n2. {} (IT Department):", it_employee.name);
    match print_access_result(&it_employee) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    //  Terminated manager (should not have access)
    println!("\n3. {} (Terminated Manager):", terminated_manager.name);
    match print_access_result(&terminated_manager) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    // Test 4: Kitchen staff (should not have access)
    println!("\n4. {} (Kitchen Staff):", kitchen_staff.name);
    match print_access_result(&kitchen_staff) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_employee(role: EmployeeRole, employed: bool) -> Employee {
        Employee {
            id: 0,
            name: "Web3bridge".to_string(),
            status: if employed { EmployeeStatus::ACTIVE } else { EmployeeStatus::TERMINATED },
            role,
            employed,
        }
    }

    #[test]
    fn media_team_has_access() {
        let emp = make_employee(EmployeeRole::MEDIATEAM, true);
        assert!(can_access_garage(&emp).is_ok());
    }

    #[test]
    fn it_department_has_access() {
        let emp = make_employee(EmployeeRole::ITDEPARTMENT, true);
        assert!(can_access_garage(&emp).is_ok());
    }

    #[test]
    fn manager_has_access() {
        let emp = make_employee(EmployeeRole::MANAGER, true);
        assert!(can_access_garage(&emp).is_ok());
    }

    #[test]
    fn social_media_team_no_access() {
        let emp = make_employee(EmployeeRole::SOCIALMEDIATEAM, true);
        assert!(can_access_garage(&emp).is_err());
    }

    #[test]
    fn technician_supervisor_no_access() {
        let emp = make_employee(EmployeeRole::TECHNICIANSUPERVISOR, true);
        assert!(can_access_garage(&emp).is_err());
    }

    #[test]
    fn kitchen_staff_no_access() {
        let emp = make_employee(EmployeeRole::KITCHENSTAFF, true);
        assert!(can_access_garage(&emp).is_err());
    }

    #[test]
    fn terminated_employee_no_access_even_if_manager() {
        let emp = make_employee(EmployeeRole::MANAGER, false);
        assert!(can_access_garage(&emp).is_err());
    }

    #[test]
    fn print_access_result_success() {
        let emp = make_employee(EmployeeRole::MEDIATEAM, true);
        assert!(print_access_result(&emp).is_ok());
    }

    #[test]
    fn print_access_result_failure() {
        let emp = make_employee(EmployeeRole::KITCHENSTAFF, true);
        assert!(print_access_result(&emp).is_err());
    }
}