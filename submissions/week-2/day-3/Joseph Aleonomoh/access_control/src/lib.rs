enum EmployeeType {
    Media,
    IT,
    Managers,
    SocialMedia,
    TechnicianSupervisors,
    KitchenStaff,
}

enum EmployeeStatus {
    Active,
    InActive,
}

struct EmployeeDetails{
    id: u32,
    name: String,
    employee_type: EmployeeType,
    status: EmployeeStatus,
}

struct Employees {
    employees : Vec<EmployeeDetails>,
    next_id: u32,
}


impl Employees {
    fn new() -> Self {
        Self {
            employees: Vec::new(),
            next_id: 1,
        }
    }

    fn employ(&mut self, name: String, employee_type: EmployeeType) -> u32 {
        let id = self.next_id;
        let employee = EmployeeDetails {
            id,
            name,
            employee_type,
            status: EmployeeStatus::Active,
        };
        self.employees.push(employee);
        self.next_id = self.next_id + 1;
        id
    }

    fn terminate(&mut self, id: u32) -> bool {
        if let Some(employee) = self.employees.iter_mut().find(|employee| employee.id == id){
            employee.status = EmployeeStatus::InActive;
            true
        } else {
            false
        }
    }

    fn get_employee(&self, id: u32) -> Result<&EmployeeDetails, String> {
        if let Some(employee) = self.employees.iter().find(|employee| employee.id == id){
            Ok(employee)
        } else {
            Err(String::from("Employee does not exist"))
        }
    } 

    fn access_building(&self, id: u32) -> Result<bool, String> {
        let employee = self.get_employee(id)?;
        match employee.status{ 
            EmployeeStatus::InActive => Err(String::from("Employee Terminated")),
            EmployeeStatus::Active => {
                match employee.employee_type {
                    EmployeeType::Media => Ok(true),
                    EmployeeType::IT => Ok(true),
                    EmployeeType::Managers => Ok(true),
                    EmployeeType::SocialMedia => Err(String::from("Not Allowed")),
                    EmployeeType::TechnicianSupervisors => Err(String::from("Not Allowed")),
                    EmployeeType::KitchenStaff => Err(String::from("Not Allowed")),
                }
            }
        }
        
    }
}

#[cfg(test)]

mod tests {
    use super::*;


    #[test]
    fn test_employ() {
        let employee_1_name = "joseph".to_string();
        let employee_1_type = EmployeeType::Managers;

        let employee_2_name = "leo".to_string();
        let employee_2_type = EmployeeType::Media;

        let employee_3_name = "david".to_string();
        let employee_3_type = EmployeeType::Managers;

        let mut employees = Employees::new();
        let employ_1 = employees.employ(employee_1_name, employee_1_type);
        assert_eq!(employ_1, 1);
        let employ_2 = employees.employ(employee_2_name, employee_2_type);
        assert_eq!(employ_2, 2);
        let employ_3 = employees.employ(employee_3_name, employee_3_type);
        assert_eq!(employ_3, 3);
    }

    #[test]
    fn test_terminate() {
        let employee_1_name = "joseph".to_string();
        let employee_1_type = EmployeeType::Managers;

        let mut employees = Employees::new();
        let employ_1 = employees.employ(employee_1_name, employee_1_type);
        assert_eq!(employ_1, 1);

        let terminate = employees.terminate(1);
        assert_eq!(terminate, true);
    }

    #[test]
    fn test_get_employee() {
        let employee_1_name = "joseph".to_string();
        let employee_1_type = EmployeeType::Managers;

        let mut employees = Employees::new();
        let employ_1 = employees.employ(employee_1_name, employee_1_type);
        assert_eq!(employ_1, 1);

        let employee = employees.get_employee(1);
        assert_eq!(employee.unwrap().name, "joseph".to_string());
    }

    #[test]
    fn test_access_building() {
        let employee_1_name = "joseph".to_string();
        let employee_1_type = EmployeeType::Managers;

        let employee_2_name = "leo".to_string();
        let employee_2_type = EmployeeType::Media;

        let employee_3_name = "david".to_string();
        let employee_3_type = EmployeeType::Managers;

        let mut employees = Employees::new();
        let employ_1 = employees.employ(employee_1_name, employee_1_type);
        assert_eq!(employ_1, 1);
        let employ_2 = employees.employ(employee_2_name, employee_2_type);
        assert_eq!(employ_2, 2);
        let employ_3 = employees.employ(employee_3_name, employee_3_type);
        assert_eq!(employ_3, 3);
        
        let terminate = employees.terminate(1);
        assert_eq!(terminate, true);

        match employees.access_building(1) {
            Ok(access) => println!("Access = {}", access),
            Err(err) => println!("{}", err)
        }

        match employees.access_building(2) {
            Ok(access) => println!("Access = {}", access),
            Err(err) => println!("{}", err)
        }
    }
}