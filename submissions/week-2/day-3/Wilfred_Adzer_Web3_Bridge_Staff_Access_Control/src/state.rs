/// Represents the different types of employees at Web3Bridge
#[derive(Debug, Clone, PartialEq)]
pub enum EmployeeType {
    Media,
    IT,
    Manager,
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}

/// Represents the employment status of an employee
#[derive(Debug, Clone, PartialEq)]
pub enum EmployeeStatus {
    Employed,
    Terminated,
}

/// Represents access control status for the garage
#[derive(Debug, Clone, PartialEq)]
 pub enum AccessControl {
    Granted,
    Denied,
}

/// Represents an employee with all their information
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    // pub employee_data: Vec<Employee>,
    pub employee_type: EmployeeType,
    pub status: EmployeeStatus,
    pub access: AccessControl,
}


#[derive(Debug, Clone, PartialEq)]
pub struct EmployeeInfo {
    pub employee_data: Vec<Employee>,
    pub next_id: u32,
    pub active_keys: std::collections::HashMap<String, u32>, // Maps access keys to employee IDs
}
