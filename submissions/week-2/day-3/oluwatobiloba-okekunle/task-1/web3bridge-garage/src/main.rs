#![allow(unused)]

use web3bridge_garage::{Garage, GarageError, StaffType, StaffStatus};

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


    // can staff access garage
    let can_access = garage.can_staff_access_garage(1);
    // println!("Can access: {:?}", can_access);
    
    
}
