#[derive(Debug)]
enum Status {
    Employed,
    Unemployed, 
}

#[derive(Debug)]
enum EmploymentType {
    Media,
    Manager,
    IT,
    SocialMedia,
    Technician,
    Kitchen,
}

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    employment_type: EmploymentType,
    is_employed: Status,
}

struct EmployeeData {
    employee_data: Vec<Employee>,
    new_id: u32,
}

impl EmployeeData {
    fn new() -> Self {
        Self {
            employee_data: Vec::new(),
            new_id: 1,
        }
    }

    fn employ(&mut self, name: String, employment_type: EmploymentType) -> u32 {
        let n_id: u32 = self.new_id;
        let employee = Employee {
            id: n_id,
            name,
            employment_type,
            is_employed: Status::Employed,
        };
        self.employee_data.push(employee);
        self.new_id += 1;
        n_id
    }

 
    fn employee_access(&mut self, employment_type: EmploymentType) -> Result<(), String> {
        let employ_access = match employment_type {
            EmploymentType::IT => "Grant Access",
            EmploymentType::Kitchen => "No Access",
            EmploymentType::Manager => "Grant Access",
            EmploymentType::Media => "Grant Access",
            EmploymentType::SocialMedia => "No Access",
            EmploymentType::Technician => "No Access",
        }; 

        if employ_access == "Grant Access" {
            println!("{:?} Access Granted", employment_type);
            Ok(())
        } else {
            println!("{:?} No access", employment_type); 
            Err("Access denied".to_string())
        }
    }

  
    fn list_employees(&self) {
        println!("Current Employees:");
        for employee in &self.employee_data {
            println!("ID: {}, Name: {}, Type: {:?}, Status: {:?}", 
                employee.id, employee.name, employee.employment_type, employee.is_employed);
        }
    }

    fn find_employee_by_id(&self, id: u32) -> Option<&Employee> {
        self.employee_data.iter().find(|emp| emp.id == id)
    }
}

fn main() {
    println!("Employee Management System");
    
    let mut company = EmployeeData::new();
    
 
    let id1 = company.employ("Alice Johnson".to_string(), EmploymentType::IT);
    let id2 = company.employ("Bob Smith".to_string(), EmploymentType::Kitchen);
    let id3 = company.employ("Carol Brown".to_string(), EmploymentType::Manager);
    
    println!("Added employees with IDs: {}, {}, {}", id1, id2, id3);
    

    company.list_employees();
    
 
    println!("\nTesting Access Control:");
    let _ = company.employee_access(EmploymentType::IT);
    let _ = company.employee_access(EmploymentType::Kitchen);
    let _ = company.employee_access(EmploymentType::Manager);
    let _ = company.employee_access(EmploymentType::SocialMedia);
}