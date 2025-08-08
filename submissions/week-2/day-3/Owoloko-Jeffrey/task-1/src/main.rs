#[derive(Debug, PartialEq)]
pub enum EmployeeType {
    Media_Team,
    IT_Department,
    Managers,
    Social_Media_Team,
    Technician_Supervisors,
    Kitchen_Staff,
}

#[derive(Debug, PartialEq)]
pub enum EmployeeStatus {
    Employed,
    Unemployed,
}

#[derive(Debug)]
pub struct Employee {
    employee_type: EmployeeType,
    employee_status: EmployeeStatus,
}

impl Employee {
    fn create_employee(employee_type: EmployeeType, employee_status: EmployeeStatus) -> Self {
        Self {
            employee_type,
            employee_status,
        }
    }

    fn check_if_employee_can_enter_building(&self) -> Result<(), String> {
        match self.employee_status {
            EmployeeStatus::Employed => match self.employee_type {
                EmployeeType::Media_Team => {
                    println!("Can Enter building");
                }
                EmployeeType::IT_Department => {
                    println!("Can Enter building");
                }
                EmployeeType::Managers => {
                    println!("Can Enter building");
                }
                _ => {
                    println!("Cannot Enter building");
                }
            },
            _ => {
                println!("Cannot Enter building");
            }
        }
        Ok(())
    }
}

fn main() {
    let employee = Employee::create_employee(EmployeeType::Media_Team, EmployeeStatus::Unemployed);

    employee.check_if_employee_can_enter_building();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_employee() {
        let employee =
            Employee::create_employee(EmployeeType::Media_Team, EmployeeStatus::Unemployed);

        assert_eq!(employee.employee_type == EmployeeType::Media_Team, true);
    }

    #[test]
    fn test_check_if_employee_can_enter_building() {
        let employee = Employee::create_employee(EmployeeType::Media_Team, EmployeeStatus::Unemployed);

        assert_eq!(employee.check_if_employee_can_enter_building().is_ok(), true);
    }
}
