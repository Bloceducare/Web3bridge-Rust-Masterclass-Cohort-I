/*
Group 15: Fleet Vehicle Manager
Description: Manage a company's vehicle fleet.
Stage 1:
Add vehicles (model, license plate, status).
View all vehicles.
Stage 2:
Remove retired vehicles.
Stage 3:
Edit vehicle details.
Cancel edits.
Implementation Tips: Use a Vec for Stage 1, then a HashMap with license plate as the key.
*/

use std::io;

use fleet_vehicle_manager::{
    MenuOption, VehiclesRegistry,
    ui::{
        confirm_action, display_menu, display_vehicle, display_vehicles, enter_login_details,
        enter_optional_string, enter_sign_up_details, enter_vehicle_license_plate,
        enter_vehicle_registration_details,
    },
};

fn main() {
    let mut registry = VehiclesRegistry::new();

    loop {
        display_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let parsed_choice = choice.trim().parse::<u8>();
        match parsed_choice
            .ok()
            .and_then(|num| MenuOption::try_from(num).ok())
        {
            Some(MenuOption::SignUp) => {
                let (user_name, user_password) = enter_sign_up_details();
                match registry.sign_up(user_name.clone(), user_password) {
                    Ok(user_name) => println!(
                        "Account created successfully! Your username is: {}",
                        user_name
                    ),
                    Err(e) => println!("Error: {}", e),
                }
            }

            Some(MenuOption::Login) => {
                let (user_name, user_password) = enter_login_details();
                match registry.login(user_name, user_password) {
                    Ok(_user_name) => println!("Login successful!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Some(MenuOption::AddVehicle) => {
                let (model, plate) = enter_vehicle_registration_details();
                match registry.add_vehicle(model, plate) {
                    Ok(id) => println!("Vehicle added with ID: {}", id),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Some(MenuOption::ViewVehicles) => {
                let vehicles = registry.view_vehicles();
                display_vehicles(vehicles);
            }
            Some(MenuOption::ViewVehicle) => {
                let license_plate =
                    enter_vehicle_license_plate("Enter vehicle license plate to view:");
                match registry.view_vehicle(license_plate) {
                    Ok(vehicle) => display_vehicle(vehicle),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Some(MenuOption::RetireVehicle) => {
                let license_plate =
                    enter_vehicle_license_plate("Enter vehicle license plate to retire:");
                if confirm_action("confirm retirement of vehicle? (y/n):") {
                    match registry.retire_vehicle(license_plate) {
                        Ok(_) => println!("Vehicle retired successfully"),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Retirement canceled.");
                }
            }
            Some(MenuOption::RemoveVehicle) => {
                let license_plate =
                    enter_vehicle_license_plate("Enter vehicle license plate to remove:");
                if confirm_action("⚠️  Confirm removal of vehicle? (y/n):") {
                    match registry.remove_vehicle(license_plate) {
                        Ok(v) => println!("Vehicle removed: {}", v.license_plate),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Removal canceled.");
                }
            }
            Some(MenuOption::EditVehicle) => {
                let license_plate =
                    enter_vehicle_license_plate("Enter vehicle license plate to edit:");
                let new_model = enter_optional_string("Enter new model");
                let new_plate = enter_optional_string("Enter new license plate");

                println!("\nYou are about to apply the following changes:");
                if let Some(ref model) = new_model {
                    println!("New Model: {}", model);
                } else {
                    println!("Model unchanged.");
                }

                if let Some(ref plate) = new_plate {
                    println!("New License Plate: {}", plate);
                } else {
                    println!("License plate unchanged.");
                }

                if confirm_action("Save changes? (y/n):") {
                    match registry.edit_vehicle(license_plate, new_model, new_plate) {
                        Ok(_) => println!("Vehicle updated successfully"),
                        Err(e) => println!("Error: {}", e),
                    }
                } else {
                    println!("Edit canceled.");
                }
            }
            Some(MenuOption::Logout) => {
                println!("Logging out program.");
                break;
            }
            None => {
                println!("Invalid input, please enter a number between 1 and 8.");
            }
        }
    }
}
