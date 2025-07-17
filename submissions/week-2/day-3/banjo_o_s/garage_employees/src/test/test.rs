

#[cfg(test)]
pub mod test {
    use crate::employees::garage::{OtherEmployeeRole, EmployeeRole, Garage, Employee};

    fn set_up_garage() -> Garage {
        Garage::new("w3b".to_string())
    }

    #[test]
    fn test_add_employee() {
        let mut garage: Garage = set_up_garage();

        garage.add_employee("name".to_string(), EmployeeRole::IT);

        assert_eq!(garage.id, 2);
    }

    #[test]
    fn test_add_multiple_employees() {
        let mut garage: Garage = set_up_garage();
        garage.add_employee("name".to_string(), EmployeeRole::IT);
        garage.add_employee("name".to_string(), EmployeeRole::Manager);
        garage.add_employee("name".to_string(), EmployeeRole::OtherEmployeeRole(2));

        assert_eq!(garage.id, 4);

        let found_employee: Option<&Employee> = garage.get_employee(2);
        assert_eq!(found_employee.is_some(), true);
        assert_eq!(found_employee.unwrap().role, EmployeeRole::Manager);
        assert_eq!(garage.other_roles.contains_key(&3_u128), true);
        assert_eq!(garage.other_roles.get(&3_u128).unwrap().role, OtherEmployeeRole::SocialMedia);
    }

    #[test]
    fn test_change_employee_role() {
        let mut garage: Garage = set_up_garage();
        garage.add_employee("name".to_string(), EmployeeRole::IT);

        let role: EmployeeRole = EmployeeRole::OtherEmployeeRole(3);
        garage.change_employee_role(1, role).expect("e didn't dey");

        assert_eq!(garage.other_roles.get(&1_u128).unwrap().role, OtherEmployeeRole::Technician);

        let role: EmployeeRole = EmployeeRole::OtherEmployeeRole(2);
        garage.add_employee("name".to_string(), role);

        assert_eq!(garage.other_roles.get(&2_u128).unwrap().role, OtherEmployeeRole::SocialMedia);
        assert_eq!(garage.other_roles.get(&2_u128).is_some(), true);

        let role: EmployeeRole = EmployeeRole::IT;
        garage.change_employee_role(2, role).expect("e didn't dey");

        assert_eq!(garage.other_roles.get(&2_u128).is_none(), true);
    }

    #[test]
    fn test_terminate_employee() {
        let mut garage: Garage = set_up_garage();
        garage.add_employee("name".to_string(), EmployeeRole::IT);

        garage.terminate_employee(1).expect("e didn't dey");

        let is_terminated = garage.get_employee(1).unwrap().status.is_terminated();
        assert_eq!(is_terminated, true);
    }

    #[test]
    #[should_panic(expected = "e didn't dey")]
    fn test_cannot_get_employee_that_is_not_registered() {
        let mut garage: Garage = set_up_garage();
        garage.terminate_employee(1).expect("e didn't dey");
    }

    #[test]
    fn test_check_employee_access() {
        let mut garage: Garage = set_up_garage();
        garage.add_employee("name".to_string(), EmployeeRole::IT);

        assert_eq!(garage.has_access(1), true);
        let e = garage.get_employee(1).unwrap();
        println!("employee :: before{:#?}", e);
        garage.change_employee_role(1, EmployeeRole::OtherEmployeeRole(2)).expect("e didn't dey");
        let e = garage.get_employee(1).unwrap();
        println!("employee :: before{:#?}", e);
        assert_eq!(garage.has_access(1), false);

        garage.add_employee("name".to_string(), EmployeeRole::IT);
        garage.terminate_employee(2).expect("e didn't dey");
        assert_eq!(garage.has_access(2), false);
    }
}