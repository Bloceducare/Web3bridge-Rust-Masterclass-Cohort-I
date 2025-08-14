//! Employee Contact Manager
//!
//! A simple employee contact management system that allows you to:
//! - Add employees with name, department, and phone information
//! - View all employees
//! - Remove employees by name
//! - Edit employee details with confirmation

pub mod employee;
pub mod error;
pub mod manager;
pub mod ui;

pub use employee::Employee;
pub use error::{EmployeeError, Result};
pub use manager::EmployeeManager;
pub use ui::UserInterface;