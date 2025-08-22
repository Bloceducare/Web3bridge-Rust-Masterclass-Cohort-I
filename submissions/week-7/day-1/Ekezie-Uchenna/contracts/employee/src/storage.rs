// src/storage.rs

use soroban_sdk::{ contracttype, Address, Env, Vec, String };
use crate::types::{ Employee };
use crate::errors::ContractErrors;

/// Storage keys for the contract data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Employee(Address),
    Institution(Address),
    InstitutionEmployees(Address),
}

/// Storage abstraction layer
pub struct Storage;

impl Storage {
    /// Admin management
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&DataKey::Admin, admin);
    }

    pub fn get_admin(env: &Env) -> Result<Address, ContractErrors> {
        env.storage().instance().get(&DataKey::Admin).ok_or(ContractErrors::AdminNotSet)
    }

    pub fn has_admin(env: &Env) -> bool {
        env.storage().instance().has(&DataKey::Admin)
    }

    /// Employee management
    pub fn set_employee(env: &Env, employee_id: &Address, employee: &Employee) {
        env.storage().persistent().set(&DataKey::Employee(employee_id.clone()), employee);
    }

    pub fn get_employee(env: &Env, employee_id: &Address) -> Result<Employee, ContractErrors> {
        env.storage()
            .persistent()
            .get(&DataKey::Employee(employee_id.clone()))
            .ok_or(ContractErrors::EmployeeNotFound)
    }

    pub fn has_employee(env: &Env, employee_id: &Address) -> bool {
        env.storage().persistent().has(&DataKey::Employee(employee_id.clone()))
    }

    pub fn remove_employee(env: &Env, employee_id: &Address) {
        env.storage().persistent().remove(&DataKey::Employee(employee_id.clone()));
    }

    /// Institution management
    pub fn set_institution(env: &Env, institution_id: &Address, institution: String) {
        env.storage().persistent().set(&DataKey::Institution(institution_id.clone()), &institution);

        // Initialize empty employee list if it doesn't exist
        if !env.storage().persistent().has(&DataKey::InstitutionEmployees(institution_id.clone())) {
            let empty_employees: Vec<Address> = Vec::new(env);
            env.storage()
                .persistent()
                .set(&DataKey::InstitutionEmployees(institution_id.clone()), &empty_employees);
        }
    }

    pub fn get_institution(env: &Env, institution_id: &Address) -> Result<String, ContractErrors> {
        env.storage()
            .persistent()
            .get(&DataKey::Institution(institution_id.clone()))
            .ok_or(ContractErrors::InstitutionNotFound)
    }

    pub fn has_institution(env: &Env, institution_id: &Address) -> bool {
        env.storage().persistent().has(&DataKey::Institution(institution_id.clone()))
    }

    /// Institution-Employee relationship management
    pub fn add_employee_to_institution(env: &Env, institution_id: &Address, employee_id: &Address) {
        let mut employees = Self::get_institution_employees(env, institution_id);
        employees.push_back(employee_id.clone());
        env.storage()
            .persistent()
            .set(&DataKey::InstitutionEmployees(institution_id.clone()), &employees);

        // Update employee count
        if let Ok(mut institution) = Self::get_institution(env, institution_id) {
            institution.employee_count += 1;
            Self::set_institution(env, institution_id, institution);
        }
    }

    pub fn remove_employee_from_institution(
        env: &Env,
        institution_id: &Address,
        employee_id: &Address
    ) {
        let mut employees = Self::get_institution_employees(env, institution_id);

        // Find and remove the employee
        for i in 0..employees.len() {
            if employees.get(i).unwrap() == *employee_id {
                employees.remove(i);
                break;
            }
        }

        env.storage()
            .persistent()
            .set(&DataKey::InstitutionEmployees(institution_id.clone()), &employees);

        // Update employee count
        if let Ok(mut institution) = Self::get_institution(env, institution_id) {
            if institution.employee_count > 0 {
                institution.employee_count -= 1;
            }
            Self::set_institution(env, institution_id, institution);
        }
    }

    pub fn get_institution_employees(env: &Env, institution_id: &Address) -> Vec<Address> {
        env.storage()
            .persistent()
            .get(&DataKey::InstitutionEmployees(institution_id.clone()))
            .unwrap_or_else(|| Vec::new(env))
    }

    /// Utility functions
    pub fn employee_exists_in_institution(
        env: &Env,
        institution_id: &Address,
        employee_id: &Address
    ) -> bool {
        let employees = Self::get_institution_employees(env, institution_id);
        for i in 0..employees.len() {
            if employees.get(i).unwrap() == *employee_id {
                return true;
            }
        }
        false
    }

    /// Get total number of employees across all institutions
    pub fn get_total_employee_count(env: &Env) -> u32 {
        // This would require iterating through all institutions
        // For now, we'll keep it simple and just count active employees per institution
        // In a full implementation, you might want to maintain a global counter
        0
    }

    /// Backup and migration utilities (for future use)
    pub fn backup_employee_data(
        env: &Env,
        employee_id: &Address
    ) -> Result<Employee, ContractErrors> {
        Self::get_employee(env, employee_id)
    }

    pub fn backup_institution_data(
        env: &Env,
        institution_id: &Address
    ) -> Result<String, ContractErrors> {
        Self::get_institution(env, institution_id)
    }
}
