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
    id: u32,
    name: String,
    department: EmployeeDepartment,
    status: EmploymentStatus
}

pub struct Building {
    id: u32,
    building_type: BuildingType,
    employees_with_access: Vec<EmployeeDepartment>
}

pub struct Web3Bridge {
    employees: Vec<Employee>,
    buildings: Vec<Building>,
    next_employee_id: u32,
    next_building_id: u32
}

impl Web3Bridge {
    pub fn new() -> Self {
        Web3Bridge { 
            employees: Vec::new(),
            buildings: Vec::new(),
            next_employee_id: 1,
            next_building_id: 1
        }
    }

    pub fn employ(&mut self, name: String, department: EmployeeDepartment) {
        let new_employee = Employee{
            id: self.next_employee_id,
            name: name,
            department: department,
            status: EmploymentStatus::Active
        };
        println!("New employee: {} created with id {} in Department: {:?}", new_employee.name, new_employee.id, new_employee.department);
        self.employees.push(new_employee);
        self.next_employee_id += 1;
    }

    pub fn build(&mut self, building_type: BuildingType) {
        let new_building = Building {
            id: self.next_building_id,
            building_type: building_type,
            employees_with_access: Vec::new(),
        };
        println!("New {:?} building created with id {}", new_building.building_type, new_building.id);
        self.buildings.push(new_building);
        self.next_building_id += 1;
    }

    pub fn grant_access_to_building(&mut self, building_id: u32, departments: Vec<EmployeeDepartment>) -> bool {
        if let Some(building) = self.buildings.iter_mut().find(|building| building.id == building_id) {
            for department in departments {
                if !building.employees_with_access.contains(&department) {
                    building.employees_with_access.push(department);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn terminate_employment(&mut self, employee_id: u32) -> bool{
        if let Some(employee) = self.employees.iter_mut().find(|employee| employee.id == employee_id) {
            employee.status = EmploymentStatus::Terminated;
            println!("{}'s employment with Web3bridge terminated!", employee.name);
            true
        } else {
            false
        }
        
    }

    pub fn can_access_building(&mut self, building_id: u32, employee_id: u32) -> Result<bool, bool> {
        let building = self.buildings
            .iter()
            .find(|building| building.id == building_id)
            .ok_or(false)?;

        let employee = self.employees
            .iter()
            .find(|employee| employee.id == employee_id)
            .ok_or(false)?;

        if building.employees_with_access.contains(&employee.department)
            && employee.status == EmploymentStatus::Active
        {
            println!("Welcome {}! You may access the {:?} building :)", employee.name, building.building_type);
            Ok(true)
        } else {
            println!("{} denied acesss to {:?} building!", employee.name, building.building_type);
            Err(false)
        }
    }

    pub fn get_employee(&self, employee_id: u32) -> &Employee {
        self.employees.iter().find(|employee| employee.id == employee_id).unwrap()
    }

    pub fn get_building(&self, building_id: u32) -> &Building {
        self.buildings.iter().find(|building| building.id == building_id).unwrap()
    }
}