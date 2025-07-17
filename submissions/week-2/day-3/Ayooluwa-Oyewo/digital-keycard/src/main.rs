/*
Task:Usage of Enum, Result & the Question Mark Operator
Requirements
Determine if an employee can access web3bridge garage using a digital keycard.
Employees that can access the building are:
Media team
IT department employees
Managers
Other employees that work at the company are:
Social media team
Technician supervisors
Kitchen staff
Ensure that terminated employees cannot access the building regardless of their position.
Notes
Use an enum to represent all types of employees.
Use a struct to store:
the employee type
whether they are still employed
Use a function that returns a Result to determine if the employee may enter the building.
Print whether the employee may access the building:
Must use a function that utilizes the question mark (?) operator to do this.
*/

#[derive(Debug, PartialEq)]
enum EmployeeType {
    Media,
    IT,
    Manager,
    SocialMedia,
    TechnicianSupervisor,
    KitchenStaff,
}
#[derive(Debug)]
struct EmployeeDetails {
    id: u32,
    name: String,
    employee_type: EmployeeType,
    is_employed: bool,
}

#[derive(Debug)]
struct Employee {
    data: Vec<EmployeeDetails>,
    next_id: u32,
}

#[derive(Debug, PartialEq)]
enum EmployeeError {
    EmployeeNotFound,
    EmployeeTerminated,
    EmployeeNotAllowed,
}

impl Employee {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }
    fn add_employee(&mut self, name: String, employee_type: EmployeeType, is_employed: bool) {
        let employee = EmployeeDetails {
            id: self.next_id,
            name,
            employee_type,
            is_employed,
        };
        self.data.push(employee);
        self.next_id += 1;
    }
    fn can_access_building_(&self, employee_id: u32) -> Result<String, EmployeeError> {
        let employee = self
            .data
            .iter()
            .find(|e| e.id == employee_id)
            .ok_or(EmployeeError::EmployeeNotFound)?;

        if !employee.is_employed {
            return Err(EmployeeError::EmployeeTerminated);
        }

        let message = match employee.employee_type {
            EmployeeType::Media | EmployeeType::IT | EmployeeType::Manager => Ok(format!(
                "Employee {} can access the building",
                employee.name
            )),

            EmployeeType::SocialMedia
            | EmployeeType::TechnicianSupervisor
            | EmployeeType::KitchenStaff => Err(EmployeeError::EmployeeNotAllowed),
        };
        message
    }
    pub fn can_access_building(&self, employee_id: u32) {
        match self.can_access_building_(employee_id) {
            Ok(message) => println!("{}", message),
            Err(error) => println!(
                "{}",
                match error {
                    EmployeeError::EmployeeNotAllowed =>
                        format!("Employee {} cannot access the building", employee_id),
                    EmployeeError::EmployeeTerminated =>
                        format!("Employee {} has been terminated", employee_id),
                    EmployeeError::EmployeeNotFound =>
                        format!("Employee {} cannot be found", employee_id),
                }
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_employee() {
        let mut employee = Employee::new();
        employee.add_employee("John Doe".to_string(), EmployeeType::Media, true);
        assert_eq!(employee.data.len(), 1);
        assert_eq!(employee.data[0].name, "John Doe");
        assert_eq!(employee.data[0].employee_type, EmployeeType::Media);
    }

    #[test]
    fn test_employee_can_access_building() {
        let mut employee = Employee::new();
        employee.add_employee("John Doe".to_string(), EmployeeType::Media, true);
        employee.add_employee("Jane Smith".to_string(), EmployeeType::IT, true);
        employee.add_employee("Jim Beam".to_string(), EmployeeType::Manager, false);
        employee.add_employee("Jill Johnson".to_string(), EmployeeType::SocialMedia, false);
        employee.add_employee("King Kong".to_string(), EmployeeType::SocialMedia, true);
        employee.add_employee(
            "Jack Johnson".to_string(),
            EmployeeType::TechnicianSupervisor,
            true,
        );
        employee.add_employee("Jill Johnson".to_string(), EmployeeType::KitchenStaff, true);

        // Test the internal function that returns Result
        assert_eq!(
            employee.can_access_building_(1),
            Ok("Employee John Doe can access the building".to_string())
        );
        assert_eq!(
            employee.can_access_building_(2),
            Ok("Employee Jane Smith can access the building".to_string())
        );

        // Test terminated employee
        assert_eq!(
            employee.can_access_building_(3),
            Err(EmployeeError::EmployeeTerminated)
        );

        // Test not allowed employee
        assert_eq!(
            employee.can_access_building_(4),
            Err(EmployeeError::EmployeeTerminated)
        );
        assert_eq!(
            employee.can_access_building_(5),
            Err(EmployeeError::EmployeeNotAllowed)
        );
        
    }
}

fn main() {
    let mut employee = Employee::new();
    employee.add_employee("John Doe".to_string(), EmployeeType::Media, true);
    employee.add_employee("Jane Smith".to_string(), EmployeeType::IT, true);
    employee.add_employee("Jim Beam".to_string(), EmployeeType::Manager, false);
    employee.add_employee("Jill Johnson".to_string(), EmployeeType::SocialMedia, false);
    employee.add_employee("King Kong".to_string(), EmployeeType::SocialMedia, true);
    employee.add_employee(
        "Jack Johnson".to_string(),
        EmployeeType::TechnicianSupervisor,
        true,
    );
    employee.add_employee("Jill Johnson".to_string(), EmployeeType::KitchenStaff, true);

    println!("Employee created: {:?}", employee.data);

    // Using the function with question mark operator
    employee.can_access_building(1);
    employee.can_access_building(2);
    employee.can_access_building(3);
    employee.can_access_building(4);
    employee.can_access_building(5);
}
