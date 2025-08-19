use uuid::Uuid;

#[derive(Debug,PartialEq)]
enum EmployeeTypes {
    Manager,
    IT,
    TechnicianSupervisor,
    Staff,
    KitchenStaff,
    SocialMedia
}

#[derive(Debug,PartialEq)]
enum EmployeeStatus {
    Employed,
    Terminated
}

enum AccessControl {
    Granted,
    Danied
}

struct Employee {
    id: u64,
    employee_type: EmployeeTypes,
    name: String,
    access: AccessControl,
    status: EmployeeStatus
}

struct EmployeeInformation {
    employee_data: Vec<Employee>,
    next_id: u64
}

impl EmployeeInformation {
    fn new() -> Self {
        Self {
            employee_data: Vec::new(),
            next_id: 1
        }
    }

    fn add_employee(&mut self, name: String, employee_type: EmployeeType, status: EmployeeStatus) -> u64 {
        let id = self.next_id;
        let employeeInformation = EmployeeInformation {
            id,
            name,
            employee_type,
            access: AccessControl.Granted
            status: EmployeeStatus.Employed,
        };
        self.next_id = self.next_id + 1;
        self.employee_data.push(employeeInformation);
        return id;
    }

    fn update_employee (&mut self, id: u32, updated_name: String, new_type: EmployeeType) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.name = updated_name;
            employee.employee_type = new_type;
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }

    fn generate_access_key(&self, id: u32) -> Result<String, String> {
        let access = self.can_access_garage(id)?;
        match access {
            AccessControl::Granted => {
                let key = Uuid::new_v4().to_string();
                Ok(key)
            }
            AccessControl::Denied => Err(format!("Employee with ID {} does not have access", id)),
        }
    }

     fn can_access_garage(&self, id: u32) -> Result<AccessControl, String> {
        let employee = self.get_employee(id)?; 

        if let EmployeeStatus::Terminated = employee.status {
            return Ok(AccessControl::Denied);
        }

        match employee.employee_type {
            EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager => {
                Ok(AccessControl::Granted)
            }
            _ => Ok(AccessControl::Denied),
        }
    }
    


}

fn main() {
    println!("Hello, world!");
}
