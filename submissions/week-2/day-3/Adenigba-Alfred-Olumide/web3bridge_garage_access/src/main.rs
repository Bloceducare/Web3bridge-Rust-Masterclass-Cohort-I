use web3bridge_garage_access::*;

fn main() {
    let mut garage = Garage::new();

    garage.create_employee("Darey".to_string(), EmployeeType::MediaTeam);
    garage.create_employee("Ola".to_string(), EmployeeType::KitchenStaff);
    garage.create_employee("Timidan".to_string(), EmployeeType::Manager);

    garage.change_employee_status(1, Some(EmployeeStatus::Inactive));

    println!("This employee: {:#?}", garage.can_access_garage(2));
    println!("This employee: {:#?}", garage.can_access_garage(1));
    

    println!("Employees: {:#?}", garage.get_employees());
}
