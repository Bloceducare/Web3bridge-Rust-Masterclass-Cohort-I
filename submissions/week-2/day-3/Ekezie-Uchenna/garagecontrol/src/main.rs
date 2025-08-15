use uuid::Uuid;

#[derive(Debug, PartialEq)]
enum EmployeeType {
    Media,
    IT,
    Manager, 
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug, PartialEq)]
enum EmployeeStatus {
    Employed,
    Terminated,
}

#[derive(Debug, PartialEq)]
enum AccessControl {
    Granted,
    Denied
}

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    employee_type: EmployeeType,
    status: EmployeeStatus,
    access: AccessControl,
}

struct EmployeeInfo {
    employee_data: Vec<Employee>,
    next_id: u32,
}

impl EmployeeInfo {
    fn new() -> Self {
        Self {
            employee_data: Vec::new(),
            next_id: 1,
        }
    }

    fn add_employee(&mut self, name: String, employee_type: EmployeeType, status: EmployeeStatus) -> u32 {
        let id = self.next_id;
        
        let access = match (&employee_type, &status) {
            (EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager, EmployeeStatus::Employed) => {
                AccessControl::Granted
            }
            _ => AccessControl::Denied,
        };
        
        let employee = Employee {
            id,
            name,
            employee_type,
            status,
            access,
        };
        
        self.next_id += 1;
        self.employee_data.push(employee);
        id
    }

    fn update_employee(&mut self, id: u32, new_name: String, new_type: EmployeeType) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.name = new_name;
            employee.employee_type = new_type;
            
            employee.access = match (&employee.employee_type, &employee.status) {
                (EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager, EmployeeStatus::Employed) => {
                    AccessControl::Granted
                }
                _ => AccessControl::Denied,
            };
            
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }
    
    fn get_employee(&self, id: u32) -> Result<&Employee, String> {
        self.employee_data
            .iter()
            .find(|e| e.id == id)
            .ok_or(format!("Employee with ID {} not found", id))
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

    fn terminate_employee(&mut self, id: u32) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.status = EmployeeStatus::Terminated;
            employee.access = AccessControl::Denied;
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }

    fn get_all_employees(&self) -> &Vec<Employee> {
        &self.employee_data
    }
}

fn main() {
    let mut employee_info = EmployeeInfo::new();
    
    // Add some employees
    let uche_id = employee_info.add_employee("Uche".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
    let prince_id = employee_info.add_employee("Prince".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
    let ridwaan_id = employee_info.add_employee("Ridwaan".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> EmployeeInfo {
        let mut employee_info = EmployeeInfo::new();
        employee_info.add_employee("Uche".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
        employee_info.add_employee("Prince".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
        employee_info.add_employee("Ridwaan".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
        employee_info.add_employee("JTMax".to_string(), EmployeeType::TechnicianSupervisor, EmployeeStatus::Employed);
        employee_info.add_employee("Oladele".to_string(), EmployeeType::KitchenStaff, EmployeeStatus::Employed);

        employee_info
    }

    #[test]
    fn test_add_employee() {
        let employee_info = setup();
        assert_eq!(employee_info.employee_data.len(), 5);
        assert_eq!(employee_info.employee_data[0].name, "Uche");
        assert_eq!(employee_info.employee_data[1].name, "Prince");
        assert_eq!(employee_info.employee_data[2].name, "Ridwaan");
        assert_eq!(employee_info.employee_data[3].name, "JTMax");
        assert_eq!(employee_info.employee_data[4].name, "Oladele");
        assert_eq!(employee_info.employee_data[0].employee_type, EmployeeType::IT);
        

        assert_eq!(employee_info.employee_data[0].status, EmployeeStatus::Employed);
        assert_eq!(employee_info.employee_data[1].status, EmployeeStatus::Employed);
    }

    #[test]
    fn test_employee_access_types() {
        let employee_info = setup();
        
        // Test that IT, Media employees have access when employed
        assert_eq!(employee_info.employee_data[0].access, AccessControl::Granted); // IT
        assert_eq!(employee_info.employee_data[1].access, AccessControl::Granted); // Media
        
        // Test that other employees don't have access
        assert_eq!(employee_info.employee_data[2].access, AccessControl::Denied); // SocialMedia
        assert_eq!(employee_info.employee_data[3].access, AccessControl::Denied); // TechnicianSupervisor
        assert_eq!(employee_info.employee_data[4].access, AccessControl::Denied); // KitchenStaff
    }

    #[test]
    fn test_generate_access_key() {
        let employee_info = setup();
        
        // Test that authorized employees get keys
        let key = employee_info.generate_access_key(1); // Uche (IT)
        assert!(key.is_ok());
        assert!(!key.unwrap().is_empty());

        let key = employee_info.generate_access_key(2); // Prince (Media)
        assert!(key.is_ok());
        assert!(!key.unwrap().is_empty());

        // Test that unauthorized employees don't get keys
        let key = employee_info.generate_access_key(3); // Ridwaan (SocialMedia)
        assert!(key.is_err());
        assert_eq!(key.unwrap_err(), "Employee with ID 3 does not have access");
    }

    #[test]
    fn test_terminate_employee() {
        let mut employee_info = setup();
        
        // Terminate an employee
        let result = employee_info.terminate_employee(1);
        assert!(result.is_ok());
        let key = employee_info.generate_access_key(1);
        assert!(key.is_err());
        assert_eq!(key.unwrap_err(), "Employee does not have access");
    }

    #[test]
    fn test_update_employee() {
        let mut employee_info = setup();
        
        let result = employee_info.update_employee(3, "Richard Manager".to_string(), EmployeeType::Manager);
        assert!(result.is_ok());
        
      
        let employee = employee_info.get_employee(3).unwrap();
        assert_eq!(employee.employee_type, EmployeeType::Manager);
        assert_eq!(employee.access, AccessControl::Granted);
        
        let key = employee_info.generate_access_key(3);
        assert!(key.is_ok());
    }

    #[test]
    fn test_get_employee() {
        let employee_info = setup();
        
        
        let employee = employee_info.get_employee(1);
        assert!(employee.is_ok());
        assert_eq!(employee.unwrap().name, "Uche");
        
    
        let employee = employee_info.get_employee(999);
        assert!(employee.is_err());
        assert_eq!(employee.unwrap_err(), "Employee with ID 999 not found");
    }
}