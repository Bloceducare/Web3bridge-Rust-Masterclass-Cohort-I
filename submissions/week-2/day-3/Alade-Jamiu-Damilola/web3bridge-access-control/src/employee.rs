#[derive(Debug, Clone, Copy)]
pub enum EmployeeType {
    MediaTeam,
    IT,
    Manager,
    SocialMediaTeam,
    TechnicalSupervisor,
    KitchenStaff,
}

#[derive(Debug, Clone)]
pub struct Employee {
    pub employee_type: EmployeeType,
    pub is_employed: bool,
}

impl Employee {
    pub fn new(employee_type: EmployeeType, is_employed: bool) -> Self {
        Self {
            employee_type,
            is_employed,
        }
    }
}