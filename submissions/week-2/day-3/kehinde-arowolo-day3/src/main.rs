#[derive(Debug, Clone)]
enum EmployeeType {
    Media,
    ITDepartment,
    Manager,
    SocialMedia,
    Technician,
    KitchenStaff,
}

#[derive(Debug)]
struct Employee {
    name: String,
    employee_type: EmployeeType,
    is_employed: bool,
}

#[derive(Debug)]
enum AccessError {
    Terminated,
    Unauthorized,
}

impl AccessError {
    fn message(&self) -> &str {
        match self {
            AccessError::Terminated => " Access denied: Employee is terminated",
            AccessError::Unauthorized => "Access denied: Employee role is not authorized for garage access",
        }
    }
}

impl Employee {
    fn new(name: &str, employee_type: EmployeeType, is_employed: bool) -> Self {
        Employee {
            name: name.to_string(),
            employee_type,
            is_employed,
        }
    }
    
    fn role_name(&self) -> &str {
        match self.employee_type {
            EmployeeType::Media => "Media Team",
            EmployeeType::ITDepartment => "IT Department",
            EmployeeType::Manager => "Manager",
            EmployeeType::SocialMedia => "Social Media Team",
            EmployeeType::Technician => "Technician Supervisor",
            EmployeeType::KitchenStaff => "Kitchen Staff",
        }
    }
}

fn check_building_access(employee: &Employee) -> Result<(), AccessError> {
    if !employee.is_employed {
        return Err(AccessError::Terminated);
    }
    
    match employee.employee_type {
        EmployeeType::Media | EmployeeType::ITDepartment | EmployeeType::Manager => Ok(()),
        EmployeeType::SocialMedia | EmployeeType::Technician | EmployeeType::KitchenStaff => {
            Err(AccessError::Unauthorized)
        }
    }
}

fn print_access_status(employee: &Employee) -> Result<(), AccessError> {
    check_building_access(employee)?;
    println!("{} ({}) can access the web3bridge garage", employee.name, employee.role_name());
    Ok(())
}

fn main() {
    let employees = vec![
        Employee::new("Arowolo", EmployeeType::Media, true),
        Employee::new("Vicent", EmployeeType::ITDepartment, true),
        Employee::new("David", EmployeeType::Manager, true),
        Employee::new("Acheiver", EmployeeType::SocialMedia, true),
        Employee::new("Fuhad", EmployeeType::Technician, true),
        Employee::new("Frank", EmployeeType::KitchenStaff, true),
        Employee::new("Ayomide", EmployeeType::Manager, false), // Terminated
    ];

    println!("Web3Bridge Garage Access Control");

    for employee in &employees {
        println!("\n Testing: {} ({})", employee.name, employee.role_name());
        println!("   Status: {}", if employee.is_employed { " Active" } else { " Terminated" });
        
        match print_access_status(employee) {
            Ok(()) => {},
            Err(e) => println!("   {}", e.message()),
        }
    }
}

