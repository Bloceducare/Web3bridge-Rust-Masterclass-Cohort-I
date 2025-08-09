#[derive(Clone, Copy)]
pub enum Employees {
    MediaTeam,
    IT,
    Managers,
    SMTeam,
    Technician,
    KitchenStaff,
}

#[derive(PartialEq)]
pub enum EmployeesStatus {
    Employed,
    Unemployed,
}

pub struct Staff {
    pub employment: Employees,
    pub status: EmployeesStatus,
}