use std::fmt::Error;

#[derive(Debug)]
enum Employee {
    MEDIA_TEAM,
    IT_DEPARTMENT,
    MANAGERS,
    SOCIAL_MEDIA_TEAM,
    TECHNICAL_SUPERVISORS,
    KITCHEN_STAFF,
}

#[derive(Debug)]
struct Store {
    employee: Employee,
    terminated: bool,
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
            Employee::KITCHEN_STAFF => {
                print!("Can not Access the Building");
                Ok(Employee::KITCHEN_STAFF)
            }
            Employee::SOCIAL_MEDIA_TEAM => {
                print!("Can not Access the Building");
                Ok(Employee::SOCIAL_MEDIA_TEAM)
            }
            Employee::TECHNICAL_SUPERVISORS => {
                print!("Can not Access the Building");
                Ok(Employee::TECHNICAL_SUPERVISORS)
            }
            _ => Err(Error),
        }
    }
}

fn main() {
    let employee = Employee::KITCHEN_STAFF;
    let store = Store {
        employee,
        terminated: true,
    };
}
