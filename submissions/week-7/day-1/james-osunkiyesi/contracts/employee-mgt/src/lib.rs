#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Address, Env, String, Timepoint, Vec};
use crate::storage::{DataKey, Employee, EmployeeDept, EmployeeRank, EmployeeStatus};

#[contract]
pub struct Contract;


#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
    pub fn init(env: Env, _admin: Address) {
       let admin_opt: Option<Address> = env.storage().persistent().get(&DataKey::Admin);
       match admin_opt {
           Some(_) => panic!("Admin already set"),
           None => env.storage().persistent().set(&DataKey::Admin, &_admin)
       }
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
                let next_rank = EmployeeRank::match_rank(rank + 1).unwrap();

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
