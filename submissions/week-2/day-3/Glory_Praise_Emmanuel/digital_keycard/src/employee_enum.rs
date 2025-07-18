#[derive(Debug)]
pub enum EmployeeType {
     MultiMedia,
     Ict,
     Manager,
     SocialMedia,
     TechnicianSupervisor,
     KitchenStaff,
}

#[derive(Debug, PartialEq)]
pub enum EmploymentStatus {
    Employed,
    Fired,
}

#[derive(Debug, PartialEq)]
pub enum AccessError {
    NotEmployed,
    NotAuthorized,
}