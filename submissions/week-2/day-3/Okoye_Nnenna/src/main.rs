use okoye_nnenna::employees::{Employee, EmployeeType};
use okoye_nnenna::garage_logic::check_access;

fn main() {
    let manager = Employee::new(EmployeeType::Manager, true);
    let kitchen_staff = Employee::new(EmployeeType::Kitchen, true);
    let terminated_manager = Employee::new(EmployeeType::Manager, false);

    println!("Checking access for Manager:");
    match check_access(&manager) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("\nChecking access for Kitchen Staff:");
    match check_access(&kitchen_staff) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("\nChecking access for Terminated Manager:");
    match check_access(&terminated_manager) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
