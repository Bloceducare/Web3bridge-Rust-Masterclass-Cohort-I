#[derive(Debug, PartialEq)]
enum EmployeeType {
    Admin,
    IT,
    Operations,
    SocialMedia,
    Technician,
    Kitchen,
}

#[derive(Debug)]
struct Employee {
    role: EmployeeType,
    is_employed: bool,
}

impl Employee {
    fn new(role: EmployeeType, is_employed: bool) -> Self {
        Employee { role, is_employed }
    }
}

fn check_access(employee: &Employee) -> Result<(), String> {
    can_access(employee)?; 
    println!("Access granted: Welcome to the facility.");
    Ok(())
}

fn can_access(employee: &Employee) -> Result<(), String> {
    if !employee.is_employed {
        return Err("Access denied: Employee's contract has been terminated.".into());
    }

    match employee.role {
        EmployeeType::Admin | EmployeeType::IT | EmployeeType::Operations=> Ok(()),
        _ => Err("Access denied: Not authorized for facility access.".into()),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup(role: EmployeeType, is_employed: bool) -> Employee {
        Employee::new(role, is_employed)
    }

    #[test]
    fn test_it_employee_access_granted() {
        let emp = setup(EmployeeType::IT, true);
        assert!(can_access(&emp).is_ok());
    }

    #[test]
    fn test_kitchen_staff_access_denied() {
        let emp = setup(EmployeeType::Kitchen, true);
        assert_eq!(
            can_access(&emp).unwrap_err(),
            "Access denied: Not authorized for facility access."
        );
    }

    #[test]
    fn test_terminated_operations_access_denied() {
        let emp = setup(EmployeeType::Operations, false);
        assert_eq!(
            can_access(&emp).unwrap_err(),
            "Access denied: Employee's contract has been terminated."
        );
    }

    #[test]
    fn test_social_media_employee_access_denied() {
        let emp = setup(EmployeeType::SocialMedia, true);
        assert!(can_access(&emp).is_err());
    }

    #[test]
    fn test_media_employee_access_granted() {
        let emp = setup(EmployeeType::Admin, true);
        assert!(can_access(&emp).is_ok());
    }
}
