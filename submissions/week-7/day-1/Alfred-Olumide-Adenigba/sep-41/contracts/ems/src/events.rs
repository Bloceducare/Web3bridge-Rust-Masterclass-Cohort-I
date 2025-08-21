use soroban_sdk::{Address, Env, symbol_short};

pub fn employee_added_event(env: &Env, employee_id: Address, institution: Address, salary: i128, rank: u32) {
    let topics = (symbol_short!("emp_add"), employee_id.clone(), institution.clone());
    let data = (salary, rank);
    env.events().publish(topics, data);
}

pub fn employee_removed_event(env: &Env, employee_id: Address) {
    let topics = (symbol_short!("emp_rem"), employee_id.clone());
    env.events().publish(topics, ());
}

pub fn employee_updated_event(env: &Env, employee_id: Address, new_salary: i128) {
    let topics = (symbol_short!("emp_upd"), employee_id.clone());
    env.events().publish(topics, new_salary);
}

pub fn employee_promoted_event(env: &Env, employee_id: Address, new_rank: u32) {
    let topics = (symbol_short!("emp_prom"), employee_id.clone());
    env.events().publish(topics, new_rank);
}

pub fn employee_suspended_event(env: &Env, employee_id: Address) {
    let topics = (symbol_short!("emp_susp"), employee_id.clone());
    env.events().publish(topics, ());
}

pub fn employee_reinstated_event(env: &Env, employee_id: Address) {
    let topics = (symbol_short!("emp_rein"), employee_id.clone());
    env.events().publish(topics, ());
}

pub fn salary_paid_event(env: &Env, employee_id: Address, from: Address, amount: i128) {
    let topics = (symbol_short!("sal_paid"), employee_id.clone(), from.clone());
    env.events().publish(topics, amount);
}
