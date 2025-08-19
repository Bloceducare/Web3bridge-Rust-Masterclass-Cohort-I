use garage_access_control::state::Employee;
use garage_access_control::state::EmployeeType;




fn main() {
    let manager = Employee::new(EmployeeType::Manager, true);
    let it_employee = Employee::new(EmployeeType::IT, true);
    let kitchen_staff = Employee::new(EmployeeType::KitchenStaff, true);
    let terminated_manager = Employee::new(EmployeeType::Manager, false);

    println!("Garage Access Control System");

    println!("Checking access for a manager:");
    if let Err(e) = manager.print_access_status() {
        println!("{}", e);
    }

    println!("Checking access for an IT employee:");
    if let Err(e) = it_employee.print_access_status() {
        println!("{}", e);
    }

    println!("Checking access for kitchen staff:");
    if let Err(e) = kitchen_staff.print_access_status() {
        println!("{}", e);
    }

    println!("Checking access for a terminated manager:");
    if let Err(e) = terminated_manager.print_access_status() {
        println!("{}", e);
    }
    println!("--- End of Report ---");
}
