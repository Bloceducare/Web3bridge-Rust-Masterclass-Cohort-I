use omemgboji_emmanuel::enums::EmployeeRoles;
use omemgboji_emmanuel::structs::Employee;

fn main() {
    let manager = Employee {
        role: EmployeeRoles::Manager,
        is_employed: true,
    };

    let kitchen_staff = Employee {
        role: EmployeeRoles::KitchenStaff,
        is_employed: true,
    };

    let previous_worker = Employee {
        role: EmployeeRoles::MediaTeam,
        is_employed: false,
    };

    println!("---- ####### Access Control System for Web3Bridge Garage ####### ----");

    match manager.access_garage() {
        Ok(_) => println!("Manager has access to the garage."),
        Err(e) => println!("Manager: {}", e),
    };

    match kitchen_staff.access_garage() {
        Ok(_) => println!("Kitchen Staff has access to the garage."),
        Err(e) => println!("Kitchen Staff: {}", e),
    };

    match previous_worker.access_garage() {
        Ok(_) => println!("Previous Worker has access to the garage."),
        Err(e) => println!("Previous Worker: {}", e),
    };
}
