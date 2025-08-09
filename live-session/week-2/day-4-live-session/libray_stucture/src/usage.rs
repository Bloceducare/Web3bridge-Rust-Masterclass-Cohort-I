#[derive(Debug)]
pub enum EmployeeType {
    MediaTeam,
    ITDepartment,
    Manager,
    SocialMedia,
    TechnicalSupervisor,
    KitchenStaff,
}

#[derive(Debug)]
pub enum EmployeeStatus {
    Active,
    Terminated,
}
#[derive(Debug)]
pub struct Employees {
    pub role: EmployeeType,
    pub status: EmployeeStatus,
}

pub fn check_access(ept: &Employees) -> Result<(), String> {
    // You can choose the version you want to use or just learn both,
    // tried this when i got home so as to give youa shorter and more
    // relatable syntax

    if let EmployeeStatus::Terminated = ept.status {
        return Err("You are not allowed!!".to_string());
    }

    //match ept.status {
    //    EmployeeStatus::Terminated => return Err("You are not allowed!!".to_string()),
    //    _ => (),
    //};

    match ept.role {
        EmployeeType::MediaTeam => Ok(()),
        EmployeeType::ITDepartment => Ok(()),
        EmployeeType::Manager => Ok(()),
        _ => Err("Access denied".to_string()),
    }
}

pub fn print_access(print_access: &Employees) {
    println!("The {:?} is {:?}", print_access.role, print_access.status);
}

pub fn attempt_access(employee: &Employees) -> Result<(), String> {
    check_access(employee)?;
    print_access(employee);
    Ok(())
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
