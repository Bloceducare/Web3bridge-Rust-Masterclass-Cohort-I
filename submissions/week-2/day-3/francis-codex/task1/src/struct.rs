#[derive(Debug)]
pub enum Employees {
    MediaTeam, 
    IT,
    Managers, 
    SocialMedia,
    TechnicianSupervisors,
    KitchenStaff,
}

#[derive(Debug)]
pub struct Employee {
    pub position: Employees,
    pub is_employed: bool, 
}