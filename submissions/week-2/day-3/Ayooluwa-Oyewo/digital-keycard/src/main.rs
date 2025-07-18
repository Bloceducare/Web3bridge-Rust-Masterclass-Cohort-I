use digital_keycard::employee::{Employee, EmployeeType};

fn main() {
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

    println!("Employee created: {:?}", employee.data);

    // Using the function with question mark operator
    employee.can_access_building(1);
    employee.can_access_building(2);
    employee.can_access_building(3);
    employee.can_access_building(4);
    employee.can_access_building(5);
}
