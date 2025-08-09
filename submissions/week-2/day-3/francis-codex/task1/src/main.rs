pub mod lib;

use crate::lib::*;

fn main() {
    let employee = Employee {
        position: Employees::IT,
        is_employed: true,
    };

    match print_access_result(&employee) {
        Ok(_) => println!("Employee access check completed successfully."),
        Err(e) => println!("{}", e),
    }

     let employee1 = Employee {
        position: Employees::KitchenStaff,
        is_employed: true,
    };

    match print_access_result(&employee1) {
        Ok(_) => println!("Employee access check completed successfully."),
        Err(e) => println!("{}", e),
    }
}