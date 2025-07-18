use crate::employee_struct::{Employee, EmployeeDB};
use crate::employee_enum::{EmployeeType, EmploymentStatus};

pub mod employee_struct;
pub mod employee_enum;
pub mod employee_state;

fn main() {
     let mut db = EmployeeDB::new();

     let tomi_id = db.new_employee(Employee {
        id: 0,
        name: "Tomi".to_string(),
        age: 29,
        dept: EmployeeType::KitchenStaff,
        status: EmploymentStatus::Employed, 
    });

    db.fire_employee(tomi_id, EmploymentStatus::Fired);

    match db.enter_building(tomi_id) {
        Ok(_) => (),
        Err(e) => println!("Access denied ❌: {:?}", e),
    }
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
        status: EmploymentStatus::Employed, 
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
