use soroban_sdk::{contract, contractimpl, Address, Env};
use crate::{
    admin,
    error::EmsError,
    events,
    storage,
    types::{Employee, EmployeeRank},
};

use crate::import::sep41_token::Client;

#[contract]
pub struct Ems;

#[contractimpl]
impl Ems {
  pub fn initialize(env: Env, admin: Address, token_address: Address) -> Result<(), EmsError> {
        if admin::has_administrator(&env) {
            return Err(EmsError::AlreadyRegistered);
        }

        // check if sep token is initialized
        let token_client = Client::new(&env, &token_address);
        if !token_client.has_admin() {
          return Err(EmsError::NotInitialized);
        }

        // set sep token address
        storage::set_sep_token_address(&env, &token_address);
        
        admin::write_administrator(&env, &admin);
        storage::extend_instance(&env);
        Ok(())
    }

    pub fn add_employee(env: Env, employee_address: Address, rank: EmployeeRank, weekly_salary: u64 ) -> Result<(), EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;

      // check if employee already exists
      if storage::has_employee(&env, &employee_address) {
        return Err(EmsError::AlreadyRegistered);
      }


      let employee_id = storage::get_next_employee_id(&env);

      let employee = Employee {
        address: employee_address.clone(),
        employee_id,
        rank,
        weekly_salary,
        is_suspended: false,
        is_active: true
      };

      storage::set_employee(&env, &employee);
      events::register_event(&env, employee_address, employee_id, weekly_salary);
      Ok(())
    }

    pub fn remove_employee(env: Env, employee_address: Address) -> Result<(), EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;

      // check if employee exists
      if !storage::has_employee(&env, &employee_address) {
        return Err(EmsError::NotRegistered);
      }
      
     storage::remove_employee(&env, &employee_address);
      events::remove_event(&env, employee_address);
     storage::extend_instance(&env);
      Ok(())
    }


    pub fn update_salary(env: Env, employee_address: Address, new_salary: u64) -> Result<(), EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;


      let mut employee = storage::get_employee(&env, &employee_address).ok_or(EmsError::NotRegistered)?;

      // check if employee is active
      if !employee.is_active {
        return Err(EmsError::NotActive);
      }


      let old_salary = employee.weekly_salary;
      employee.weekly_salary = new_salary;

      storage::set_employee(&env, &employee);
      events::salary_update_event(&env, admin, employee_address, old_salary, new_salary);
      storage::extend_instance(&env);
      Ok(())  
    }

    pub fn promote_employee(env: Env, employee_address: Address, new_rank: EmployeeRank) -> Result<(), EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;


      let mut employee = storage::get_employee(&env, &employee_address).ok_or(EmsError::NotRegistered)?;

      // check if employee is active
      if !employee.is_active {
        return Err(EmsError::NotActive);
      }

      if employee.is_suspended {
        return Err(EmsError::AlreadySuspended);
      }

      if new_rank == employee.rank {
        return Err(EmsError::AlreadyPromoted);
      }

      employee.rank = new_rank;

      storage::set_employee(&env, &employee);
      events::promote_event(&env, admin, employee_address);
      storage::extend_instance(&env);
      Ok(())
    }

    pub fn suspend_employee(env: Env, employee_address: Address) -> Result<(), EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;

      let mut employee = storage::get_employee(&env, &employee_address).ok_or(EmsError::NotRegistered)?;

      // check if employee is active
      if !employee.is_active {
        return Err(EmsError::NotActive);
      }

      if employee.is_suspended {
        return Err(EmsError::AlreadySuspended);
      }

      employee.is_suspended = true;
      storage::set_employee(&env, &employee);
      events::suspend_event(&env, admin, employee_address);
      storage::extend_instance(&env);
      Ok(())
    }


    pub fn unsuspend_employee(env: Env, employee_address: Address) -> Result<(), EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;
      

      let mut employee = storage::get_employee(&env, &employee_address).ok_or(EmsError::NotRegistered)?;

       if !employee.is_active {
            return Err(EmsError::NotActive);
        }
        if !employee.is_suspended {
            return Err(EmsError::NotSuspended);
        }

        employee.is_suspended = false;
        storage::set_employee(&env, &employee);
        events::unsuspend_event(&env, admin, employee_address);
        storage::extend_instance(&env);
        Ok(())
    }

        pub fn get_employee(env: Env, employee_address: Address) -> Option<Employee> {
        storage::get_employee(&env, &employee_address)
    }

    pub fn employee_exists(env: Env, employee_address: Address) -> bool {
        storage::has_employee(&env, &employee_address)
    }

    pub fn get_admin(env: Env) -> Result<Address, EmsError> {
        if !admin::has_administrator(&env) {
            return Err(EmsError::NotInitialized);
        }
        Ok(admin::read_administrator(&env))
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), EmsError> {
        admin::set_admin(&env, &new_admin)
    }


    pub fn is_payment_due(
        env: Env,
        employee_address: Address,
    ) -> Result<bool, EmsError> {
        if !storage::has_employee(&env, &employee_address) {
            return Err(EmsError::NotRegistered);
        }

        Ok(storage::is_employee_due_payment(&env, &employee_address))
    }


    pub fn get_payment_info(
        env: Env,
        employee_address: Address,
    ) -> Result<(u64, Option<u64>, bool), EmsError> {
        let employee = storage::get_employee(&env, &employee_address)
            .ok_or(EmsError::NotRegistered)?;

        let last_paid = storage::get_employee_last_paid(&env, &employee_address);
        let is_due = storage::is_employee_due_payment(&env, &employee_address);

        Ok((employee.weekly_salary, last_paid, is_due))
    }


    pub fn pay_employee(env: Env, employee_address: Address) -> Result<bool, EmsError> {
      let admin = admin::read_administrator(&env);
      admin.require_auth();
      admin::check_admin(&env, &admin)?;

      let employee = storage::get_employee(&env, &employee_address).ok_or(EmsError::NotRegistered)?;

      if !employee.is_active || employee.is_suspended {
            return Err(EmsError::NotActive);
        }

     // check if employee is due for payment
     if !storage::is_employee_due_payment(&env, &employee_address) {
      return Err(EmsError::NotDueForPayment);
     }


    // get token address
    let token_address = storage::get_sep_token_address(&env)
        .ok_or(EmsError::NotInitialized)?;

    // get token client
    let token_client = Client::new(&env, &token_address);

      // get employee salary
      let salary = employee.weekly_salary;

      // transfer salary to employee
      token_client.transfer(&admin, &employee_address, &(salary as i128));

      // update last paid date
      let current_ledger = env.ledger().sequence() as u64;
      storage::set_employee_last_paid(&env, &employee_address, current_ledger);

      events::payment_event(&env, employee_address, employee.weekly_salary, current_ledger);
      storage::extend_instance(&env);

      // extend instance
      storage::extend_instance(&env);

      Ok(true)
    }


}