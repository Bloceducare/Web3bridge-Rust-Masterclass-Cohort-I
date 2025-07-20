pub mod state;

// Import the types from our state module for internal use
use crate::state::{EmployeeInfo, Employee, EmployeeStatus, EmployeeType, AccessControl};
use uuid::Uuid;
use std::collections::HashMap;



impl EmployeeInfo {
    /// Creates a new EmployeeInfo instance
    pub fn new() -> Self {
        Self {
            employee_data: Vec::new(),
            next_id: 1,
            active_keys: HashMap::new(),
        }
    }

    /// Adds a new employee to the system
    pub fn add_employee(&mut self, name: String, employee_type: EmployeeType, status: EmployeeStatus) -> u32 {
        let current_id = self.next_id;
        let employee = Employee {
            id: current_id,
            name,
            employee_type,
            status,
            access: AccessControl::Granted,
        };
        self.next_id += 1;
        self.employee_data.push(employee);
        current_id
    }

    /// Updates an existing employee's information
    pub fn update_employee(&mut self, id: u32, new_name: String, new_type: EmployeeType) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.name = new_name;
            employee.employee_type = new_type;
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }

    /// Gets an employee by ID
    pub fn get_employee(&self, id: u32) -> Result<&Employee, String> {
        self.employee_data
            .iter()
            .find(|e| e.id == id)
            .ok_or(format!("Employee with ID {} not found", id))
    }

    /// Generates an access key for an employee if they have access
    pub fn generate_access_key(&mut self, id: u32) -> Result<String, String> {
        let access = self.can_access_garage(id)?;
        match access {
            AccessControl::Granted => {
                let key = Uuid::new_v4().to_string();
                // Store the key for future validation
                self.active_keys.insert(key.clone(), id);
                Ok(key)
            },
            AccessControl::Denied => Err(format!("Employee with ID {} does not have access", id)),
        }
    }

    /// Determines if an employee can access the garage
    /// Only Media, IT, and Manager roles have access
    /// Terminated employees cannot access regardless of their role
    pub fn can_access_garage(&self, id: u32) -> Result<AccessControl, String> {
        let employee = self.get_employee(id)?;
        
        // Terminated employees cannot access regardless of position
        if let EmployeeStatus::Terminated = employee.status {
            return Ok(AccessControl::Denied);
        }

        // Check if employee type has access
        match employee.employee_type {
            EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager => Ok(AccessControl::Granted),
            _ => Ok(AccessControl::Denied),
        }
    }

    /// Terminates an employee and revokes their access
    pub fn terminate_employee(&mut self, id: u32) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.status = EmployeeStatus::Terminated;
            employee.access = AccessControl::Denied;
            
            // Revoke all active keys for this employee
            self.revoke_employee_keys(id);
            
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }

    /// Revokes all active keys for a specific employee
    fn revoke_employee_keys(&mut self, employee_id: u32) {
        self.active_keys.retain(|_key, &mut id| id != employee_id);
    }

    /// Gets all employees in the system
    pub fn get_all_employees(&self) -> &Vec<Employee> {
        &self.employee_data
    }

    /// Helper function that uses the ? operator to print access status
    /// This function demonstrates the use of the ? operator as requested
    pub fn print_access_status(&self, id: u32) -> Result<(), String> {
        let employee = self.get_employee(id)?;
        let access = self.can_access_garage(id)?;
        
        match access {
            AccessControl::Granted => {
                println!("✅ Employee {} ({}) may access the building", employee.name, employee.id);
            },
            AccessControl::Denied => {
                let reason = match employee.status {
                    EmployeeStatus::Terminated => "terminated",
                    _ => "not authorized for this area"
                };
                println!("❌ Employee {} ({}) may NOT access the building ({})", employee.name, employee.id, reason);
            }
        }
        Ok(())
    }

    /// **MAIN FUNCTION: Uses generated key to grant building access**
    /// This function demonstrates using the ? operator to validate and grant access
    pub fn grant_access_with_key(&self, access_key: &str) -> Result<String, String> {
        // Check if the key exists in our system (uses ? operator for Option -> Result conversion)
        let employee_id = self.active_keys.get(access_key)
            .ok_or("Invalid or expired access key".to_string())?;

        // Get the employee information (uses ? operator)
        let employee = self.get_employee(*employee_id)?;

        // Double-check that the employee still has access (uses ? operator)
        let current_access = self.can_access_garage(*employee_id)?;

        match current_access {
            AccessControl::Granted => {
                Ok(format!(
                    "🟢 ACCESS GRANTED! Welcome {}, you may enter the Web3Bridge garage. 
                    Employee ID: {}, Role: {:?}, Key: {}...{}",
                    employee.name,
                    employee.id,
                    employee.employee_type,
                    &access_key[..8],  // Show first 8 chars of key
                    &access_key[access_key.len()-4..]  // Show last 4 chars of key
                ))
            },
            AccessControl::Denied => {
                Err(format!(
                    "🔴 ACCESS DENIED! Employee {} no longer has garage access. 
                    Status: {:?}",
                    employee.name,
                    employee.status
                ))
            }
        }
    }

    /// Validates an access key and prints the result (uses ? operator)
    pub fn validate_and_print_access(&self, access_key: &str) -> Result<(), String> {
        let result = self.grant_access_with_key(access_key)?;
        println!("{}", result);
        Ok(())
    }

    /// Lists all active access keys (for administrative purposes)
    pub fn list_active_keys(&self) -> Vec<(String, u32, String)> {
        self.active_keys
            .iter()
            .filter_map(|(key, &employee_id)| {
                self.get_employee(employee_id).ok().map(|emp| {
                    (key.clone(), employee_id, emp.name.clone())
                })
            })
            .collect()
    }

    /// Revokes a specific access key
    pub fn revoke_access_key(&mut self, access_key: &str) -> Result<(), String> {
        match self.active_keys.remove(access_key) {
            Some(employee_id) => {
                let employee = self.get_employee(employee_id)?;
                println!("🔑 Access key revoked for employee: {}", employee.name);
                Ok(())
            },
            None => Err("Access key not found".to_string())
        }
    }
}

// impl Default for EmployeeInfo {
//     fn default() -> Self {
//         Self::new()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> EmployeeInfo {
        let mut info = EmployeeInfo::new();
        info.add_employee("Wilfred".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
        info.add_employee("Chris".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
        info.add_employee("Richard".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
        info.add_employee("Charlie".to_string(), EmployeeType::TechnicianSupervisor, EmployeeStatus::Employed);
        info.add_employee("Mike".to_string(), EmployeeType::KitchenStaff, EmployeeStatus::Employed);
        info
    }

    #[test]
    fn test_add_employee() {
        let info = setup();
        assert_eq!(info.employee_data.len(), 5);
        assert_eq!(info.employee_data[0].name, "Wilfred");
        assert_eq!(info.employee_data[1].employee_type, EmployeeType::Media);
    }

    #[test]
    fn test_can_access_garage() {
        let info = setup();
        
        // IT employee should have access
        assert_eq!(info.can_access_garage(1).unwrap(), AccessControl::Granted);
        
        // Media employee should have access
        assert_eq!(info.can_access_garage(2).unwrap(), AccessControl::Granted);
        
        // Social media employee should NOT have access
        assert_eq!(info.can_access_garage(3).unwrap(), AccessControl::Denied);
        
        // Technician supervisor should NOT have access
        assert_eq!(info.can_access_garage(4).unwrap(), AccessControl::Denied);
        
        // Kitchen staff should NOT have access
        assert_eq!(info.can_access_garage(5).unwrap(), AccessControl::Denied);
    }

    #[test]
    fn test_generate_access_key() {
        let mut info = setup();
        
        // IT employee should get access key
        let key = info.generate_access_key(1);
        assert!(key.is_ok());
        assert!(!key.unwrap().is_empty());
        
        // Media employee should get access key
        let key2 = info.generate_access_key(2);
        assert!(key2.is_ok());
        assert!(!key2.unwrap().is_empty());
        
        // Social media employee should be denied
        let key3 = info.generate_access_key(3);
        assert!(key3.is_err());
        assert_eq!(key3.unwrap_err(), "Employee with ID 3 does not have access");
    }

    #[test]
    fn test_key_based_access() {
        let mut info = setup();
        
        // Generate key for IT employee
        let key = info.generate_access_key(1).unwrap();
        
        // Key should grant access
        let access_result = info.grant_access_with_key(&key);
        assert!(access_result.is_ok());
        assert!(access_result.unwrap().contains("ACCESS GRANTED"));
        
        // Invalid key should be denied
        let invalid_access = info.grant_access_with_key("invalid-key");
        assert!(invalid_access.is_err());
        assert!(invalid_access.unwrap_err().contains("Invalid or expired"));
    }

    #[test]
    fn test_terminated_employee_access() {
        let mut info = setup();
        
        // Add a manager who should initially have access
        info.add_employee("Manager".to_string(), EmployeeType::Manager, EmployeeStatus::Employed);
        let manager_id = info.employee_data.len() as u32;
        assert_eq!(info.can_access_garage(manager_id).unwrap(), AccessControl::Granted);
        
        // Generate key before termination
        let key = info.generate_access_key(manager_id).unwrap();
        assert!(info.grant_access_with_key(&key).is_ok());
        
        // After termination, should lose access
        info.terminate_employee(manager_id).unwrap();
        assert_eq!(info.can_access_garage(manager_id).unwrap(), AccessControl::Denied);
        
        // Should not be able to generate new access key
        assert!(info.generate_access_key(manager_id).is_err());
        
        // Old key should no longer work (keys are revoked on termination)
        assert!(info.grant_access_with_key(&key).is_err());
    }

    #[test]
    fn test_manager_access() {
        let mut info = EmployeeInfo::new();
        let manager_id = info.add_employee("Manager".to_string(), EmployeeType::Manager, EmployeeStatus::Employed);
        
        // Manager should have access
        assert_eq!(info.can_access_garage(manager_id).unwrap(), AccessControl::Granted);
        
        // Should be able to generate access key
        assert!(info.generate_access_key(manager_id).is_ok());
    }

    #[test]
    fn test_update_employee() {
        let mut info = setup();
        
        // Update employee name and type
        let result = info.update_employee(3, "Richard Updated".to_string(), EmployeeType::Manager);
        assert!(result.is_ok());
        
        // Check that the update was successful
        let employee = info.get_employee(3).unwrap();
        assert_eq!(employee.name, "Richard Updated");
        assert_eq!(employee.employee_type, EmployeeType::Manager);
        
        // Now they should have access as a manager
        assert_eq!(info.can_access_garage(3).unwrap(), AccessControl::Granted);
    }

    #[test]
    fn test_print_access_status_uses_question_mark() {
        let info = setup();
        
        // This should work without panicking (uses ? operator internally)
        assert!(info.print_access_status(1).is_ok());
        assert!(info.print_access_status(2).is_ok());
        assert!(info.print_access_status(3).is_ok());
        
        // Invalid ID should return error
        assert!(info.print_access_status(999).is_err());
    }

    #[test]
    fn test_get_employee_not_found() {
        let info = setup();
        
        let result = info.get_employee(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Employee with ID 999 not found");
    }

    #[test]
    fn test_terminate_employee_not_found() {
        let mut info = setup();
        
        let result = info.terminate_employee(999);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Employee with ID 999 not found");
    }

    #[test]
    fn test_all_employee_types_access() {
        let mut info = EmployeeInfo::new();
        
        // Add one employee of each type
        let media_id = info.add_employee("Media".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
        let it_id = info.add_employee("IT".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
        let manager_id = info.add_employee("Manager".to_string(), EmployeeType::Manager, EmployeeStatus::Employed);
        let social_id = info.add_employee("Social".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
        let tech_id = info.add_employee("Tech".to_string(), EmployeeType::TechnicianSupervisor, EmployeeStatus::Employed);
        let kitchen_id = info.add_employee("Kitchen".to_string(), EmployeeType::KitchenStaff, EmployeeStatus::Employed);
        
        // Test access for each type
        assert_eq!(info.can_access_garage(media_id).unwrap(), AccessControl::Granted);
        assert_eq!(info.can_access_garage(it_id).unwrap(), AccessControl::Granted);
        assert_eq!(info.can_access_garage(manager_id).unwrap(), AccessControl::Granted);
        assert_eq!(info.can_access_garage(social_id).unwrap(), AccessControl::Denied);
        assert_eq!(info.can_access_garage(tech_id).unwrap(), AccessControl::Denied);
        assert_eq!(info.can_access_garage(kitchen_id).unwrap(), AccessControl::Denied);
    }

    #[test]
    fn test_key_revocation() {
        let mut info = EmployeeInfo::new();
        let emp_id = info.add_employee("Test".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
        
        // Generate key
        let key = info.generate_access_key(emp_id).unwrap();
        
        // Key should work initially
        assert!(info.grant_access_with_key(&key).is_ok());
        
        // Revoke the key
        assert!(info.revoke_access_key(&key).is_ok());
        
        // Key should no longer work
        assert!(info.grant_access_with_key(&key).is_err());
        
        // Revoking non-existent key should fail
        assert!(info.revoke_access_key("non-existent-key").is_err());
    }

    #[test]
    fn test_validate_and_print_access_uses_question_mark() {
        let mut info = setup();
        let key = info.generate_access_key(1).unwrap();
        
        // Valid key should work (uses ? operator internally)
        assert!(info.validate_and_print_access(&key).is_ok());
        
        // Invalid key should return error (uses ? operator internally)
        assert!(info.validate_and_print_access("invalid-key").is_err());
    }

    #[test]
    fn test_list_active_keys() {
        let mut info = setup();
        
        // Generate some keys
        let key1 = info.generate_access_key(1).unwrap(); // IT - should work
        let key2 = info.generate_access_key(2).unwrap(); // Media - should work
        
        // List active keys
        let active_keys = info.list_active_keys();
        assert_eq!(active_keys.len(), 2);
        
        // Check that both keys are in the list
        let key_strings: Vec<String> = active_keys.iter().map(|(k, _, _)| k.clone()).collect();
        assert!(key_strings.contains(&key1));
        assert!(key_strings.contains(&key2));
        
        // Check that employee names are correct
        let names: Vec<String> = active_keys.iter().map(|(_, _, name)| name.clone()).collect();
        assert!(names.contains(&"Wilfred".to_string()));
        assert!(names.contains(&"Chris".to_string()));
    }

    #[test]
    fn test_complete_workflow_from_main() {
        let mut system = EmployeeInfo::new();
        
        // Simulate the exact workflow from main.rs
        let alice_id = system.add_employee("Alice".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
        let bob_id = system.add_employee("Bob".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
        let david_id = system.add_employee("David".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
        
        // Generate keys
        let alice_key = system.generate_access_key(alice_id).unwrap();
        assert!(system.generate_access_key(david_id).is_err()); // Should fail for SocialMedia
        
        // Test access
        assert!(system.grant_access_with_key(&alice_key).is_ok());
        assert!(system.grant_access_with_key("invalid-key").is_err());
        
        // Terminate and test
        let bob_key = system.generate_access_key(bob_id).unwrap();
        assert!(system.validate_and_print_access(&bob_key).is_ok());
        
        system.terminate_employee(bob_id).unwrap();
        assert!(system.grant_access_with_key(&bob_key).is_err()); // Should fail after termination
        
        // Update employee and test
        system.update_employee(david_id, "David (Manager)".to_string(), EmployeeType::Manager).unwrap();
        assert!(system.generate_access_key(david_id).is_ok()); // Should work now as Manager
        
        // Key management
        let active_keys = system.list_active_keys();
        assert!(!active_keys.is_empty());
        
        system.revoke_access_key(&alice_key).unwrap();
        assert!(system.grant_access_with_key(&alice_key).is_err()); // Should fail after revocation
    }
}