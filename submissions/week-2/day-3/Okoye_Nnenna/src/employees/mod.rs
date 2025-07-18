#[derive(Debug, PartialEq)]
pub enum EmployeeType {
    Media,
    IT,
    Manager,
    SocialMedia,
    Technician,
    Kitchen,
}

#[derive(Debug)]
pub struct Employee {
    pub role: EmployeeType,
    pub is_employed: bool,
}

impl Employee {
    pub fn new(role: EmployeeType, is_employed: bool) -> Self {
        Employee { role, is_employed }
    }
}
