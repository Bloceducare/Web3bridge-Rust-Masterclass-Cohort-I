use crate::employee::{Employee, EmployeeType};
use crate::error::EmployeeError;

#[test]
fn test_add_employee() {
    let mut employee = Employee::new();
    employee.add_employee("John Doe".to_string(), EmployeeType::Media, true);
    assert_eq!(employee.data.len(), 1);
    assert_eq!(employee.data[0].name, "John Doe");
    assert_eq!(employee.data[0].employee_type, EmployeeType::Media);
}

#[test]
fn test_employee_can_access_building() {
    let mut employee = Employee::new();
    employee.add_employee("John Doe".to_string(), EmployeeType::Media, true);
    employee.add_employee("Jane Smith".to_string(), EmployeeType::IT, true);
    employee.add_employee("Jim Beam".to_string(), EmployeeType::Manager, false);
    employee.add_employee("Jill Johnson".to_string(), EmployeeType::SocialMedia, false);
    employee.add_employee("King Kong".to_string(), EmployeeType::SocialMedia, true);
    employee.add_employee(
        "Jack Johnson".to_string(),
        EmployeeType::TechnicianSupervisor,
        true,
    );
    employee.add_employee("Jill Johnson".to_string(), EmployeeType::KitchenStaff, true);

    // Test the internal function that returns Result
    assert_eq!(
        employee.check_access(1),
        Ok("Employee John Doe can access the building".to_string())
    );
    assert_eq!(
        employee.check_access(2),
        Ok("Employee Jane Smith can access the building".to_string())
    );

    // Test terminated employee
    assert_eq!(
        employee.check_access(3),
        Err(EmployeeError::EmployeeTerminated)
    );

    // Test not allowed employee
    assert_eq!(
        employee.check_access(4),
        Err(EmployeeError::EmployeeTerminated)
    );
    assert_eq!(
        employee.check_access(5),
        Err(EmployeeError::EmployeeNotAllowed)
    );
}
