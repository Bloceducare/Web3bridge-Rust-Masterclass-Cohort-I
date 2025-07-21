//! Employee management functionality

use crate::employee::Employee;
use crate::error::{EmployeeError, Result};
use std::collections::HashMap;

/// Manages a collection of employees
#[derive(Debug)]
pub struct EmployeeManager {
    employees: HashMap<String, Employee>,
}

impl EmployeeManager {
    /// Creates a new empty EmployeeManager
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_contact_manager::EmployeeManager;
    ///
    /// let manager = EmployeeManager::new();
    /// assert_eq!(manager.employee_count(), 0);
    /// ```
    pub fn new() -> Self {
        EmployeeManager {
            employees: HashMap::new(),
        }
    }

    /// Adds an employee to the manager
    ///
    /// If an employee with the same name already exists, it will be replaced.
    ///
    /// # Arguments
    ///
    /// * `employee` - The employee to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the operation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_contact_manager::{EmployeeManager, Employee};
    ///
    /// let mut manager = EmployeeManager::new();
    /// let employee = Employee::new(
    ///     "John Doe".to_string(),
    ///     "Engineering".to_string(),
    ///     "555-1234".to_string(),
    /// ).unwrap();
    ///
    /// manager.add_employee(employee).unwrap();
    /// assert_eq!(manager.employee_count(), 1);
    /// ```
    pub fn add_employee(&mut self, employee: Employee) -> Result<()> {
        let name = employee.id().to_string();
        self.employees.insert(name, employee);
        Ok(())
    }

    /// Gets an employee by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the employee to retrieve
    ///
    /// # Returns
    ///
    /// Returns a reference to the employee if found, or an error if not found
    pub fn get_employee(&self, name: &str) -> Result<&Employee> {
        self.employees
            .get(name)
            .ok_or_else(|| EmployeeError::NotFound(name.to_string()))
    }

    /// Gets a mutable reference to an employee by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the employee to retrieve
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to the employee if found, or an error if not found
    pub fn get_employee_mut(&mut self, name: &str) -> Result<&mut Employee> {
        self.employees
            .get_mut(name)
            .ok_or_else(|| EmployeeError::NotFound(name.to_string()))
    }

    /// Removes an employee by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the employee to remove
    ///
    /// # Returns
    ///
    /// Returns the removed employee on success, or an error if the employee was not found
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_contact_manager::{EmployeeManager, Employee};
    ///
    /// let mut manager = EmployeeManager::new();
    /// let employee = Employee::new(
    ///     "John Doe".to_string(),
    ///     "Engineering".to_string(),
    ///     "555-1234".to_string(),
    /// ).unwrap();
    ///
    /// manager.add_employee(employee).unwrap();
    /// let removed = manager.remove_employee("John Doe").unwrap();
    /// assert_eq!(removed.name, "John Doe");
    /// assert_eq!(manager.employee_count(), 0);
    /// ```
    pub fn remove_employee(&mut self, name: &str) -> Result<Employee> {
        self.employees
            .remove(name)
            .ok_or_else(|| EmployeeError::NotFound(name.to_string()))
    }

    /// Returns an iterator over all employees
    ///
    /// # Returns
    ///
    /// An iterator that yields references to all employees
    pub fn employees(&self) -> impl Iterator<Item = &Employee> {
        self.employees.values()
    }

    /// Returns the number of employees
    ///
    /// # Returns
    ///
    /// The total number of employees managed
    pub fn employee_count(&self) -> usize {
        self.employees.len()
    }

    /// Checks if the manager is empty
    ///
    /// # Returns
    ///
    /// `true` if there are no employees, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.employees.is_empty()
    }

    /// Updates an employee's department
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the employee to update
    /// * `new_department` - The new department
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the employee was not found or validation fails
    pub fn update_employee_department(&mut self, name: &str, new_department: String) -> Result<()> {
        let employee = self.get_employee_mut(name)?;
        employee.update_department(new_department)
    }

    /// Updates an employee's phone number
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the employee to update
    /// * `new_phone` - The new phone number
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the employee was not found or validation fails
    pub fn update_employee_phone(&mut self, name: &str, new_phone: String) -> Result<()> {
        let employee = self.get_employee_mut(name)?;
        employee.update_phone(new_phone)
    }

    /// Searches for employees by department
    ///
    /// # Arguments
    ///
    /// * `department` - The department to search for
    ///
    /// # Returns
    ///
    /// A vector of references to employees in the specified department
    pub fn find_by_department(&self, department: &str) -> Vec<&Employee> {
        self.employees
            .values()
            .filter(|employee| employee.department.eq_ignore_ascii_case(department))
            .collect()
    }
}

impl Default for EmployeeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_employee(name: &str, dept: &str, phone: &str) -> Employee {
        Employee::new(name.to_string(), dept.to_string(), phone.to_string()).unwrap()
    }

    #[test]
    fn test_manager_creation() {
        let manager = EmployeeManager::new();
        assert!(manager.is_empty());
        assert_eq!(manager.employee_count(), 0);
    }

    #[test]
    fn test_add_employee() {
        let mut manager = EmployeeManager::new();
        let employee = create_test_employee("Alice Smith", "Marketing", "555-5678");
        
        manager.add_employee(employee).unwrap();
        assert_eq!(manager.employee_count(), 1);
        assert!(!manager.is_empty());
    }

    #[test]
    fn test_get_employee() {
        let mut manager = EmployeeManager::new();
        let employee = create_test_employee("Bob Johnson", "Sales", "555-9012");
        
        manager.add_employee(employee).unwrap();
        
        let retrieved = manager.get_employee("Bob Johnson").unwrap();
        assert_eq!(retrieved.name, "Bob Johnson");
        assert_eq!(retrieved.department, "Sales");
        assert_eq!(retrieved.phone, "555-9012");
    }

    #[test]
    fn test_get_nonexistent_employee() {
        let manager = EmployeeManager::new();
        let result = manager.get_employee("Nonexistent Person");
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EmployeeError::NotFound("Nonexistent Person".to_string()));
    }

    #[test]
    fn test_remove_employee() {
        let mut manager = EmployeeManager::new();
        let employee = create_test_employee("Charlie Brown", "HR", "555-3456");
        
        manager.add_employee(employee).unwrap();
        assert_eq!(manager.employee_count(), 1);
        
        let removed = manager.remove_employee("Charlie Brown").unwrap();
        assert_eq!(removed.name, "Charlie Brown");
        assert_eq!(manager.employee_count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_employee() {
        let mut manager = EmployeeManager::new();
        let result = manager.remove_employee("Nonexistent Person");
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), EmployeeError::NotFound("Nonexistent Person".to_string()));
    }

    #[test]
    fn test_update_employee_department() {
        let mut manager = EmployeeManager::new();
        let employee = create_test_employee("David Wilson", "Engineering", "555-7890");
        
        manager.add_employee(employee).unwrap();
        manager.update_employee_department("David Wilson", "Research".to_string()).unwrap();
        
        let updated = manager.get_employee("David Wilson").unwrap();
        assert_eq!(updated.department, "Research");
    }

    #[test]
    fn test_update_employee_phone() {
        let mut manager = EmployeeManager::new();
        let employee = create_test_employee("Eva Garcia", "Marketing", "555-2468");
        
        manager.add_employee(employee).unwrap();
        manager.update_employee_phone("Eva Garcia", "555-9999".to_string()).unwrap();
        
        let updated = manager.get_employee("Eva Garcia").unwrap();
        assert_eq!(updated.phone, "555-9999");
    }

    #[test]
    fn test_employees_iterator() {
        let mut manager = EmployeeManager::new();
        manager.add_employee(create_test_employee("John Doe", "Engineering", "555-1234")).unwrap();
        manager.add_employee(create_test_employee("Jane Smith", "HR", "555-5678")).unwrap();
        
        let employee_names: Vec<&str> = manager.employees()
            .map(|emp| emp.name.as_str())
            .collect();
        
        assert_eq!(employee_names.len(), 2);
        assert!(employee_names.contains(&"John Doe"));
        assert!(employee_names.contains(&"Jane Smith"));
    }

    #[test]
    fn test_find_by_department() {
        let mut manager = EmployeeManager::new();
        manager.add_employee(create_test_employee("Alice", "Engineering", "555-1111")).unwrap();
        manager.add_employee(create_test_employee("Bob", "Engineering", "555-2222")).unwrap();
        manager.add_employee(create_test_employee("Charlie", "HR", "555-3333")).unwrap();
        
        let engineering_employees = manager.find_by_department("Engineering");
        assert_eq!(engineering_employees.len(), 2);
        
        let hr_employees = manager.find_by_department("HR");
        assert_eq!(hr_employees.len(), 1);
        
        let marketing_employees = manager.find_by_department("Marketing");
        assert_eq!(marketing_employees.len(), 0);
    }

    #[test]
    fn test_replace_existing_employee() {
        let mut manager = EmployeeManager::new();
        let employee1 = create_test_employee("John Doe", "Engineering", "555-1234");
        let employee2 = create_test_employee("John Doe", "Marketing", "555-9999");
        
        manager.add_employee(employee1).unwrap();
        manager.add_employee(employee2).unwrap();
        
        assert_eq!(manager.employee_count(), 1);
        let retrieved = manager.get_employee("John Doe").unwrap();
        assert_eq!(retrieved.department, "Marketing");
        assert_eq!(retrieved.phone, "555-9999");
    }
}