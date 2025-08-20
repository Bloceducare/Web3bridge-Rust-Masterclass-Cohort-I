pub mod employees;
pub mod garage_logic;

#[cfg(test)]
mod tests {
    use super::employees::{Employee, EmployeeType};
    use super::garage_logic::can_access;

    fn setup(role: EmployeeType, is_employed: bool) -> Employee {
        Employee::new(role, is_employed)
    }

    #[test]
    fn test_it_employee_access_granted() {
        let emp = setup(EmployeeType::IT, true);
        assert!(can_access(&emp).is_ok());
    }

    #[test]
    fn test_terminated_manager_access_denied() {
        let emp = setup(EmployeeType::Manager, false);
        assert_eq!(
            can_access(&emp).unwrap_err(),
            "Access denied: Employee is terminated."
        );
    }

    #[test]
    fn test_kitchen_staff_access_denied() {
        let emp = setup(EmployeeType::Kitchen, true);
        assert_eq!(
            can_access(&emp).unwrap_err(),
            "Access denied: Not authorized for garage access."
        );
    }

    #[test]
    fn test_social_media_employee_access_denied() {
        let emp = setup(EmployeeType::SocialMedia, true);
        assert!(can_access(&emp).is_err());
    }

    #[test]
    fn test_media_employee_access_granted() {
        let emp = setup(EmployeeType::Media, true);
        assert!(can_access(&emp).is_ok());
    }
}

