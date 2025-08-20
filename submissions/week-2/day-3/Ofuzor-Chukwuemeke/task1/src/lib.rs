use std::fmt::Error;

#[derive(Debug, PartialEq)]
pub enum Employee {
    MEDIA_TEAM,
    IT_DEPARTMENT,
    MANAGERS,
    SOCIAL_MEDIA_TEAM,
    TECHNICAL_SUPERVISORS,
    KITCHEN_STAFF,
}

#[derive(Debug)]
pub struct Store {
    pub employee: Employee,
    pub terminated: bool,
}

impl Store {
    fn new_staff(employee: Employee, terminated: bool) -> Self {
        Self {
            employee,
            terminated,
        }
    }

    fn terminate(&mut self) {
        self.terminated = false
    }

    fn access(self) -> Result<Employee, Error> {
        match self.employee {
            Employee::IT_DEPARTMENT => {
                println!("Can Access the Building");
                Ok(Employee::IT_DEPARTMENT)
            }
            Employee::MANAGERS => {
                println!("Can Access the Building");
                Ok(Employee::MANAGERS)
            }
            Employee::MEDIA_TEAM => {
                println!("Can Access the Building");
                Ok(Employee::MEDIA_TEAM)
            }
            _ => Err(Error),
        }
    }
}