#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Timepoint, Vec};
use storage::{DataKey, Employee, EmployeeDept, EmployeeRank, EmployeeStatus};

pub mod token {
    soroban_sdk::contractimport!(file ="../../target/wasm32v1-none/release/sep_41_token.wasm");
}

use token::Client as TokenClient;

#[contract]
pub struct Contract;

pub fn payment_token(env: Env) -> (Env, Address) {
    let token = env.storage().persistent().get::<DataKey, Address>(&DataKey::PaymentToken);
    match token {
        Some(x) => (env, x),
        None => panic!("Payment Token has not been set")
    }
}

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }


    pub fn init(env: Env, _admin: Address, token_id: Address) {
       let admin_opt: Option<Address> = env.storage().persistent().get(&DataKey::Admin);
       match admin_opt {
           Some(_) => panic!("Admin already set"),
           None => {
               env.storage().persistent().set(&DataKey::Admin, &_admin);
               let token = TokenClient::new(&env, &token_id);
               if token.check_is_initialized() {
                   panic!("Associated Token Already initialized");
               }
               token.init(&env.current_contract_address()); // This contract will be the Token Admin

               env.storage().persistent().set(&DataKey::PaymentToken, &token_id);
           }
       }
    }

    pub fn test_token_transfer(env: Env, to: Address, amount: i128) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());
        let (env, payment_token) = payment_token(env);
        let payment_token = TokenClient::new(&env, &payment_token);

        payment_token.transfer(&env.current_contract_address(), &to, &amount)
    }
    pub fn add_employee(env: Env, user: Address, name: String, rank: EmployeeRank, dept: EmployeeDept) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            Some(_) => panic!("Employee data already exists"),
            None => env.storage().persistent().set(&employee_key, &Employee {
                name,
                rank,
                dept,
                time_employed: Timepoint::from_unix(&env, env.ledger().timestamp()),
                time_since_last_pay: Timepoint::from_unix(&env, env.ledger().timestamp()),
                status: EmployeeStatus::ACTIVE,
            })
        }
    }

    pub fn remove_employee(env: Env, user: Address) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(_) => env.storage().persistent().remove(&employee_key)
        }
    }

    pub fn promote_employee(env: Env, user: Address) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(x) => {
                let rank = x.rank as u32;
                let next_rank = rank + 1;
                if next_rank > EmployeeRank::MANAGER as u32 {
                    panic!("Max Rank Reached");
                };
                let next_rank = EmployeeRank::match_rank(next_rank).unwrap();

                env.storage().persistent().set(&employee_key, &Employee {
                    rank: next_rank,
                    ..x
                })
            }
        }
    }

    pub fn suspend_employee(env: Env, user: Address, time_in_days: u64) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(x) => {
                let duration = env.ledger().timestamp() + (time_in_days * 86_400);
                env.storage().persistent().set(&employee_key, &Employee {
                    status: EmployeeStatus::SUSPENDED(duration),
                    ..x
                })
            }
        }

    }

    pub fn collect_pay(env: Env, user: Address) {
        Self::auth_user(user.clone());

        let employee_key = DataKey::Employee(user.clone());
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(x) => {
               if  x.status.check_is_active(env.ledger().timestamp()) {
                   let time_since_last_pay = x.time_since_last_pay.to_unix();
                   let current_time = env.ledger().timestamp();
                   let diff_in_days = (current_time - time_since_last_pay) / 86_400;
                   if diff_in_days >= 28 {
                       env.storage().persistent().set(&employee_key, &Employee {
                           time_since_last_pay: Timepoint::from_unix(&env,time_since_last_pay + 28),
                           status: EmployeeStatus::ACTIVE,
                           rank: x.rank.clone(),
                           ..x
                       });

                       let (env, payment_token) = payment_token(env);
                       let payment_token = TokenClient::new(&env, &payment_token);

                       payment_token.transfer(&env.current_contract_address(), &user, &EmployeeRank::get_pay(x.rank))

                   } else {
                       panic!("Payday has not reached");
                   }
               }
            }
        }
    }

    // Placeholder function to test employee's access to the company
    pub fn employee_action(env: Env, user: Address) -> bool {
        Self::auth_user(user.clone());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);
        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(x) => {
               let is_active = x.status.check_is_active(env.ledger().timestamp());
                if is_active {
                    env.storage().persistent().set(&employee_key, &Employee {
                        status: EmployeeStatus::ACTIVE,
                        ..x
                    });

                    // Perform any company action
                    return true
                }
            }
        }

        false
    }

    pub fn update_employee_dept(env: Env, user: Address, dept: EmployeeDept) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(x) => {
                env.storage().persistent().set(&employee_key, &Employee {
                    dept,
                    ..x
                })
            }
        }
    }

    pub fn update_employee_name(env: Env, user: Address, name: String) {
        Self::auth_user(env.storage().persistent().get(&DataKey::Admin).unwrap());

        let employee_key = DataKey::Employee(user);
        let employee_opt: Option<Employee> = env.storage().persistent().get(&employee_key);

        match employee_opt {
            None => panic!("Employee data does not exists"),
            Some(x) => {
                env.storage().persistent().set(&employee_key, &Employee {
                    name,
                    ..x
                })
            }
        }
    }


    // ================================
    // ==== INTERNAL FUNCTION
    fn auth_user(admin: Address) {
        admin.require_auth()
    }


}

mod test;
mod storage;