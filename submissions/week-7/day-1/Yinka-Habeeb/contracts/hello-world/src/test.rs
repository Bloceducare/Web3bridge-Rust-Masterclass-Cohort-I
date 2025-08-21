use soroban_sdk::{testutils::Address as _, Address, Env, String};
use crate::{employee::EmployeeContract, token::Token};


#[cfg(test)]
#[test]
fn test_employee_pay() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let emp = Address::generate(&env);

    let token_id = env.register_contract(None, Token);
    let token_client = crate::token::TokenClient::new(&env, &token_id);

    token_client.mint(&admin, &1000);

    let emp_id = env.register_contract(None, EmployeeContract);
    let emp_client = crate::employee::EmployeeContractClient::new(&env, &emp_id);

    emp_client.init(&admin, &token_id);
    emp_client.add_employee(&admin, &String::from_str(&env, "Yinka"), &emp, &100);

    emp_client.pay_employee(&admin, &emp);

    assert_eq!(token_client.balance_of(&emp), 100);
}
