use soroban_sdk::{Env, Address, String, symbol_short, contracttype};
use crate::employee::Rank;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EmployeeAddedEvent {
    pub id: u32,
    pub name: String,
    pub rank: Rank,
    pub salary: i128,
    pub institution: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EmployeeRemovedEvent {
    pub id: u32,
    pub name: String,
    pub institution: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EmployeeUpdatedEvent {
    pub id: u32,
    pub name: String,
    pub salary: i128,
    pub institution: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EmployeePromotedEvent {
    pub id: u32,
    pub name: String,
    pub rank: Rank,
    pub salary: i128,
    pub institution: Address,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct EmployeeSuspendedEvent {
    pub id: u32,
    pub name: String,
    pub institution: Address,
}

pub fn emit_employee_added(env: &Env, id: u32, name: &String, rank: &Rank, salary: i128, institution: &Address) {
    let event = EmployeeAddedEvent {
        id,
        name: name.clone(),
        rank: rank.clone(),
        salary,
        institution: institution.clone(),
    };
    env.events().publish((symbol_short!("emp_added"),), event);
}

pub fn emit_employee_removed(env: &Env, id: u32, name: &String, institution: &Address) {
    let event = EmployeeRemovedEvent {
        id,
        name: name.clone(),
        institution: institution.clone(),
    };
    env.events().publish((symbol_short!("emp_rmvd"),), event);
}

pub fn emit_employee_updated(env: &Env, id: u32, name: &String, salary: i128, institution: &Address) {
    let event = EmployeeUpdatedEvent {
        id,
        name: name.clone(),
        salary,
        institution: institution.clone(),
    };
    env.events().publish((symbol_short!("emp_updt"),), event);
}

pub fn emit_employee_promoted(env: &Env, id: u32, name: &String, rank: &Rank, salary: i128, institution: &Address) {
    let event = EmployeePromotedEvent {
        id,
        name: name.clone(),
        rank: rank.clone(),
        salary,
        institution: institution.clone(),
    };
    env.events().publish((symbol_short!("emp_prmt"),), event);
}

pub fn emit_employee_suspended(env: &Env, id: u32, name: &String, institution: &Address) {
    let event = EmployeeSuspendedEvent {
        id,
        name: name.clone(),
        institution: institution.clone(),
    };
    env.events().publish((symbol_short!("emp_susp"),), event);
}