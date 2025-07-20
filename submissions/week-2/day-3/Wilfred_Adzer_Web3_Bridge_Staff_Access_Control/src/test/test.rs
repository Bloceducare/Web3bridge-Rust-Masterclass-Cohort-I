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
        
        // Kitchen staff should NOT have access
        assert_eq!(info.can_access_garage(5).unwrap(), AccessControl::Denied);
    }

    #[test]
    fn test_generate_access_key() {
        let info = setup();
        
        // IT employee should get access key
        let key = info.generate_access_key(1);
        assert!(key.is_ok());
        
        // Social media employee should be denied
        let key2 = info.generate_access_key(3);
        assert!(key2.is_err());
        assert_eq!(key2.unwrap_err(), "Employee with ID 3 does not have access");
    }

    #[test]
    fn test_terminated_employee_access() {
        let mut info = setup();
        
        // Manager should initially have access
        info.add_employee("Manager".to_string(), EmployeeType::Manager, EmployeeStatus::Employed);
        let manager_id = info.employee_data.len() as u32;
        assert_eq!(info.can_access_garage(manager_id).unwrap(), AccessControl::Granted);
        
        // After termination, should lose access
        info.terminate_employee(manager_id).unwrap();
        assert_eq!(info.can_access_garage(manager_id).unwrap(), AccessControl::Denied);
        
        // Should not be able to generate access key
        assert!(info.generate_access_key(manager_id).is_err());
    }

    #[test]
    fn test_print_access_status_uses_question_mark() {
        let info = setup();
        
        // This should work without panicking (uses ? operator internally)
        assert!(info.print_access_status(1).is_ok());
        
        // Invalid ID should return error
        assert!(info.print_access_status(999).is_err());
    }
}