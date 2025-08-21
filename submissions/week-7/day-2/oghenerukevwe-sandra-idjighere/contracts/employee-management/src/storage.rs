use soroban_sdk::{contracttype, Address, Env};
use crate::types::{Employee, Institution};

/// Storage keys for the employee management contract
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// Institution information
    Institution,
    /// Employee by ID
    Employee(u64),
    /// Employee ID by address (for reverse lookup)
    EmployeeByAddress(Address),
    /// Next employee ID counter
    NextEmployeeId,
    /// Total number of employees
    EmployeeCount,
    /// Initialization status
    Initialized,
    /// Admin address
    Admin,
}

/// Storage operations for institution
pub fn read_institution(env: &Env) -> Institution {
    let key = DataKey::Institution;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_institution(env: &Env, institution: &Institution) {
    let key = DataKey::Institution;
    env.storage().instance().set(&key, institution);
}

/// Storage operations for employees
pub fn read_employee(env: &Env, employee_id: u64) -> Option<Employee> {
    let key = DataKey::Employee(employee_id);
    env.storage().persistent().get(&key)
}

pub fn write_employee(env: &Env, employee: &Employee) {
    let key = DataKey::Employee(employee.id);
    env.storage().persistent().set(&key, employee);
    
    // Also store reverse lookup (address -> employee_id)
    let address_key = DataKey::EmployeeByAddress(employee.address.clone());
    env.storage().persistent().set(&address_key, &employee.id);
}

pub fn remove_employee(env: &Env, employee_id: u64) {
    if let Some(employee) = read_employee(env, employee_id) {
        let key = DataKey::Employee(employee_id);
        env.storage().persistent().remove(&key);
        
        // Remove reverse lookup
        let address_key = DataKey::EmployeeByAddress(employee.address);
        env.storage().persistent().remove(&address_key);
    }
}

/// Get employee ID by address
pub fn get_employee_id_by_address(env: &Env, address: &Address) -> Option<u64> {
    let key = DataKey::EmployeeByAddress(address.clone());
    env.storage().persistent().get(&key)
}

/// Storage operations for employee ID counter
pub fn get_next_employee_id(env: &Env) -> u64 {
    let key = DataKey::NextEmployeeId;
    let next_id = env.storage().instance().get(&key).unwrap_or(1u64);
    env.storage().instance().set(&key, &(next_id + 1));
    next_id
}

/// Storage operations for employee count
pub fn get_employee_count(env: &Env) -> u64 {
    let key = DataKey::EmployeeCount;
    env.storage().instance().get(&key).unwrap_or(0u64)
}

pub fn increment_employee_count(env: &Env) {
    let key = DataKey::EmployeeCount;
    let count = get_employee_count(env);
    env.storage().instance().set(&key, &(count + 1));
}

pub fn decrement_employee_count(env: &Env) {
    let key = DataKey::EmployeeCount;
    let count = get_employee_count(env);
    if count > 0 {
        env.storage().instance().set(&key, &(count - 1));
    }
}

/// Storage operations for initialization status
pub fn is_initialized(env: &Env) -> bool {
    let key = DataKey::Initialized;
    env.storage().instance().get(&key).unwrap_or(false)
}

pub fn set_initialized(env: &Env) {
    let key = DataKey::Initialized;
    env.storage().instance().set(&key, &true);
}

/// Storage operations for admin
pub fn read_admin(env: &Env) -> Address {
    let key = DataKey::Admin;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_admin(env: &Env, admin: &Address) {
    let key = DataKey::Admin;
    env.storage().instance().set(&key, admin);
}

/// Helper function to check if employee exists
pub fn employee_exists(env: &Env, employee_id: u64) -> bool {
    read_employee(env, employee_id).is_some()
}

/// Helper function to check if address is already an employee
pub fn address_is_employee(env: &Env, address: &Address) -> bool {
    get_employee_id_by_address(env, address).is_some()
}
