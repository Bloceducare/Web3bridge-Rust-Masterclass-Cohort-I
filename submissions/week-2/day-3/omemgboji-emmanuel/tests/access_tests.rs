use omemgboji_emmanuel::enums::EmployeeRoles;
use omemgboji_emmanuel::structs::Employee;

#[test]
fn test_access_garage() {
    let employee = Employee {
        role: EmployeeRoles::ITDepartment,
        is_employed: true,
    };
    assert!(employee.access_garage().is_ok());
}

#[test]
fn test_access_garage_denied() {
    let employee = Employee {
        role: EmployeeRoles::KitchenStaff,
        is_employed: true,
    };
    assert!(employee.access_garage().is_err());
}

#[test]
fn test_access_garage_not_employed() {
    let employee = Employee {
        role: EmployeeRoles::MediaTeam,
        is_employed: false,
    };
    assert!(employee.access_garage().is_err());
}
