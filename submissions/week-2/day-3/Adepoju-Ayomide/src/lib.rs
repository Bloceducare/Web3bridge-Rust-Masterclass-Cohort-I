#[derive(Clone, Debug)]
pub struct Web3Bridge {
    name: String,
    sector: employeeRole,
    status: employeeStatus,
}

#[derive(Clone, Debug)]
pub enum employeeRole {
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

impl employeeRole {
    pub fn check_access(self) -> Result<(), String> {
        match self {
            employeeRole::Media => {
                println!("has access");
                Ok(())
            }
            employeeRole::IT => {
                println!("has access");
                Ok(())
            }
            employeeRole::Managers => {
                println!("has access");
                Ok(())
            }
            employeeRole::SocialMedia => {
                println!("has no access");
                Ok(())
            }
            employeeRole::TechnicianSuper => {
                println!("has no access");
                Ok(())
            }
            employeeRole::KitchenStaff => {
                println!("has no access");
                Ok(())
            }
            _=> Err("Not found".to_string()),
        }
    }
}

impl Web3Bridge {
    pub fn init(name: String, sector: employeeRole, status: employeeStatus) -> Self {
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
            employeeRole::Media,
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
            employeeRole::SocialMedia,
            employeeStatus::Terminated,
        );
        assert!(employee.authorize_access().is_err())
    }
}