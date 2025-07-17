#[derive(Debug)]
enum EmployeeType {
     MultiMedia,
     Ict,
     Manager,
     SocialMedia,
     TechnicianSupervisor,
     KitchenStaff,
}

#[derive(Debug, PartialEq)]
enum EmploymentStatus {
    Employed,
    Fired,
}

#[derive(Debug, PartialEq)]
enum AccessError {
    NotEmployed,
    NotAuthorized,
}

#[derive(Debug)]
struct Employee {
    id: u32,
    name: String,
    age: u32,
    dept: EmployeeType,
    status: EmploymentStatus,
    
}

struct EmployeeDB {
    data: Vec<Employee>,
    next_id: u32,
}

impl EmployeeDB {
    fn new() -> Self {
        Self{ 
            data: Vec::new(),
            next_id: 1,
         }
    }

    fn new_employee(&mut self, employee: Employee) -> u32 {
        let present_id = self.next_id;

        let new_employee = Employee {
            id: present_id,
            name: employee.name,
            age: employee.age,
            dept: employee.dept,
            status: EmploymentStatus::Employed,
        };

        self.next_id += 1;
        self.data.push(new_employee);
        present_id

    }

    fn fire_employee(&mut self, id:u32, new_status: EmploymentStatus) -> bool {
        if let Some(employee) = self.data.iter_mut().find(|employee_id| employee_id.id == id) {
            employee.status = new_status;
            true
        } else {
            false
        }

    }

    fn can_access_building(&self, id: u32) -> Result<(), AccessError> {
        let employee = self.data.iter().find(|e| e.id == id)
            .ok_or(AccessError::NotAuthorized)?;

        if employee.status != EmploymentStatus::Employed {
            return Err(AccessError::NotEmployed);
        }

        match employee.dept {
            EmployeeType::MultiMedia | EmployeeType::Ict | EmployeeType::Manager => Ok(()),
            _ => Err(AccessError::NotAuthorized),
        }
    }

    fn enter_building(&self, id: u32) -> Result<(), AccessError> {
        self.can_access_building(id)?; // ? will return early if Err
        println!("Access granted ✅");
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> EmployeeDB {
    let mut db = EmployeeDB::new();

    db.new_employee(Employee {
        id: 0,
        name: "Ada".to_string(),
        age: 25,
        dept: EmployeeType::Manager,
        status: EmploymentStatus::Employed,
    });

    db.new_employee(Employee {
        id: 0,
        name: "Chinedu".to_string(),
        age: 22,
        dept: EmployeeType::SocialMedia,
        status: EmploymentStatus::Employed,
    });

    let tomi_id = db.new_employee(Employee {
        id: 0,
        name: "Tomi".to_string(),
        age: 29,
        dept: EmployeeType::KitchenStaff,
        status: EmploymentStatus::Employed, // gets overwritten anyway
    });

        db.fire_employee(tomi_id, EmploymentStatus::Fired);

        db
    }


    #[test]
    fn test_access_granted() {
        let db = setup();
        assert_eq!(db.can_access_building(1), Ok(()));
    }

    #[test]
    fn test_access_denied_due_to_department() {
        let db = setup();
        assert_eq!(db.can_access_building(2), Err(AccessError::NotAuthorized));
    }

    #[test]
    fn test_access_denied_due_to_termination() {
        let db = setup();
        assert_eq!(db.can_access_building(3), Err(AccessError::NotEmployed));
    }

    #[test]
    fn test_enter_building_success() {
        let db = setup();
        let result = db.enter_building(1);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_enter_building_failure() {
        let db = setup();
        let result = db.enter_building(2);
        assert_eq!(result, Err(AccessError::NotAuthorized));
    }
}
