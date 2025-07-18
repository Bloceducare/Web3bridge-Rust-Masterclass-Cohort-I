use web3bridge::{enums, state::EmployeeDetails, can_access_building};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let employee1 = EmployeeDetails {
        name: "Alice".to_string(),
        department: enums::Department::MEDIA,
        is_terminated: enums::Status::ACTIVE,
    };

    let employee2 = EmployeeDetails {
        name: "Bob".to_string(),
        department: enums::Department::SOCIAL_MEDIA,
        is_terminated: enums::Status::TERMINATED,
    };

    let employee3 = EmployeeDetails {
        name: "Rick".to_string(),
        department: enums::Department::MANAGER,
        is_terminated: enums::Status::TERMINATED,
    };

    let res = can_access_building(&employee1).unwrap();
    println!("Access for {}: {}", employee1.name, res);


    let res2 = can_access_building(&employee2)?;
    println!("Access for {}: {}", employee2.name, res2);

    Ok(())
}
