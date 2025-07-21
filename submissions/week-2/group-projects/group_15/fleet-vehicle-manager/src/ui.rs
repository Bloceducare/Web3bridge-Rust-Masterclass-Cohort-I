use std::io::{self, Write};

pub fn enter_sign_up_details() -> (String, String) {
    println!("Enter user name: ");
    io::stdout().flush().unwrap();
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();

    println!("Enter user password: ");
    io::stdout().flush().unwrap();
    let mut user_password = String::new();
    io::stdin().read_line(&mut user_password).unwrap();

    (
        user_name.trim().to_string(),
        user_password.trim().to_string(),
    )
}

pub fn enter_login_details() -> (String, String) {
    println!("Enter user name: ");
    io::stdout().flush().unwrap();
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).unwrap();

    println!("Enter user password: ");
    io::stdout().flush().unwrap();
    let mut user_password = String::new();
    io::stdin().read_line(&mut user_password).unwrap();

    (
        user_name.trim().to_string(),
        user_password.trim().to_string(),
    )
}

pub fn enter_vehicle_registration_details() -> (String, String) {
    println!("Enter vehicle model: ");
    io::stdout().flush().unwrap();
    let mut model = String::new();
    io::stdin().read_line(&mut model).unwrap();

    println!("Enter vehicle license plate: ");
    io::stdout().flush().unwrap();
    let mut plate = String::new();
    io::stdin().read_line(&mut plate).unwrap();

    (model.trim().to_string(), plate.trim().to_string())
}

pub fn enter_vehicle_license_plate(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut license_plate = String::new();
    io::stdin().read_line(&mut license_plate).unwrap();

    license_plate.trim().to_string()
}

pub fn enter_optional_string(prompt: &str) -> Option<String> {
    println!("{} (or press ENTER to skip):", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn confirm_action(prompt: &str) -> bool {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).unwrap();
    matches!(confirmation.trim().to_lowercase().as_str(), "y" | "yes")
}

pub fn display_menu() {
    println!("\n******* WELCOME TO YOUR SMART VEHICLES REGISTRY *******");
    println!("1. Please create your user name");
    println!("2. Login");
    println!("3. Add Vehicle");
    println!("4. View Vehicles");
    println!("5. Retire Vehicle");
    println!("6. Remove Vehicle");
    println!("7. Edit Vehicle");
    println!("8. View Vehicle");
    println!("9. Logout");
    println!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

pub fn display_vehicles(vehicles: &[crate::models::Vehicle]) {
    if vehicles.is_empty() {
        println!("ℹ️  No vehicles registered yet.");
    } else {
        println!("📋 Registered Vehicles:");
        for v in vehicles {
            println!(
                "ID: {}, Model: {}, Plate: {}, Status: {:?}",
                v.id, v.model, v.license_plate, v.status
            );
        }
    }
}

pub fn display_vehicle(vehicle: &crate::models::Vehicle) {
    println!(
        "ID: {}, Model: {}, Plate: {}, Status: {:?}",
        vehicle.id, vehicle.model, vehicle.license_plate, vehicle.status
    );
}
