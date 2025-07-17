use garage_access::*;

fn main() {
    let mut w3b = Web3Bridge::new();


    println!("\n=======================START=====HERE===========================\n");
    w3b.build(BuildingType::EthereumHouse);
    w3b.build(BuildingType::LiskGarage);
    w3b.build(BuildingType::StaffQuaters);

    println!("\n================================================================\n");
    w3b.employ("Muritadhor".to_string(), EmployeeDepartment::IT);
    w3b.employ("Chef Lumzy".to_string(), EmployeeDepartment::Kitchen);
    w3b.employ("Debby".to_string(), EmployeeDepartment::Management);
    w3b.employ("Xcel".to_string(), EmployeeDepartment::Media);
    w3b.employ("Peace".to_string(), EmployeeDepartment::SocialMedia);
    w3b.employ("Joshua".to_string(), EmployeeDepartment::TechnicianSupervisor);
    w3b.employ("Goodness".to_string(), EmployeeDepartment::IT);

    w3b.grant_access_to_building(1, [EmployeeDepartment::Media, EmployeeDepartment::Kitchen, EmployeeDepartment::Management, EmployeeDepartment::SocialMedia].to_vec());
    w3b.grant_access_to_building(2, [EmployeeDepartment::IT, EmployeeDepartment::Management, EmployeeDepartment::Media].to_vec());
    w3b.grant_access_to_building(3, [EmployeeDepartment::Kitchen].to_vec());


    println!("\n================================================================\n");
    let _ = w3b.can_access_building(2, 1);
    _ = w3b.can_access_building(2, 2);
    _ = w3b.can_access_building(2, 3);
    _ = w3b.can_access_building(2, 4);
    _ = w3b.can_access_building(2, 5);
    _ = w3b.can_access_building(2, 6);
    _ = w3b.can_access_building(2, 7);

    println!("\n================================================================\n");
    _ = w3b.terminate_employment(7);

    println!("\n================================================================\n");
    _ = w3b.can_access_building(2, 1);
    _ = w3b.can_access_building(2, 2);
    _ = w3b.can_access_building(2, 3);
    _ = w3b.can_access_building(2, 4);
    _ = w3b.can_access_building(2, 5);
    _ = w3b.can_access_building(2, 6);
    _ = w3b.can_access_building(2, 7);
    println!("\n============================END=================================\n");
}
