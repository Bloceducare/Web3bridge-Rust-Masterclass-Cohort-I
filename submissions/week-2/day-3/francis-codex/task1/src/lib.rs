pub fn check_building_access(employee: &Employee) -> Result <(), String> {

    if !employee.is_employed {
        return Err("Access Denied | Unemployed".to_string());
    }

    match employee.position {
        Employees::MediaTeam | Employees::IT | Employees::Managers => Ok(()),
        Employees::SocialMedia | Employees::TechnicianSupervisors | Employees::KitchenStaff => Err("Access Denied | Unauthorized Employee".to_string()),
    }
}

pub fn check_employee_access(employee: &Employee) -> Result<(), String> {
    check_building_access(employee)
}

pub fn print_access_result(employee: &Employee) -> Result<(), String> {
    check_employee_access(employee)?;
    println!("Access Granted | Authorized Employee");
    Ok(())
 
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_employee(position: Employees, is_employed: bool) -> Employee {
        Employee {
            position,
            is_employed,
        }
    }

    #[test]
    fn test_check_building_access() {
        let employee = setup_employee(Employees::IT, true);
        assert!(check_building_access(&employee).is_ok()); 
        let employee1 = setup_employee(Employees::KitchenStaff, true);
        assert!(check_building_access(&employee1).is_err());
        let employee2 = setup_employee(Employees::SocialMedia, false);
        assert!(check_building_access(&employee2).is_err());
        let employee3 = setup_employee(Employees::Managers, true);
        assert!(check_building_access(&employee3).is_ok());
    }   

    #[test]
    fn test_check_employee_access() {
        let employee = setup_employee(Employees::MediaTeam, true);
        assert!(check_employee_access(&employee).is_ok());
        let employee1 = setup_employee(Employees::TechnicianSupervisors, true);
        assert!(check_employee_access(&employee1).is_err());
        let employee2 = setup_employee(Employees::IT, false);
        assert!(check_employee_access(&employee2).is_err());
        let employee3 = setup_employee(Employees::KitchenStaff, true);
        assert!(check_employee_access(&employee3).is_err());
    }   

    #[test]
    fn test_print_access_result() {
        let employee = setup_employee(Employees::MediaTeam, true);
        assert!(print_access_result(&employee).is_ok());
        let employee1 = setup_employee(Employees::SocialMedia, true);
        assert!(print_access_result(&employee1).is_err());
        let employee2 = setup_employee(Employees::IT, false);
        assert!(print_access_result(&employee2).is_err());
        let employee3 = setup_employee(Employees::Managers, true);
        assert!(print_access_result(&employee3).is_ok());
    }
}