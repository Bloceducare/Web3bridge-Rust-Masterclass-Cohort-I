use std::result;

#[derive(Debug)]
enum EmployeeType {
    MediaTeam,
    ITdept,
    Managers,
    SocialMedia,
    Supervisors,
    KitchenStaff
}

#[derive(PartialEq)] 
#[derive(Debug)]
enum EmploymentStatus {
    Employed,
    NotEmployed
}

#[derive(Debug)]
struct EmployeeDetails {
    employee_id: u32,
    employee_name: String,
    employee_type: EmployeeType,
    is_employed: EmploymentStatus
}

impl EmployeeDetails {

    fn is_employee_access(&mut self) -> Result<&str, &str> {
        if self.is_employed == EmploymentStatus::Employed {
            match self.employee_type {
                EmployeeType::ITdept => { Ok("You have access") },
                EmployeeType::KitchenStaff => { Err("No access") },
                EmployeeType::Managers => { Ok("You have access") },
                EmployeeType::MediaTeam => { Ok("You have access") },
                EmployeeType::SocialMedia => { Err("No access") },
                EmployeeType::Supervisors => { Err("No access") },
            }
        } else {
            Err("You are not Employed")
        }
    }

    fn show_access(&mut self) -> Result<&str, &str> {
        let pass = self.is_employee_access()?;
        println!("Access Granted {}", pass);
        Ok(pass)
    }

    fn print_access(&mut self) -> Result<(), &str> {
        match self.show_access() {
            Ok(pass) => {
                println!("You have access");
                Ok(())
            }
            Err(Fail) => {
                println!("You have no access");
                Ok(())
            }
        }
    }
}

fn main() {
    let mut new_access1 = EmployeeDetails {
        employee_id: 1,
        employee_name: String::from("Gbolahan"),
        employee_type: EmployeeType::MediaTeam,
        is_employed: EmploymentStatus::Employed
    };

    let mut new_access2 = EmployeeDetails {
        employee_id: 1,
        employee_name: String::from("Gbolahan"),
        employee_type: EmployeeType::ITdept,
        is_employed: EmploymentStatus::NotEmployed
    };
    let me1 = new_access1.print_access();
    println!("{:#?}", me1);
    let me2 = new_access2.print_access();
    println!("{:#?}", me2);
}
