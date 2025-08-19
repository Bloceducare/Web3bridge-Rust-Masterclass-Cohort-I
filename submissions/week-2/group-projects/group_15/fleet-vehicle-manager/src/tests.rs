#[cfg(test)]
use crate::{MenuOption, VehicleStatus, VehiclesRegistry};

#[test]
fn test_new_registry() {
    let registry = VehiclesRegistry::new();
    let vehicles = registry.view_vehicles();
    assert!(vehicles.is_empty());
}

#[test]
fn test_sign_up_success() {
    let mut registry = VehiclesRegistry::new();
    let result = registry.sign_up("testuser".to_string(), "password123".to_string());

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "testuser");
}

#[test]
fn test_sign_up_duplicate_username() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.sign_up("testuser".to_string(), "different_password".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "User name already exists");
}

#[test]
fn test_login_success() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.login("testuser".to_string(), "password123".to_string());
    assert!(result.is_ok());
}

#[test]
fn test_login_invalid_credentials() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.login("testuser".to_string(), "wrongpassword".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid username or password.");
}

#[test]
fn test_login_nonexistent_user() {
    let mut registry = VehiclesRegistry::new();

    let result = registry.login("nonexistent".to_string(), "password123".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid username or password.");
}

#[test]
fn test_add_vehicle_not_logged_in() {
    let mut registry = VehiclesRegistry::new();

    let result = registry.add_vehicle("Toyota Camry".to_string(), "ABC123".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "You must log in first.");
}

#[test]
fn test_add_vehicle_success() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.add_vehicle("Toyota Camry".to_string(), "ABC123".to_string());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);

    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 1);
    assert_eq!(vehicles[0].model, "Toyota Camry");
    assert_eq!(vehicles[0].license_plate, "ABC123");
    assert!(matches!(vehicles[0].status, VehicleStatus::Registered));
    assert_eq!(vehicles[0].id, 1);
}

#[test]
fn test_add_vehicle_duplicate_license_plate() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.add_vehicle("Honda Civic".to_string(), "ABC123".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Vehicle already exists");

    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 1); // Should still only have one vehicle
}

#[test]
fn test_view_vehicles_empty() {
    let registry = VehiclesRegistry::new();
    let vehicles = registry.view_vehicles();
    assert!(vehicles.is_empty());
}

#[test]
fn test_view_vehicles_with_vehicles() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();
    registry
        .add_vehicle("Honda Civic".to_string(), "XYZ789".to_string())
        .unwrap();

    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 2);
    assert_eq!(vehicles[0].license_plate, "ABC123");
    assert_eq!(vehicles[1].license_plate, "XYZ789");
}

#[test]
fn test_view_vehicle_success() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.view_vehicle("ABC123".to_string());
    assert!(result.is_ok());
    let vehicle = result.unwrap();
    assert_eq!(vehicle.model, "Toyota Camry");
    assert_eq!(vehicle.license_plate, "ABC123");
}

#[test]
fn test_view_vehicle_not_found() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.view_vehicle("NONEXISTENT".to_string());
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Vehicle with license plate NONEXISTENT not found"
    );
}

#[test]
fn test_retire_vehicle_success() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.retire_vehicle("ABC123".to_string());
    assert!(result.is_ok());

    let vehicle = registry.view_vehicle("ABC123".to_string()).unwrap();
    assert!(matches!(vehicle.status, VehicleStatus::Retired));
}

#[test]
fn test_retire_vehicle_already_retired() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();
    registry.retire_vehicle("ABC123".to_string()).unwrap();

    let result = registry.retire_vehicle("ABC123".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Vehicle is already retired");
}

#[test]
fn test_retire_vehicle_not_found() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.retire_vehicle("NONEXISTENT".to_string());
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Vehicle with license plate NONEXISTENT not found"
    );
}

#[test]
fn test_retire_vehicle_not_logged_in() {
    let mut registry = VehiclesRegistry::new();

    let result = registry.retire_vehicle("ABC123".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "You must log in first.");
}

#[test]
fn test_remove_vehicle_success() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();
    registry.retire_vehicle("ABC123".to_string()).unwrap();

    let result = registry.remove_vehicle("ABC123".to_string());
    assert!(result.is_ok());
    let removed_vehicle = result.unwrap();
    assert_eq!(removed_vehicle.license_plate, "ABC123");

    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 0);
}

#[test]
fn test_remove_vehicle_not_retired() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.remove_vehicle("ABC123".to_string());
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Vehicle must be retired before removal"
    );

    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 1); // Vehicle should still exist
}

#[test]
fn test_remove_vehicle_not_found() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.remove_vehicle("NONEXISTENT".to_string());
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Vehicle with license plate NONEXISTENT not found"
    );
}

#[test]
fn test_edit_vehicle_success() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.edit_vehicle(
        "ABC123".to_string(),
        Some("Toyota Corolla".to_string()),
        Some("XYZ789".to_string()),
    );
    assert!(result.is_ok());

    let vehicle = registry.view_vehicle("XYZ789".to_string()).unwrap();
    assert_eq!(vehicle.model, "Toyota Corolla");
    assert_eq!(vehicle.license_plate, "XYZ789");

    // Old license plate should not exist
    let old_vehicle_result = registry.view_vehicle("ABC123".to_string());
    assert!(old_vehicle_result.is_err());
}

#[test]
fn test_edit_vehicle_model_only() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.edit_vehicle(
        "ABC123".to_string(),
        Some("Toyota Corolla".to_string()),
        None,
    );
    assert!(result.is_ok());

    let vehicle = registry.view_vehicle("ABC123".to_string()).unwrap();
    assert_eq!(vehicle.model, "Toyota Corolla");
    assert_eq!(vehicle.license_plate, "ABC123"); // Should remain unchanged
}

#[test]
fn test_edit_vehicle_license_plate_only() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();

    let result = registry.edit_vehicle("ABC123".to_string(), None, Some("XYZ789".to_string()));
    assert!(result.is_ok());

    let vehicle = registry.view_vehicle("XYZ789".to_string()).unwrap();
    assert_eq!(vehicle.model, "Toyota Camry"); // Should remain unchanged
    assert_eq!(vehicle.license_plate, "XYZ789");
}

#[test]
fn test_edit_vehicle_duplicate_license_plate() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();
    registry
        .add_vehicle("Honda Civic".to_string(), "XYZ789".to_string())
        .unwrap();

    let result = registry.edit_vehicle("ABC123".to_string(), None, Some("XYZ789".to_string()));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "License plate XYZ789 already in use");
}

#[test]
fn test_edit_vehicle_not_found() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    let result = registry.edit_vehicle(
        "NONEXISTENT".to_string(),
        Some("Toyota Corolla".to_string()),
        None,
    );
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Vehicle with license plate NONEXISTENT not found"
    );
}

#[test]
fn test_edit_vehicle_not_logged_in() {
    let mut registry = VehiclesRegistry::new();

    let result = registry.edit_vehicle(
        "ABC123".to_string(),
        Some("Toyota Corolla".to_string()),
        None,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "You must log in first.");
}

#[test]
fn test_logout() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    registry.logout();

    // Try to add a vehicle after logout - should fail
    let result = registry.add_vehicle("Toyota Camry".to_string(), "ABC123".to_string());
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "You must log in first.");
}

#[test]
fn test_menu_option_try_from() {
    assert_eq!(MenuOption::try_from(1), Ok(MenuOption::SignUp));
    assert_eq!(MenuOption::try_from(2), Ok(MenuOption::Login));
    assert_eq!(MenuOption::try_from(3), Ok(MenuOption::AddVehicle));
    assert_eq!(MenuOption::try_from(4), Ok(MenuOption::ViewVehicles));
    assert_eq!(MenuOption::try_from(5), Ok(MenuOption::ViewVehicle));
    assert_eq!(MenuOption::try_from(6), Ok(MenuOption::RetireVehicle));
    assert_eq!(MenuOption::try_from(7), Ok(MenuOption::RemoveVehicle));
    assert_eq!(MenuOption::try_from(8), Ok(MenuOption::EditVehicle));
    assert_eq!(MenuOption::try_from(9), Ok(MenuOption::Logout));
    assert_eq!(MenuOption::try_from(0), Err(()));
    assert_eq!(MenuOption::try_from(10), Err(()));
}

#[test]
fn test_vehicle_status() {
    let registered = VehicleStatus::Registered;
    let retired = VehicleStatus::Retired;

    assert!(matches!(registered, VehicleStatus::Registered));
    assert!(matches!(retired, VehicleStatus::Retired));
}

#[test]
fn test_multiple_users_and_vehicles() {
    let mut registry = VehiclesRegistry::new();

    // Create multiple users
    registry
        .sign_up("user1".to_string(), "pass1".to_string())
        .unwrap();
    registry
        .sign_up("user2".to_string(), "pass2".to_string())
        .unwrap();

    // Login as user1 and add vehicles
    registry
        .login("user1".to_string(), "pass1".to_string())
        .unwrap();
    registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();
    registry
        .add_vehicle("Honda Civic".to_string(), "XYZ789".to_string())
        .unwrap();

    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 2);

    // Logout and login as user2
    registry.logout();
    registry
        .login("user2".to_string(), "pass2".to_string())
        .unwrap();

    // User2 should see the same vehicles (shared fleet)
    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 2);
}

#[test]
fn test_complete_vehicle_lifecycle() {
    let mut registry = VehiclesRegistry::new();
    registry
        .sign_up("testuser".to_string(), "password123".to_string())
        .unwrap();
    registry
        .login("testuser".to_string(), "password123".to_string())
        .unwrap();

    // 1. Add vehicle
    let vehicle_id = registry
        .add_vehicle("Toyota Camry".to_string(), "ABC123".to_string())
        .unwrap();
    assert_eq!(vehicle_id, 1);

    // 2. View vehicle
    let vehicle = registry.view_vehicle("ABC123".to_string()).unwrap();
    assert_eq!(vehicle.model, "Toyota Camry");
    assert!(matches!(vehicle.status, VehicleStatus::Registered));

    // 3. Edit vehicle
    registry
        .edit_vehicle(
            "ABC123".to_string(),
            Some("Toyota Corolla".to_string()),
            None,
        )
        .unwrap();
    let updated_vehicle = registry.view_vehicle("ABC123".to_string()).unwrap();
    assert_eq!(updated_vehicle.model, "Toyota Corolla");

    // 4. Retire vehicle
    registry.retire_vehicle("ABC123".to_string()).unwrap();
    let retired_vehicle = registry.view_vehicle("ABC123".to_string()).unwrap();
    assert!(matches!(retired_vehicle.status, VehicleStatus::Retired));

    // 5. Remove vehicle
    let removed_vehicle = registry.remove_vehicle("ABC123".to_string()).unwrap();
    assert_eq!(removed_vehicle.license_plate, "ABC123");

    // 6. Verify vehicle is gone
    let vehicles = registry.view_vehicles();
    assert_eq!(vehicles.len(), 0);

    let result = registry.view_vehicle("ABC123".to_string());
    assert!(result.is_err());
}
