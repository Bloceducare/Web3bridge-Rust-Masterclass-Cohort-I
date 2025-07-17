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

#[derive(Clone, Debug)]
pub struct Web3Bridge {
    name: String,
    sector: employeeDep,
    status: employeeStatus,
}

#[derive(Clone, Debug)]
pub enum employeeDep {
    Media,
    IT,
    Managers,
    SocialMedia,
    TechnicianSuper,
    KitchenStaff,
}

#[derive(Clone, Debug, PartialEq)]
pub enum employeeStatus {
    Active,
    Terminated,
}

impl employeeDep {
    pub fn check_access(self) -> Result<(), String> {
        match self {
            employeeDep::Media => {
                println!("has access");
                Ok(())
            }
            employeeDep::IT => {
                println!("has access");
                Ok(())
            }
            employeeDep::Managers => {
                println!("has access");
                Ok(())
            }
            employeeDep::SocialMedia => {
                println!("has no access");
                Ok(())
            }
            employeeDep::TechnicianSuper => {
                println!("has no access");
                Ok(())
            }
            employeeDep::KitchenStaff => {
                println!("has no access");
                Ok(())
            }
            _=> Err("Not found".to_string()),
        }
    }
}

impl Web3Bridge {
    pub fn init(name: String, sector: employeeDep, status: employeeStatus) -> Self {
        Self {
            name: name,
            sector: sector,
            status: status,
        }
    }

    pub fn authorize_access(self) -> Result<(), String> {
        if self.status == employeeStatus::Active {
            self.sector.check_access()
        } else {
            Err("Employee has no access".to_string())
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Web3Bridge {
        Web3Bridge::init(
            "Ayoola".to_string(),
            employeeDep::Media,
            employeeStatus::Active,
        )
    }

    #[test]
    fn test_check_access() {
        let employee = setup();
        assert!(employee.authorize_access().is_ok());
    }

    #[test]
    fn test_check_access_to_fail() {
        let employee = Web3Bridge::init(
            "Security Reseacher".to_string(),
            employeeDep::SocialMedia,
            employeeStatus::Terminated,
        );
        assert!(employee.authorize_access().is_err())
    }
}


