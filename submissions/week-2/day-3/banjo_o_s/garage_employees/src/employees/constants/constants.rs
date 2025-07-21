#[derive(Debug, PartialEq)]
pub enum OtherEmployeeRole {
    SocialMedia,
    Technician,
    Kitchen,
}

#[derive(Debug)]
pub enum EmployeeStatus {
    Terminated,
    Engaged,
}

#[derive(Debug, PartialEq)]
pub enum EmployeeRole {
    Media,
    IT,
    Manager,
    OtherEmployeeRole(u8),
}
