#[derive(Debug)]
enum Status {
    Employed,
    UmEmployed,
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
    fn employee_access(&mut self, employment_type: EmploymentType) -> Result<> {
        let employ_access: String = match employment_type {
            EmploymentType::IT => {
                "Grant Access";
            }
            EmploymentType::Kitchen => {
                "No Access";

            }
            EmploymentType::Manager => {
                "Grant Access";

            }
            EmploymentType::Media => {
                "Grant Access";

            }
            EmploymentType::SocialMedia => {
                "No Access";

            }
            EmploymentType::Technician => {
                "No Access";

            }
        if employ_access == "Grant Access" {
            println!("{:?} Access Granted", employment_type);
        }
        else {
            println!("{:?}No access", employment_type);
        }
        
    }
}




fn main() {
    println!("Hello, world!");
}
