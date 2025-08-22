#![allow(unused)]

use web3bridge_garage::{Garage, GarageError, StaffStatus, StaffType};

fn try_access_garage(garage: &Garage, id: u32) -> Result<(), GarageError> {
    if garage.can_staff_access_garage(id)? {
        println!("Staff with id {} can access the garage.", id);
    } else {
        println!("Staff with id {} cannot access the garage.", id);
    }
    Ok(())
}
fn main() {
    let mut garage = Garage::new();
    garage.add_staff(String::from("John"), StaffType::Manager);
    garage.add_staff(String::from("Jane"), StaffType::ITTeam);
    garage.add_staff(String::from("Akin"), StaffType::MediaTeam);
    garage.add_staff(String::from("Jim"), StaffType::TechinalSupervisor);
    garage.add_staff(String::from("Becky"), StaffType::SocialMediaTeam);
    garage.add_staff(String::from("Ade"), StaffType::MarketingTeam);
    garage.add_staff(String::from("Ayo"), StaffType::SalesTeam);
    garage.add_staff(String::from("Tobi"), StaffType::CustomerServiceTeam);
    garage.add_staff(String::from("Josh"), StaffType::KitchenStaff);

    println!("All staff: {:?}", garage.get_all_staff());

    match try_access_garage(&garage, 1) {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    };

    match try_access_garage(&garage, 2) {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    };

    match try_access_garage(&garage, 4) {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    };

    match try_access_garage(&garage, 5) {
        Ok(()) => (),
        Err(e) => println!("Error: {:?}", e),
    };
    
}
