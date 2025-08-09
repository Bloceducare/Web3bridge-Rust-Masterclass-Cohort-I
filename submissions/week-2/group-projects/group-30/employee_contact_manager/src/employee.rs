//! Employee data structure and related functionality

use crate::error::{EmployeeError, Result};
use std::fmt;

/// Represents an employee with contact information
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    /// Employee's full name
    pub name: String,
    /// Employee's department
    pub department: String,
    /// Employee's phone number
    pub phone: String,
}

impl Employee {
    /// Creates a new Employee instance
    ///
    /// # Arguments
    ///
    /// * `name` - The employee's full name
    /// * `department` - The employee's department
    /// * `phone` - The employee's phone number
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `Employee` or an `EmployeeError` if validation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_contact_manager::Employee;
    ///
    /// let employee = Employee::new(
    ///     "John Doe".to_string(),
    ///     "Engineering".to_string(),
    ///     "555-1234".to_string(),
    /// ).unwrap();
    /// assert_eq!(employee.name, "John Doe");
    /// ```
    pub fn new(name: String, department: String, phone: String) -> Result<Self> {
        Self::validate_input(&name, &department, &phone)?;
        
        Ok(Employee {
            name: name.trim().to_string(),
            department: department.trim().to_string(),
            phone: phone.trim().to_string(),
        })
    }

    /// Validates employee input data
    fn validate_input(name: &str, department: &str, phone: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(EmployeeError::InvalidInput("Name cannot be empty".to_string()));
        }
        
        if department.trim().is_empty() {
            return Err(EmployeeError::InvalidInput("Department cannot be empty".to_string()));
        }
        
        if phone.trim().is_empty() {
            return Err(EmployeeError::InvalidInput("Phone cannot be empty".to_string()));
        }
        
        Ok(())
    }

    /// Updates the employee's department
    ///
    /// # Arguments
    ///
    /// * `new_department` - The new department name
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure
    pub fn update_department(&mut self, new_department: String) -> Result<()> {
        if new_department.trim().is_empty() {
            return Err(EmployeeError::InvalidInput("Department cannot be empty".to_string()));
        }
        
        self.department = new_department.trim().to_string();
        Ok(())
    }

    /// Updates the employee's phone number
    ///
    /// # Arguments
    ///
    /// * `new_phone` - The new phone number
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or failure
    pub fn update_phone(&mut self, new_phone: String) -> Result<()> {
        if new_phone.trim().is_empty() {
            return Err(EmployeeError::InvalidInput("Phone cannot be empty".to_string()));
        }
        
        self.phone = new_phone.trim().to_string();
        Ok(())
    }

    /// Returns the employee's name (used as unique identifier)
    pub fn id(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nDepartment: {}\nPhone: {}",
            self.name, self.department, self.phone
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_creation_valid() {
        let employee = Employee::new(
            "John Doe".to_string(),
            "Engineering".to_string(),
            "555-1234".to_string(),
        ).unwrap();
        
        assert_eq!(employee.name, "John Doe");
        assert_eq!(employee.department, "Engineering");
        assert_eq!(employee.phone, "555-1234");
    }

    #[test]
    fn test_employee_creation_empty_name() {
        let result = Employee::new(
            "".to_string(),
            "Engineering".to_string(),
            "555-1234".to_string(),
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EmployeeError::InvalidInput("Name cannot be empty".to_string()));
    }

    #[test]
    fn test_employee_creation_empty_department() {
        let result = Employee::new(
            "John Doe".to_string(),
            "".to_string(),
            "555-1234".to_string(),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_employee_creation_empty_phone() {
        let result = Employee::new(
            "John Doe".to_string(),
            "Engineering".to_string(),
            "".to_string(),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_employee_creation_with_whitespace() {
        let employee = Employee::new(
            "  John Doe  ".to_string(),
            "  Engineering  ".to_string(),
            "  555-1234  ".to_string(),
        ).unwrap();
        
        assert_eq!(employee.name, "John Doe");
        assert_eq!(employee.department, "Engineering");
        assert_eq!(employee.phone, "555-1234");
    }

    #[test]
    fn test_update_department() {
        let mut employee = Employee::new(
            "John Doe".to_string(),
            "Engineering".to_string(),
            "555-1234".to_string(),
        ).unwrap();
        
        employee.update_department("Marketing".to_string()).unwrap();
        assert_eq!(employee.department, "Marketing");
    }

    #[test]
    fn test_update_phone() {
        let mut employee = Employee::new(
            "John Doe".to_string(),
            "Engineering".to_string(),
            "555-1234".to_string(),
        ).unwrap();
        
        employee.update_phone("555-9999".to_string()).unwrap();
        assert_eq!(employee.phone, "555-9999");
    }

    #[test]
    fn test_employee_display() {
        let employee = Employee::new(
            "John Doe".to_string(),
            "Engineering".to_string(),
            "555-1234".to_string(),
        ).unwrap();
        
        let display = format!("{}", employee);
        assert!(display.contains("John Doe"));
        assert!(display.contains("Engineering"));
        assert!(display.contains("555-1234"));
    }

    #[test]
    fn test_employee_id() {
        let employee = Employee::new(
            "John Doe".to_string(),
            "Engineering".to_string(),
            "555-1234".to_string(),
        ).unwrap();
        
        assert_eq!(employee.id(), "John Doe");
    }
}