#[derive(Debug, PartialEq, Clone)]
pub enum EmployeeType {
    MediaTeam,
    IT,
    Manager,
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Clone)]
pub struct Employee {
    pub employee_type: EmployeeType,
    pub is_employed: bool,
}
