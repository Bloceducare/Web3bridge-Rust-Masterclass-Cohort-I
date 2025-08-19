use soroban_sdk::{Env, Symbol, contracttype, symbol_short, Vec};
use crate::employee::Employee;

#[contracttype]
pub enum DataKey {
    Employee(u32),
    EmployeeList,
}

pub fn get_employee_key(_env: &Env, id: u32) -> Symbol {
    // Use predefined symbols for common IDs to avoid format! macro
    match id {
        0 => symbol_short!("emp0"),
        1 => symbol_short!("emp1"),
        2 => symbol_short!("emp2"),
        3 => symbol_short!("emp3"),
        4 => symbol_short!("emp4"),
        5 => symbol_short!("emp5"),
        6 => symbol_short!("emp6"),
        7 => symbol_short!("emp7"),
        8 => symbol_short!("emp8"),
        9 => symbol_short!("emp9"),
        _ => symbol_short!("emp_x"), // TODOS: I will implement For ids > 9, use a generic key with DataKey enum instead
    }
}

pub fn set_employee(env: &Env, id: u32, employee: &Employee) {
    let key = DataKey::Employee(id);
    env.storage().persistent().set(&key, employee);
    
    // Update employee list
    let mut employee_list: Vec<u32> = env.storage().persistent().get(&DataKey::EmployeeList).unwrap_or(Vec::new(env));
    if !employee_list.contains(&id) {
        employee_list.push_back(id);
        env.storage().persistent().set(&DataKey::EmployeeList, &employee_list);
    }
}

pub fn get_employee(env: &Env, id: u32) -> Option<Employee> {
    let key = DataKey::Employee(id);
    env.storage().persistent().get(&key)
}

pub fn remove_employee(env: &Env, id: u32) {
    let key = DataKey::Employee(id);
    env.storage().persistent().remove(&key);
    
    // Update employee list
    let mut employee_list: Vec<u32> = env.storage().persistent().get(&DataKey::EmployeeList).unwrap_or(Vec::new(env));
    if let Some(index) = employee_list.iter().position(|&x| x == id) {
        employee_list.remove(index.try_into().unwrap());
        env.storage().persistent().set(&DataKey::EmployeeList, &employee_list);
    }
}

pub fn get_all_employees(env: &Env) -> Vec<(u32, Employee)> {
    let employee_list: Vec<u32> = env.storage().persistent().get(&DataKey::EmployeeList).unwrap_or(Vec::new(env));
    let mut result = Vec::new(env);
    
    for id in employee_list.iter() {
        if let Some(employee) = get_employee(env, id) {
            result.push_back((id, employee));
        }
    }
    
    result
}