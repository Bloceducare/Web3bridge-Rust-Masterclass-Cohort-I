pub mod enums;
pub mod state;

pub use enums::{Department, Status};
pub use state::EmployeeDetails;

pub fn can_access_building(employee: &EmployeeDetails) -> Result<bool, String> {
    if employee.is_terminated == Status::TERMINATED {
        println!("{} is terminated and cannot access the building.", employee.name);
        return Ok(false);
    }

    match employee.department {
        Department::MEDIA | Department::IT | Department::MANAGER => {
          println!("{} from {:?} department can access the building.", employee.name, employee.department);
          return Ok(true);
        }
        Department::SOCIAL_MEDIA | Department::TECHNICIAN_SUPERVISOR | Department::KITCHEN_STAFF => {
          println!("{} from {:?} department can access the building.", employee.name, employee.department);
          return Ok(false);
        }
    }

    Err("Invalid department".to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_can_access_building() {
    let employee1 = EmployeeDetails {
      name: "Alice".to_string(),
      department: Department::MEDIA,
      is_terminated: Status::ACTIVE,
    };

    let employee2 = EmployeeDetails {
      name: "Bob".to_string(),
      department: Department::SOCIAL_MEDIA,
      is_terminated: Status::TERMINATED,
    };

    assert_eq!(can_access_building(&employee1), Ok(true));
    assert_eq!(can_access_building(&employee2), Ok(false));
  }
}