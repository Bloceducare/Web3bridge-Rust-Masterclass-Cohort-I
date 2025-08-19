#[derive(Debug)]
enum EmployeeType {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMediaTeam,
    TechnicianSupervisor,
    KitchenStaff,
}

struct Employee {
    emp_type: EmployeeType,
    is_employed: bool,
}

fn check_access(employee: &Employee) -> Result<(), &'static str> {
    if !employee.is_employed {
        return Err("Access denied: Employee is terminated");
    }

    match employee.emp_type {
        EmployeeType::MediaTeam => Ok(()),
        EmployeeType::ITDepartment => Ok(()),
        EmployeeType::Manager => Ok(()),
        _ => Err("Access denied: Employee type not authorized"),
    }
}

fn print_access(employee: &Employee) -> Result<(), &'static str> {
    check_access(employee)?;
    println!("Access granted for {:?}", employee.emp_type);
    Ok(())
}

fn main() {
    let employees = vec![
        Employee { emp_type: EmployeeType::MediaTeam, is_employed: true },
        Employee { emp_type: EmployeeType::ITDepartment, is_employed: true },
        Employee { emp_type: EmployeeType::Manager, is_employed: true },
        Employee { emp_type: EmployeeType::SocialMediaTeam, is_employed: true },
        Employee { emp_type: EmployeeType::TechnicianSupervisor, is_employed: true },
        Employee { emp_type: EmployeeType::KitchenStaff, is_employed: true },
        Employee { emp_type: EmployeeType::Manager, is_employed: false },
    ];

    for emp in employees {
        match print_access(&emp) {
            Ok(()) => {},
            Err(e) => println!("{:?}: {}", emp.emp_type, e),
        }
    }
}