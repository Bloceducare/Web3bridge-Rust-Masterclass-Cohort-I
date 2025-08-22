use ::access_control_system::*;
fn main() {
    let mut org = AllEmployees::new();
    org.add_employee(Department::Media);
    org.add_employee(Department::IT);
    org.add_employee(Department::Manager);
    org.add_employee(Department::SocialMedia);
    org.add_employee(Department::Technician);
    org.add_employee(Department::Kitchen);
    println!("{org:?}");


    let _ = match Employee::enter_garage( &org.all_employees[0], &org){
        Ok(_) => println!("Employee entered garage."),
        Err(e) => println!("Access Denied: {:?}", e),
    };
    org.update_employee_status(1, EmploymentStatus::Terminated);
    let _ = match Employee::enter_garage( &org.all_employees[0], &org){
        Ok(_) => println!("Employee entered garage."),
        Err(e) => println!("Access Denied: {:?}", e),
    };
}