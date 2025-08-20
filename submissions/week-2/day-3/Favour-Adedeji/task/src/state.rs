use crate::enums::{Department, Status};

pub struct EmployeeDetails {
    pub name: String,
    pub department: Department,
    pub is_terminated: Status,
}
