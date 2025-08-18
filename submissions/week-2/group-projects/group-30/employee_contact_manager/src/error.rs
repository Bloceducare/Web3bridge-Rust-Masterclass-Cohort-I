//! Error handling for the Employee Contact Manager

use std::fmt;

/// Custom error types for the employee management system
#[derive(Debug, PartialEq)]
pub enum EmployeeError {
    /// Employee not found error
    NotFound(String),
    /// Invalid input error
    InvalidInput(String),
    /// Operation cancelled by user
    OperationCancelled,
}

impl fmt::Display for EmployeeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmployeeError::NotFound(name) => write!(f, "Employee '{}' not found", name),
            EmployeeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            EmployeeError::OperationCancelled => write!(f, "Operation cancelled by user"),
        }
    }
}

impl std::error::Error for EmployeeError {}

/// Type alias for Results in this crate
pub type Result<T> = std::result::Result<T, EmployeeError>;