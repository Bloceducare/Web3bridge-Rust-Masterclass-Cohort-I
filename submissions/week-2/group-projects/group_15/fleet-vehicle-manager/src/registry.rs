use std::{collections::HashMap, time::SystemTime};

use crate::models::{UserProfile, Vehicle, VehicleStatus};

pub struct VehiclesRegistry {
    users: HashMap<String, UserProfile>,
    logged_in: Option<String>,
    fleet_vehicles: Vec<Vehicle>,
    fleet_vehicles_license_plate: HashMap<String, Vehicle>,
    next_user_id: u32,
    next_vehicle_id: u32,
}

impl VehiclesRegistry {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            logged_in: None,
            fleet_vehicles: Vec::new(),
            fleet_vehicles_license_plate: HashMap::new(),
            next_user_id: 1,
            next_vehicle_id: 1,
        }
    }

    pub fn sign_up(&mut self, user_name: String, password: String) -> Result<String, String> {
        if self.users.contains_key(&user_name) {
            return Err("User name already exists".to_string());
        }
        let user = UserProfile {
            id: self.next_user_id,
            user_name: user_name.clone(),
            password,
            created_at: SystemTime::now(),
        };

        self.users.insert(user_name.clone(), user);
        self.next_user_id += 1;

        Ok(user_name)
    }

    pub fn login(&mut self, user_name: String, password: String) -> Result<(), String> {
        match self.users.get(&user_name) {
            Some(user) if user.password == password => {
                self.logged_in = Some(user_name.clone());
                Ok(())
            }
            _ => Err("Invalid username or password.".to_string()),
        }
    }

    pub fn add_vehicle(&mut self, model: String, license_plate: String) -> Result<u32, String> {
        if let Some(user_id) = &self.logged_in {
            if let Some(_user) = self.users.get_mut(user_id) {
                if self
                    .fleet_vehicles_license_plate
                    .contains_key(&license_plate)
                {
                    return Err("Vehicle already exists".to_string());
                }

                let new_vehicle_id = self.next_vehicle_id;
                let new_vehicle = Vehicle {
                    id: new_vehicle_id,
                    model,
                    license_plate: license_plate.clone(),
                    status: VehicleStatus::Registered,
                    registered_at: SystemTime::now(),
                };

                self.fleet_vehicles.push(new_vehicle.clone());
                self.fleet_vehicles_license_plate
                    .insert(license_plate, new_vehicle);
                self.next_vehicle_id += 1;

                return Ok(new_vehicle_id);
            } else {
                return Err("User not found.".to_string());
            }
        }

        Err("You must log in first.".to_string())
    }

    pub fn view_vehicles(&self) -> &[Vehicle] {
        // view all vehicles
        &self.fleet_vehicles
    }

    pub fn view_vehicle(&self, license_plate: String) -> Result<&Vehicle, String> {
        // view a vehicle by license plate
        self.fleet_vehicles_license_plate
            .get(&license_plate)
            .ok_or_else(|| format!("Vehicle with license plate {} not found", license_plate))
    }

    pub fn retire_vehicle(&mut self, license_plate: String) -> Result<(), String> {
        if let Some(user_id) = &self.logged_in {
            if let Some(_user) = self.users.get_mut(user_id) {
                if let Some(vehicle) = self.fleet_vehicles_license_plate.get_mut(&license_plate) {
                    match vehicle.status {
                        VehicleStatus::Registered => {
                            vehicle.status = VehicleStatus::Retired;
                            // Update the vehicle in the Vec as well
                            if let Some(vec_vehicle) = self
                                .fleet_vehicles
                                .iter_mut()
                                .find(|v| v.license_plate == license_plate)
                            {
                                vec_vehicle.status = VehicleStatus::Retired;
                            }
                            Ok(())
                        }
                        VehicleStatus::Retired => Err("Vehicle is already retired".to_string()),
                    }
                } else {
                    Err(format!(
                        "Vehicle with license plate {} not found",
                        license_plate
                    ))
                }
            } else {
                Err("User not found.".to_string())
            }
        } else {
            Err("You must log in first.".to_string())
        }
    }

    pub fn remove_vehicle(&mut self, license_plate: String) -> Result<Vehicle, String> {
        if let Some(user_id) = &self.logged_in {
            if let Some(_user) = self.users.get_mut(user_id) {
                match self.fleet_vehicles_license_plate.get(&license_plate) {
                    Some(vehicle) => match vehicle.status {
                        VehicleStatus::Retired => {
                            self.fleet_vehicles
                                .retain(|v| v.license_plate != license_plate);
                            self.fleet_vehicles_license_plate
                                .remove(&license_plate)
                                .ok_or_else(|| "Failed to remove vehicle".to_string())
                        }
                        _ => Err("Vehicle must be retired before removal".to_string()),
                    },
                    None => Err(format!(
                        "Vehicle with license plate {} not found",
                        license_plate
                    )),
                }
            } else {
                return Err("User not found.".to_string());
            }
        } else {
            return Err("You must log in first.".to_string());
        }
    }

    pub fn edit_vehicle(
        &mut self,
        license_plate: String,
        new_model: Option<String>,
        new_license_plate: Option<String>,
    ) -> Result<(), String> {
        if let Some(user_id) = &self.logged_in {
            if let Some(_user) = self.users.get_mut(user_id) {
                // Check if vehicle exists
                if !self
                    .fleet_vehicles_license_plate
                    .contains_key(&license_plate)
                {
                    return Err(format!(
                        "Vehicle with license plate {} not found",
                        license_plate
                    ));
                }

                // Check if new license plate already exists (if provided)
                if let Some(ref new_plate) = new_license_plate {
                    if self.fleet_vehicles_license_plate.contains_key(new_plate)
                        && new_plate != &license_plate
                    {
                        return Err(format!("License plate {} already in use", new_plate));
                    }
                }

                // Get the vehicle from HashMap
                let mut vehicle = self
                    .fleet_vehicles_license_plate
                    .remove(&license_plate)
                    .expect("Vehicle should exist after check");

                // Update fields
                if let Some(new_model) = new_model {
                    vehicle.model = new_model;
                }

                let old_license_plate = vehicle.license_plate.clone();
                if let Some(new_plate) = new_license_plate {
                    vehicle.license_plate = new_plate.clone();
                }

                // Update in Vec
                if let Some(vec_vehicle) = self
                    .fleet_vehicles
                    .iter_mut()
                    .find(|v| v.license_plate == old_license_plate)
                {
                    vec_vehicle.model = vehicle.model.clone();
                    vec_vehicle.license_plate = vehicle.license_plate.clone();
                }

                // Re-insert into HashMap with new license plate as key
                self.fleet_vehicles_license_plate
                    .insert(vehicle.license_plate.clone(), vehicle);

                Ok(())
            } else {
                return Err("User not found.".to_string());
            }
        } else {
            // Return error if user is not logged in
            return Err("You must log in first.".to_string());
        }
    }

    pub fn logout(&mut self) {
        self.logged_in = None;
        println!("Logout successful!");
    }
}
