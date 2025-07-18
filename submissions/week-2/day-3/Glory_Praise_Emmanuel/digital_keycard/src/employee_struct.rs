use crate::employee_enum::{EmployeeType, EmploymentStatus};

#[derive(Debug)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub dept: EmployeeType,
    pub status: EmploymentStatus,
    
}

pub struct EmployeeDB {
    pub data: Vec<Employee>,
    pub next_id: u32,
}