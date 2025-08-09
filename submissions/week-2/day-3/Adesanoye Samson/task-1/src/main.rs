mod lib;

use crate::lib::{Garage, EmployeeType};
fn main() {
    let mut garage = Garage::new();

    garage.create_employee(
        "Alice".to_string(),
        EmployeeType::MEDIATEAM
    );

    println!("{}", garage.employees.len())
}
