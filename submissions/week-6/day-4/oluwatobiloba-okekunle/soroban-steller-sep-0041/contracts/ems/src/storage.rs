use soroban_sdk::{contracttype, Address, Env};
use crate::types::{Employee};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    NextId,
    Employee(Address),
    SepTokenAddress,
    LastPayrollRun,
    EmployeeLastPaid(Address),
}

const DAY_IN_LEDGERS: u32 = 17280;
const WEEK_IN_LEDGERS: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_next_employee_id(e: &Env) -> u32 {
    let current_id: u32 = e.storage().instance().get(&DataKey::NextId).unwrap_or(1000);
    let next_id = current_id + 1;
    e.storage().instance().set(&DataKey::NextId, &next_id);
    extend_instance(e);
    current_id
}

pub fn has_employee(e: &Env, address: &Address) -> bool {
    e.storage().persistent().has(&DataKey::Employee(address.clone()))
}

pub fn get_employee(e: &Env, address: &Address) -> Option<Employee> {
    let key = DataKey::Employee(address.clone());
    let result = e.storage().persistent().get(&key);
    if result.is_some() {
        extend_employee_ttl(e, address);
    }
    result
}

pub fn set_employee(e: &Env, employee: &Employee) {
    let key = DataKey::Employee(employee.address.clone());
    e.storage().persistent().set(&key, employee);
    extend_employee_ttl(e, &employee.address);
}

pub fn remove_employee(e: &Env, address: &Address) {
    let key = DataKey::Employee(address.clone());
    e.storage().persistent().remove(&key);
    
    let payroll_key = DataKey::EmployeeLastPaid(address.clone());
    e.storage().persistent().remove(&payroll_key);
}


pub fn set_sep_token_address(e: &Env, token_address: &Address) {
    e.storage().instance().set(&DataKey::SepTokenAddress, token_address);
    extend_instance(e);
}

pub fn get_sep_token_address(e: &Env) -> Option<Address> {
    e.storage().instance().get(&DataKey::SepTokenAddress)
}

pub fn set_last_payroll_run(e: &Env, ledger_timestamp: u64) {
    e.storage().instance().set(&DataKey::LastPayrollRun, &ledger_timestamp);
    extend_instance(e);
}

pub fn get_last_payroll_run(e: &Env) -> Option<u64> {
    e.storage().instance().get(&DataKey::LastPayrollRun)
}

pub fn set_employee_last_paid(e: &Env, address: &Address, ledger_timestamp: u64) {
    let key = DataKey::EmployeeLastPaid(address.clone());
    e.storage().persistent().set(&key, &ledger_timestamp);
    extend_employee_ttl(e, address);
}

pub fn get_employee_last_paid(e: &Env, address: &Address) -> Option<u64> {
    let key = DataKey::EmployeeLastPaid(address.clone());
    let result = e.storage().persistent().get(&key);
    if result.is_some() {
        extend_employee_ttl(e, address);
    }
    result
}

// Check if employee is due for payment (1 week = ~120,960 ledgers)
pub fn is_employee_due_payment(e: &Env, address: &Address) -> bool {
    let current_ledger = e.ledger().sequence() as u64;
    
    if let Some(last_paid) = get_employee_last_paid(e, address) {
        // Check if a week has passed
        current_ledger >= last_paid + WEEK_IN_LEDGERS as u64
    } else {
        // Never been paid, so due for payment
        true
    }
}

pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn extend_employee_ttl(e: &Env, address: &Address) {
    let key = DataKey::Employee(address.clone());
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}