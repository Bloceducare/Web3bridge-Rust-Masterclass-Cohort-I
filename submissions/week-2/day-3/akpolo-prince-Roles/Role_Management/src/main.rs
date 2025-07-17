// # Task:Usage of Enum, Result & the Question Mark Operator

// ## Requirements

// - Determine if an employee can access web3bridge garage using a digital keycard.
// - Employees that **can access** the building are:
//   - Media team
//   - IT department employees
//   - Managers
// - Other employees that **work at the company** are:
//   - Social media team
//   - Technician supervisors
//   - Kitchen staff
// - Ensure that **terminated employees cannot access** the building regardless of their position.

// ## Notes

// - Use an `enum` to represent all types of employees.
// - Use a `struct` to store:
//   - the employee type
//   - whether they are still employed
// - Use a function that returns a `Result` to determine if the employee may enter the building.
// - Print whether the employee may access the building:
//   - Must use a function that utilizes the **question mark (`?`) operator** to do this.

// ---
#[derive(Debug)]
enum EmployeeType {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
enum EmploymentStatus {
    Employed,
    Terminated,
}

#[derive(Debug)]
struct Employee {
    name: String,
    employee_type: EmployeeType,
    status: EmploymentStatus,
    Eid: u32,
}

#[derive(Debug)]
enum AccessError {
    TerminatedEmployee,
    NoPermission,
}

impl Employee {
    fn new(name: String, employee_type: EmployeeType, status: EmploymentStatus, _id: u32) -> Self {
        Employee {
            name,
            employee_type,
            status,
            Eid: _id,
        }
    }

    fn can_access_building(&self) -> Result<bool, AccessError> {
        match self.status {
            EmploymentStatus::Terminated => Err(AccessError::TerminatedEmployee),
            EmploymentStatus::Employed => {
                match self.employee_type {
                    EmployeeType::MediaTeam | EmployeeType::ITDepartment | EmployeeType::Manager => {
                        Ok(true)
                    }
                    EmployeeType::SocialMediaTeam | EmployeeType::TechnicianSupervisor | EmployeeType::KitchenStaff => {
                        Err(AccessError::NoPermission)
                    }
                }
            }
        }
    }
}

struct Staffs {
    employees: Vec<Employee>,
    next_id: u32,
}

impl Staffs {
    fn new() -> Self {
        Staffs {
            employees: Vec::new(),
            next_id: 1,
        }
    }

    fn add_employee(&mut self, name: String, employee_type: EmployeeType, status: EmploymentStatus) {
        let employee = Employee::new(name, employee_type, status, self.next_id);
        self.employees.push(employee);
        self.next_id += 1;
    }
}

fn check_access(employee: &Employee) -> Result<(), AccessError> {
    employee.can_access_building()?;
    println!("Access granted for {}", employee.name);
    Ok(())
}

fn print_result(employee: &Employee) {
    match check_access(employee) {
        Ok(()) => {},
        Err(AccessError::TerminatedEmployee) => {
            println!("Access denied for {} - terminated", employee.name);
        }
        Err(AccessError::NoPermission) => {
            println!("Access denied for {} - insufficient permissions", employee.name);
        }
    }
}

fn main() {
    println!("welcome to garage");
    
    let mut staff = Staffs::new();
    
    staff.add_employee("john".to_string(), EmployeeType::MediaTeam, EmploymentStatus::Employed);
    staff.add_employee("sarah".to_string(), EmployeeType::ITDepartment, EmploymentStatus::Employed);
    staff.add_employee("mike".to_string(), EmployeeType::Manager, EmploymentStatus::Employed);
    staff.add_employee("lisa".to_string(), EmployeeType::SocialMediaTeam, EmploymentStatus::Employed);
    staff.add_employee("tom".to_string(), EmployeeType::KitchenStaff, EmploymentStatus::Employed);
    staff.add_employee("alex".to_string(), EmployeeType::Manager, EmploymentStatus::Terminated);
    staff.add_employee("emma".to_string(), EmployeeType::TechnicianSupervisor, EmploymentStatus::Employed);
    
    for employee in &staff.employees {
        print_result(employee);
    }
}


