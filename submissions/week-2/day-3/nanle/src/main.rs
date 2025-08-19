mod employee;

use employee::{Employee, EmployeeType};

fn main() {
    let manager = Employee::new(EmployeeType::Manager, true);
    let social_media = Employee::new(EmployeeType::SocialMediaTeam, true);
    let terminated = Employee::new(EmployeeType::ITDepartment, false);

    manager.print_access_status();
    social_media.print_access_status();
    terminated.print_access_status();
}