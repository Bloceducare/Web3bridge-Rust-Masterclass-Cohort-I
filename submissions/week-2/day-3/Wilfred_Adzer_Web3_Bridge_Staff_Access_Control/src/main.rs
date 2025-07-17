use uuid::Uuid;



// Determine if an employee can access web3bridge garage using a digital keycard.
// Employees that can access the building are:
//     Media team
//     IT department employees
//     Managers
// Other employees that work at the company are:
//     Social media team
//     Technician supervisors
//     Kitchen staff
// Ensure that terminated employees cannot access the building regardless of their position.

// Notes

// Use an enum to represent all types of employees.
// Use a struct to store:
//     the employee type
//     whether they are still employed
// Use a function that returns a Result to determine if the employee may enter the building.
// Print whether the employee may access the building:
//     Must use a function that utilizes the question mark (?) operator to do this.


#[derive(Debug,PartialEq)]
enum EmployeeType {
    Media,
    IT,
    Manager,
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}

enum EmployeeStatus {
    Employed,
    Terminated,
}

enum AccessControl {
    Granted,
    Denied
}

struct Employee {
    id: u32,
    name: String,
    employee_type: EmployeeType,
    status: EmployeeStatus,
    access: AccessControl,
}

struct EmployeeInfo{
    employee_data: Vec<Employee>,
    next_id: u32,

}


impl EmployeeInfo {
    fn new()-> Self {
        Self {
            employee_data: Vec::new(),
            next_id: 1,
        }
    }


    fn add_employee(&mut self, name: String, employee_type: EmployeeType, status: EmployeeStatus) -> u32 {

        let id = self.next_id;
        let employee = Employee {
            id: id,
            name,
            employee_type,
            status: EmployeeStatus::Employed,
            access: AccessControl::Granted,
        };
        self.next_id +=1;
        self.employee_data.push(employee);
        id
    }


    fn update_employee (&mut self, id: u32, new_name: String, new_type: EmployeeType) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.name = new_name;
            employee.employee_type = new_type;
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }
    fn get_employee(&self, id: u32) -> Result<&Employee, String> {
        self.employee_data
            .iter()
            .find(|e| e.id == id)
            .ok_or(format!("Employee with ID {} not found", id))
    }



    fn generate_access_key(&self, id: u32) -> Result<String, String> {
        let access = self.can_access_garage(id)?;
        match access {
            AccessControl::Granted => {
                let key = Uuid::new_v4().to_string();
                Ok(key)
            }
            AccessControl::Denied => Err(format!("Employee with ID {} does not have access", id)),
        }
    }


    fn can_access_garage(&self, id: u32) -> Result<AccessControl, String> {
        let employee = self.get_employee(id)?; 

        if let EmployeeStatus::Terminated = employee.status {
            return Ok(AccessControl::Denied);
        }

        match employee.employee_type {
            EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager => {
                Ok(AccessControl::Granted)
            }
            _ => Ok(AccessControl::Denied),
        }
    }


    fn terminate_employee(&mut self, id: u32) -> Result<(), String> {
        if let Some(employee) = self.employee_data.iter_mut().find(|e| e.id == id) {
            employee.status = EmployeeStatus::Terminated;
            employee.access = AccessControl::Denied;
            Ok(())
        } else {
            Err(format!("Employee with ID {} not found", id))
        }
    }

    fn get_all_employees(&self) -> &Vec<Employee> {
        &self.employee_data
    }
}


fn main(){
}


#[cfg(test)]

mod tests {
    use super::*;

    fn setup() -> EmployeeInfo {
        let mut employee_info = EmployeeInfo::new();
        employee_info.add_employee("Wilfred".to_string(), EmployeeType::IT, EmployeeStatus::Employed);
        employee_info.add_employee("Chris".to_string(), EmployeeType::Media, EmployeeStatus::Employed);
        employee_info.add_employee("Richard".to_string(), EmployeeType::SocialMedia, EmployeeStatus::Employed);
        employee_info.add_employee("Charlie".to_string(), EmployeeType::TechnicianSupervisor, EmployeeStatus::Employed);
        employee_info.add_employee("Mike".to_string(), EmployeeType::KitchenStaff, EmployeeStatus::Employed);

        employee_info
    }



    #[test]
    fn test_add_employee(){
        let mut employee_info = setup();
        assert_eq!(employee_info.employee_data.len(), 5);
        assert_eq!(employee_info.employee_data[0].name, "Wilfred");
        assert_eq!(employee_info.employee_data[1].name, "Chris");
        assert_eq!(employee_info.employee_data[2].name, "Richard");
        assert_eq!(employee_info.employee_data[3].name, "Charlie");
        assert_eq!(employee_info.employee_data[4].name, "Mike");
        assert_eq!(employee_info.employee_data[0].employee_type, EmployeeType::IT);
        // assert_eq!(employee_info.employee_data[1].status::Employed);
    }



    #[test]
    fn test_generate_access_key() {
        let mut employee_info = setup();
        let key = employee_info.generate_access_key(1);
        assert!(key.is_ok());
        assert!(!key.unwrap().is_empty());

        let key = employee_info.generate_access_key(3);
        assert!(key.is_err());
        assert_eq!(key.unwrap_err(), "Employee with ID 3 does not have access");
    }
}