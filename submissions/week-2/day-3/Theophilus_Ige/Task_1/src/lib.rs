#[derive(PartialEq, Clone, Debug)]
pub enum Employee {
    MediaTeam,
    ITDepartmentEmployees,
    Managers,
    SocialMediaTeam,
    TechnicianSupervisors,
    KitchenStaff
}

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    Employed,
    Terminated
}

#[derive(PartialEq, Clone, Debug)]
pub struct EmployeeDetails {
    pub name: String,
    pub position: Employee,
    pub status: Status,
}

