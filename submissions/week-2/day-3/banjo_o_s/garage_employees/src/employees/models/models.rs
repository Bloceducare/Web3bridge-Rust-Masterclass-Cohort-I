use crate::employees::constants::constants::{EmployeeRole, EmployeeStatus, OtherEmployeeRole};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub status: EmployeeStatus,
    pub role: EmployeeRole,
}

pub struct OtherRoles {
    pub id: u128,
    pub role: OtherEmployeeRole,
}

struct Hostel;

pub struct Company {
    _hostel: Hostel,
    _garage: Garage,
}

pub struct Garage {
    pub name: String,
    pub employees: HashMap<u128, Employee>,
    pub other_roles: HashMap<u128, OtherRoles>,
    pub id: u128,
}
