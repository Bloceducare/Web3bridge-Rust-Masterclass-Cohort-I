use std::fmt::Error;

#[derive(PartialEq)]
enum Employee {
    Media,
    IT,
    Manager,
    SocialMedia,
    Supervisor,
    Kitchen,
}
#[derive(Debug, PartialEq)]
enum HasAccess {
    Affirmative,
    Negative,
}

#[derive(Debug, PartialEq)]
enum IsEmployed {
    Active,
    Inactive,
}

struct Web3Bridge {
    employee: Employee,
    employment_status: IsEmployed,
}

impl Web3Bridge {
    fn new(employee: Employee) -> Self {
        Web3Bridge {
            employee,
            employment_status: IsEmployed::Active,
        }
    }

    fn give_access(&self) -> HasAccess {
        match self.employee {
            Employee::Media | Employee::IT | Employee::Manager => HasAccess::Affirmative,
            _ => HasAccess::Negative,
        }
    }

    fn has_access(&self) -> Result<bool, Error> {
        if self.give_access() == HasAccess::Affirmative {
            println!("I have access");
            Ok(true)
        } else {
            println!("I do not have access");
            Err(Error)
        }
    }

    fn is_employed(&self) -> &IsEmployed {
        &self.employment_status
    }
    fn sacked(&mut self, employee: Employee) {
        if self.employee == employee {
            self.employment_status = IsEmployed::Inactive;
        }
    }
}

fn main() -> Result<(), Error> {
    let employee_IT = Web3Bridge::new(Employee::IT);
    let mut employee_kitchen = Web3Bridge::new(Employee::Kitchen);

    match employee_IT.give_access() {
        HasAccess::Affirmative => println!("Access granted."),
        HasAccess::Negative => println!("Access denied."),
    }

    match employee_IT.is_employed() {
        IsEmployed::Active => println!("Employee is active."),
        IsEmployed::Inactive => println!("Employee is inactive."),
    }

    employee_kitchen.sacked(Employee::Kitchen);
    match employee_kitchen.is_employed() {
        IsEmployed::Active => println!("Employee is active."),
        IsEmployed::Inactive => println!("Employee is inactive."),
    }

    employee_kitchen.has_access()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_access() {
        let employee = Web3Bridge::new(Employee::IT);
        assert_eq!(employee.give_access(), HasAccess::Affirmative);
    }

    #[test]
    fn test_employee_employment_status() {
        let employee = Web3Bridge::new(Employee::Kitchen);
        assert_eq!(employee.is_employed(), &IsEmployed::Active);
    }

    #[test]
    fn test_staff_sacked() {
        let mut employee = Web3Bridge::new(Employee::Media);
        employee.sacked(Employee::Media);

        assert_eq!(employee.is_employed(), &IsEmployed::Inactive);
    }

    #[test]
    fn test_access() {
        let employee = Web3Bridge::new(Employee::Kitchen);
        employee.give_access();
        assert_eq!(employee.has_access(), Err(Error));
    }
}
