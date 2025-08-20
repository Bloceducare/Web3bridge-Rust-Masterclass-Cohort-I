#[cfg(test)]
mod tests {
    use crate::{
        sep41::{Sep41, Sep41Client}, 
        traits::EmployeeInfo, 
        EmployeeSystem, EmployeeSystemClient
    };
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    fn setup() -> (Env, EmployeeSystemClient<'static>, Sep41Client<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let token_contract_id = env.register_contract(None, Sep41);
        let token_client = Sep41Client::new(&env, &token_contract_id);

        let ems_contract_id = env.register_contract(None, EmployeeSystem);
        let ems_client = EmployeeSystemClient::new(&env, &ems_contract_id);

        let admin = Address::generate(&env);

        token_client.initialize(&admin);
        ems_client.initialize(&admin, &token_contract_id);

        (env, ems_client, token_client, admin)
    }

    #[test]
    fn test_initialize() {
        let (env, ems_client, token_client, admin) = setup();

        let stored_admin = ems_client.get_admin();
        let stored_token = ems_client.get_token_contract();

        assert_eq!(stored_admin, admin);
        assert_eq!(stored_token, token_client.address);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_initialize_twice() {
        let (env, ems_client, token_client, admin) = setup();

        ems_client.initialize(&admin, &token_client.address);
    }

    #[test]
    fn test_add_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        let employee_info = ems_client.get_employee(&employee).unwrap();
        assert_eq!(employee_info.rank, rank);
        assert_eq!(employee_info.salary, 5000);
        assert!(employee_info.is_active);
    }

    #[test]
    #[should_panic(expected = "employee already exists")]
    fn test_add_employee_twice() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);
        ems_client.add_employee(&employee, &rank, &6000);
    }

    #[test]
    #[should_panic(expected = "salary cannot be negative")]
    fn test_add_employee_negative_salary() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &-1000);
    }

    #[test]
    fn test_remove_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        ems_client.remove_employee(&employee);

        let employee_info = ems_client.get_employee(&employee);
        assert!(employee_info.is_none());
    }

    #[test]
    #[should_panic(expected = "employee not found")]
    fn test_remove_nonexistent_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        ems_client.remove_employee(&employee);
    }

    #[test]
    fn test_update_salary() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        ems_client.update_salary(&employee, &6000);

        let employee_info = ems_client.get_employee(&employee).unwrap();
        assert_eq!(employee_info.salary, 6000);
        assert_eq!(employee_info.rank, rank);
    }

    #[test]
    fn test_promote_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let old_rank = String::from_str(&env, "Developer");
        let new_rank = String::from_str(&env, "Senior Developer");

        ems_client.add_employee(&employee, &old_rank, &5000);
        ems_client.promote_employee(&employee, &new_rank, &7000);

        let employee_info = ems_client.get_employee(&employee).unwrap();
        assert_eq!(employee_info.rank, new_rank);
        assert_eq!(employee_info.salary, 7000);
    }

    #[test]
    fn test_suspend_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        assert!(ems_client.is_active(&employee));

        ems_client.suspend_employee(&employee);

        assert!(!ems_client.is_active(&employee));

        let employee_info = ems_client.get_employee(&employee).unwrap();
        assert!(!employee_info.is_active);
    }

    #[test]
    fn test_reactivate_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        ems_client.suspend_employee(&employee);
        assert!(!ems_client.is_active(&employee));

        ems_client.reactivate_employee(&employee);
        assert!(ems_client.is_active(&employee));
    }

    #[test]
    fn test_pay_salary() {
        let (env, ems_client, token_client, admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        token_client.mint(&admin, &10000);

        let admin_balance_before = token_client.balance(&admin);
        let employee_balance_before = token_client.balance(&employee);

        ems_client.pay_salary(&employee);

        let admin_balance_after = token_client.balance(&admin);
        let employee_balance_after = token_client.balance(&employee);

        assert_eq!(admin_balance_after, admin_balance_before - 5000);
        assert_eq!(employee_balance_after, employee_balance_before + 5000);
    }

    #[test]
    #[should_panic(expected = "employee is suspended")]
    fn test_pay_salary_suspended_employee() {
        let (env, ems_client, token_client, admin) = setup();
        let employee = Address::generate(&env);

        let rank = String::from_str(&env, "Developer");
        ems_client.add_employee(&employee, &rank, &5000);

        ems_client.suspend_employee(&employee);

        token_client.mint(&admin, &10000);
        ems_client.pay_salary(&employee);
    }

    #[test]
    fn test_is_active_nonexistent_employee() {
        let (env, ems_client, _token_client, _admin) = setup();
        let employee = Address::generate(&env);

        assert!(!ems_client.is_active(&employee));
    }

    #[test]
    fn test_set_admin() {
        let (env, ems_client, _token_client, admin) = setup();
        let new_admin = Address::generate(&env);

        ems_client.set_admin(&new_admin);

        let stored_admin = ems_client.get_admin();
        assert_eq!(stored_admin, new_admin);
    }

    #[test]
    fn test_multiple_employees() {
        let (env, ems_client, token_client, admin) = setup();
        let employee1 = Address::generate(&env);
        let employee2 = Address::generate(&env);

        let dev_rank = String::from_str(&env, "Developer");
        let manager_rank = String::from_str(&env, "Manager");

        ems_client.add_employee(&employee1, &dev_rank, &5000);
        ems_client.add_employee(&employee2, &manager_rank, &8000);

        token_client.mint(&admin, &20000);

        ems_client.pay_salary(&employee1);
        ems_client.pay_salary(&employee2);

        assert_eq!(token_client.balance(&employee1), 5000);
        assert_eq!(token_client.balance(&employee2), 8000);
        assert_eq!(token_client.balance(&admin), 7000);

        assert!(ems_client.is_active(&employee1));
        assert!(ems_client.is_active(&employee2));
    }

    #[test]
    fn test_complete_employee_lifecycle() {
        let (env, ems_client, token_client, admin) = setup();
        let employee = Address::generate(&env);

        let junior_rank = String::from_str(&env, "Junior Developer");
        let senior_rank = String::from_str(&env, "Senior Developer");

        ems_client.add_employee(&employee, &junior_rank, &4000);

        token_client.mint(&admin, &20000);
        ems_client.pay_salary(&employee);
        assert_eq!(token_client.balance(&employee), 4000);

        ems_client.promote_employee(&employee, &senior_rank, &6000);
        ems_client.pay_salary(&employee);
        assert_eq!(token_client.balance(&employee), 10000);

        ems_client.suspend_employee(&employee);
        assert!(!ems_client.is_active(&employee));

        ems_client.reactivate_employee(&employee);
        assert!(ems_client.is_active(&employee));

        ems_client.update_salary(&employee, &7000);
        ems_client.pay_salary(&employee);
        assert_eq!(token_client.balance(&employee), 17000);

        let final_info = ems_client.get_employee(&employee).unwrap();
        assert_eq!(final_info.rank, senior_rank);
        assert_eq!(final_info.salary, 7000);
        assert!(final_info.is_active);
    }
}
