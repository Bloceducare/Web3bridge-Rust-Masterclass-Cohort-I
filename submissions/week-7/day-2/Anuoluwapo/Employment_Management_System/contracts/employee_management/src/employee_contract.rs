use soroban_sdk::{
    contract, contractimpl, contractmeta, contracttype,
    Address, Env, String
};

use crate::imports::sep_41::Client as Sep41Client;
use crate::errors::EmployeeError;

contractmeta!(
    key = "Description",
    val = "Employee Management System with SEP-41 Token Integration"
);

contractmeta!(
    key = "version",
    val = "1.0.0"
);

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TokenContract,
    Employee(Address),
    Institution(Address),
    EmployeeCount,
    NextEmployeeId,
}

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum EmployeeRank {
    Intern = 1,
    Junior = 2,
    Mid = 3,
    Senior = 4,
    Lead = 5,
    Manager = 6,
    Director = 7,
    VP = 8,
    CEO = 9,
}

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum EmployeeStatus {
    Active,
    Suspended,
    Terminated,
}

#[derive(Clone)]
#[contracttype]
pub struct Employee {
    pub id: u64,
    pub address: Address,
    pub institution: Address,
    pub name: String,
    pub rank: EmployeeRank,
    pub salary: i128,
    pub status: EmployeeStatus,
    pub hire_date: u64,
    pub last_promotion: Option<u64>,
}

#[derive(Clone)]
#[contracttype]
pub struct Institution {
    pub address: Address,
    pub name: String,
    pub admin: Address,
    pub employee_count: u32,
    pub is_active: bool,
}

#[contract]
pub struct EmployeeManagement;

#[contractimpl]
impl EmployeeManagement {
    pub fn initialize(
        env: Env,
        admin: Address,
        token_contract: Address,
    ) -> Result<(), EmployeeError> {
        if Self::has_admin(&env) {
            return Err(EmployeeError::AlreadyInitialized);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TokenContract, &token_contract);
        env.storage().instance().set(&DataKey::EmployeeCount, &0u32);
        env.storage().instance().set(&DataKey::NextEmployeeId, &1u64);

        Ok(())
    }

    pub fn register_institution(
        env: Env,
        institution_address: Address,
        name: String,
        admin: Address,
    ) -> Result<(), EmployeeError> {
        Self::require_admin(&env)?;
        
        if name.len() > 50 {
            return Err(EmployeeError::InvalidRank);
        }

        let institution = Institution {
            address: institution_address.clone(),
            name,
            admin,
            employee_count: 0,
            is_active: true,
        };

        env.storage().persistent().set(&DataKey::Institution(institution_address.clone()), &institution);
        env.storage().persistent().extend_ttl(&DataKey::Institution(institution_address), 100, 100);

        Ok(())
    }

    pub fn add_employee(
        env: Env,
        employee_address: Address,
        institution: Address,
        name: String,
        rank: EmployeeRank,
        salary: i128,
    ) -> Result<u64, EmployeeError> {
        let inst_data = Self::get_institution(&env, institution.clone())?;
        inst_data.admin.require_auth();

        if !inst_data.is_active {
            return Err(EmployeeError::InstitutionNotActive);
        }

        if env.storage().persistent().has(&DataKey::Employee(employee_address.clone())) {
            return Err(EmployeeError::EmployeeAlreadyExists);
        }

        if salary <= 0 {
            return Err(EmployeeError::InvalidSalary);
        }

        if name.len() > 50 {
            return Err(EmployeeError::InvalidRank);
        }

        let employee_id = env.storage().instance().get(&DataKey::NextEmployeeId).unwrap_or(1u64);
        let current_time = env.ledger().timestamp();

        let employee = Employee {
            id: employee_id,
            address: employee_address.clone(),
            institution: institution.clone(),
            name,
            rank,
            salary,
            status: EmployeeStatus::Active,
            hire_date: current_time,
            last_promotion: None,
        };

        env.storage().persistent().set(&DataKey::Employee(employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&DataKey::Employee(employee_address), 100, 100);

        env.storage().instance().set(&DataKey::NextEmployeeId, &(employee_id + 1));
        
        let mut updated_inst = inst_data;
        updated_inst.employee_count += 1;
        env.storage().persistent().set(&DataKey::Institution(institution.clone()), &updated_inst);
        env.storage().persistent().extend_ttl(&DataKey::Institution(institution), 100, 100);

        let total_count: u32 = env.storage().instance().get(&DataKey::EmployeeCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::EmployeeCount, &(total_count + 1));

        Ok(employee_id)
    }

    pub fn update_employee(
        env: Env,
        employee_address: Address,
        name: Option<String>,
        salary: Option<i128>,
    ) -> Result<(), EmployeeError> {
        let mut employee = Self::get_employee(&env, employee_address.clone())?;
        
        let inst_data = Self::get_institution(&env, employee.institution.clone())?;
        inst_data.admin.require_auth();

        if let Some(new_name) = name {
            if new_name.len() > 50 {
                return Err(EmployeeError::InvalidRank);
            }
            employee.name = new_name;
        }

        if let Some(new_salary) = salary {
            if new_salary <= 0 {
                return Err(EmployeeError::InvalidSalary);
            }
            employee.salary = new_salary;
        }

        env.storage().persistent().set(&DataKey::Employee(employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&DataKey::Employee(employee_address), 100, 100);

        Ok(())
    }

    pub fn promote_employee(
        env: Env,
        employee_address: Address,
        new_rank: EmployeeRank,
        new_salary: i128,
    ) -> Result<(), EmployeeError> {
        let mut employee = Self::get_employee(&env, employee_address.clone())?;
        
        let inst_data = Self::get_institution(&env, employee.institution.clone())?;
        inst_data.admin.require_auth();

        if employee.status != EmployeeStatus::Active {
            return Err(EmployeeError::EmployeeNotActive);
        }

        let current_rank_value = employee.rank.clone() as u32;
        let new_rank_value = new_rank.clone() as u32;
        
        if new_rank_value <= current_rank_value {
            return Err(EmployeeError::InvalidRank);
        }

        if new_salary <= employee.salary {
            return Err(EmployeeError::InvalidSalary);
        }

        employee.rank = new_rank;
        employee.salary = new_salary;
        employee.last_promotion = Some(env.ledger().timestamp());

        env.storage().persistent().set(&DataKey::Employee(employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&DataKey::Employee(employee_address), 100, 100);

        Ok(())
    }

    pub fn suspend_employee(
        env: Env,
        employee_address: Address,
    ) -> Result<(), EmployeeError> {
        let mut employee = Self::get_employee(&env, employee_address.clone())?;
        
        let inst_data = Self::get_institution(&env, employee.institution.clone())?;
        inst_data.admin.require_auth();

        if employee.status != EmployeeStatus::Active {
            return Err(EmployeeError::EmployeeAlreadySuspended);
        }

        employee.status = EmployeeStatus::Suspended;

        env.storage().persistent().set(&DataKey::Employee(employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&DataKey::Employee(employee_address), 100, 100);

        Ok(())
    }

    pub fn reactivate_employee(
        env: Env,
        employee_address: Address,
    ) -> Result<(), EmployeeError> {
        let mut employee = Self::get_employee(&env, employee_address.clone())?;
        
        let inst_data = Self::get_institution(&env, employee.institution.clone())?;
        inst_data.admin.require_auth();

        if employee.status != EmployeeStatus::Suspended {
            return Err(EmployeeError::EmployeeNotActive);
        }

        employee.status = EmployeeStatus::Active;

        env.storage().persistent().set(&DataKey::Employee(employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&DataKey::Employee(employee_address), 100, 100);

        Ok(())
    }

    pub fn remove_employee(
        env: Env,
        employee_address: Address,
    ) -> Result<(), EmployeeError> {
        let mut employee = Self::get_employee(&env, employee_address.clone())?;
        
        let inst_data = Self::get_institution(&env, employee.institution.clone())?;
        inst_data.admin.require_auth();

        employee.status = EmployeeStatus::Terminated;

        let mut updated_inst = inst_data;
        if updated_inst.employee_count > 0 {
            updated_inst.employee_count -= 1;
        }
        env.storage().persistent().set(&DataKey::Institution(employee.institution.clone()), &updated_inst);
        env.storage().persistent().extend_ttl(&DataKey::Institution(employee.institution.clone()), 100, 100);

        let total_count: u32 = env.storage().instance().get(&DataKey::EmployeeCount).unwrap_or(0);
        if total_count > 0 {
            env.storage().instance().set(&DataKey::EmployeeCount, &(total_count - 1));
        }

        env.storage().persistent().set(&DataKey::Employee(employee_address.clone()), &employee);
        env.storage().persistent().extend_ttl(&DataKey::Employee(employee_address), 100, 100);

        Ok(())
    }

    pub fn pay_salary(
        env: Env,
        employee_address: Address,
    ) -> Result<(), EmployeeError> {
        let employee = Self::get_employee(&env, employee_address.clone())?;
        
        let inst_data = Self::get_institution(&env, employee.institution.clone())?;
        inst_data.admin.require_auth();

        if employee.status != EmployeeStatus::Active {
            return Err(EmployeeError::EmployeeNotActive);
        }

        let token_contract_address = Self::get_token_contract(&env)?;

        let sep41_client = Sep41Client::new(&env, &token_contract_address);
        
        match sep41_client.try_transfer(&employee.institution, &employee.address, &employee.salary) {
            Ok(_) => Ok(()),
            Err(_) => Err(EmployeeError::TokenError),
        }
    }

    pub fn get_employee(env: &Env, employee_address: Address) -> Result<Employee, EmployeeError> {
        env.storage()
            .persistent()
            .get(&DataKey::Employee(employee_address))
            .ok_or(EmployeeError::EmployeeNotFound)
    }

    pub fn get_institution(env: &Env, institution_address: Address) -> Result<Institution, EmployeeError> {
        env.storage()
            .persistent()
            .get(&DataKey::Institution(institution_address))
            .ok_or(EmployeeError::InstitutionNotFound)
    }

    pub fn view_employee(env: Env, employee_address: Address) -> Result<Employee, EmployeeError> {
        Self::get_employee(&env, employee_address)
    }

    pub fn view_institution(env: Env, institution_address: Address) -> Result<Institution, EmployeeError> {
        Self::get_institution(&env, institution_address)
    }

    pub fn get_employee_count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::EmployeeCount).unwrap_or(0)
    }

    pub fn get_admin(env: Env) -> Result<Address, EmployeeError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(EmployeeError::NotInitialized)
    }

    pub fn get_token_contract(env: &Env) -> Result<Address, EmployeeError> {
        env.storage()
            .instance()
            .get(&DataKey::TokenContract)
            .ok_or(EmployeeError::NotInitialized)
    }

    fn has_admin(env: &Env) -> bool {
        env.storage().instance().has(&DataKey::Admin)
    }

    fn require_admin(env: &Env) -> Result<(), EmployeeError> {
        let admin = Self::get_admin(env.clone())?;
        admin.require_auth();
        Ok(())
    }
}