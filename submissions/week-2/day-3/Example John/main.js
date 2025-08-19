#[derive(Clone, Copy)]
enum Employees {
    MediaTeam,
    IT,
    Managers,
    SMTeam,
    Technician,
    KitchenStaff,
}

impl Employees {
    fn require_level_access(self) -> Result<(), String> {
        match self {
            Employees::MediaTeam => {
                println!("Access granted");
                Ok(())
            }
            Employees::IT => {
                println!("Access granted");
                Ok(())
            }
            Employees::Managers => {
                println!("Access granted");
                Ok(())
            }
            Employees::SMTeam => {
                println!("You do not have access to this area");
                Ok(())
            }
            Employees::Technician => {
                println!("You do not have access to this area");
                Ok(())
            }
            Employees::KitchenStaff => {
                println!("You do not have access to this area");
                Ok(())
            }
            _ => Err("You are not an employee".to_string()),
        }
    }
}

#[derive(PartialEq)]
enum EmployeesStatus {
    Employed,
    Unemployed,
}

struct Staff {
    employment: Employees,
    status: EmployeesStatus,
}

impl Staff {
    fn new(status: EmployeesStatus, employment: Employees) -> Self {
        Self { status, employment }
    }

    fn check_employee_access(&self) -> Result<(), String> {
        if self.status == EmployeesStatus::Employed {
            self.employment.require_level_access()
        } else {
            Err("You are not Employed".to_string())
        }
    }

    fn try_access(&self) -> Result<(), String> {
        self.check_employee_access()?;
        Ok(())
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Staff {
        Staff::new(EmployeesStatus::Employed, Employees::MediaTeam)
    }

    #[test]
    fn check_employee_access() {
        let staff = setup();
        assert!(staff.check_employee_access().is_ok())
    }

    #[test]
    fn check_employee_access_fail() {
        let staff = Staff::new(EmployeesStatus::Unemployed, Employees::MediaTeam);
        assert!(staff.check_employee_access().is_err());
    }

    #[test]
    fn test_try_access() {
        let staff = setup();
        assert!(staff.try_access().is_ok());
    }
}
