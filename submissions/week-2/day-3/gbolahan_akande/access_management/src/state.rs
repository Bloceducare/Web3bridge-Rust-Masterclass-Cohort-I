
#[derive(Debug)]
pub enum EmployeeType {
    MediaTeam,
    ITdept,
    Managers,
    SocialMedia,
    Supervisors,
    KitchenStaff
}

#[derive(Debug, PartialEq)]
pub enum EmploymentStatus {
    Employed,
    NotEmployed
}

#[derive(Debug)]
pub struct EmployeeDetails {
    pub employee_id: u32,
    pub employee_name: String,
    pub employee_type: EmployeeType,
    pub is_employed: EmploymentStatus
}
