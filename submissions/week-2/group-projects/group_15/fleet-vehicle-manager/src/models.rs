use std::time::SystemTime;

#[derive(Debug, PartialEq)]
pub enum MenuOption {
    SignUp,
    Login,
    AddVehicle,
    ViewVehicles,
    ViewVehicle,
    RetireVehicle,
    RemoveVehicle,
    EditVehicle,
    Logout,
}

impl TryFrom<u8> for MenuOption {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MenuOption::SignUp),
            2 => Ok(MenuOption::Login),
            3 => Ok(MenuOption::AddVehicle),
            4 => Ok(MenuOption::ViewVehicles),
            5 => Ok(MenuOption::ViewVehicle),
            6 => Ok(MenuOption::RetireVehicle),
            7 => Ok(MenuOption::RemoveVehicle),
            8 => Ok(MenuOption::EditVehicle),
            9 => Ok(MenuOption::Logout),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum VehicleStatus {
    Registered,
    Retired,
}

#[derive(Debug, Clone)]
pub struct UserProfile {
    pub id: u32,
    pub user_name: String,
    pub password: String,
    pub created_at: SystemTime,
}

impl UserProfile {
    pub fn new() -> Self {
        UserProfile {
            id: 0,
            user_name: String::new(),
            password: String::new(),
            created_at: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub model: String,
    pub license_plate: String,
    pub status: VehicleStatus,
    pub registered_at: SystemTime,
}
