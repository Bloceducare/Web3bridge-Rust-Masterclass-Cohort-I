use access_management::*;
use access_management::state::*;

fn main() {
    let test = EmployeeType::ITdept;
    let mut new_access1 = EmployeeDetails {
        employee_id: 1,
        employee_name: String::from("Gbolahan"),
        employee_type: EmployeeType::MediaTeam,
        is_employed: EmploymentStatus::Employed
    };

    let mut new_access2 = EmployeeDetails {
        employee_id: 1,
        employee_name: String::from("Gbolahan"),
        employee_type: EmployeeType::ITdept,
        is_employed: EmploymentStatus::NotEmployed
    };
    let access1 = print_access(&new_access1);
    println!("{:#?}", access1);
    let access2 = print_access(&new_access2);
    println!("{:#?}", access2);
}
