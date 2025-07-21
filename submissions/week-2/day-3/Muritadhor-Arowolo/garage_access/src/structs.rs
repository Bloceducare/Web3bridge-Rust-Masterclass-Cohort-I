#[derive(Debug, PartialEq, Clone)]
pub enum EmployeeDepartment{
    Media,
    IT,
    Management,
    SocialMedia,
    TechnicianSupervisor,
    Kitchen
}

#[derive(Debug, PartialEq)]
pub enum EmploymentStatus{
    Active,
    Terminated
}

#[derive(Debug)]
pub enum BuildingType{
    EthereumHouse,
    StaffQuaters,
    LiskGarage
}

pub struct Employee{
    pub id: u32,
    pub name: String,
    pub department: EmployeeDepartment,
    pub status: EmploymentStatus
}

pub struct Building {
    pub id: u32,
    pub building_type: BuildingType,
    pub employees_with_access: Vec<EmployeeDepartment>
}

pub struct Web3Bridge {
    pub employees: Vec<Employee>,
    pub buildings: Vec<Building>,
    pub next_employee_id: u32,
    pub next_building_id: u32
}