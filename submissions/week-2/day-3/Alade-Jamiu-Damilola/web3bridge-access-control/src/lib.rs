pub mod employee;
pub mod access;


pub use employee::{Employee, EmployeeType};
pub use access::{check_access, print_access, AccessError};