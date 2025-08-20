use soroban_sdk::{symbol_short, Address, Env};

pub fn register_event(env: &Env, employee: Address, employee_id: u32, salary: u64) {
    let topics = (symbol_short!("add"), employee);
    env.events().publish(topics, (employee_id, salary));
}

pub fn remove_event(env: &Env, employee: Address) {
    let topics = (symbol_short!("remove"), employee);
    env.events().publish(topics, ());
}

pub fn promote_event(env: &Env, admin: Address, employee: Address) {
    let topics = (symbol_short!("promote"), admin, employee);
    env.events().publish(topics, ());
}

pub fn suspend_event(env: &Env, admin: Address, employee: Address) {
    let topics = (symbol_short!("suspend"), admin, employee);
    env.events().publish(topics, ());
}

pub fn unsuspend_event(env: &Env, admin: Address, employee: Address) {
    let topics = (symbol_short!("unsuspnd"), admin, employee);
    env.events().publish(topics, ());
}

pub fn set_admin_event(env: &Env, old_admin: Address, new_admin: Address) {
    let topics = (symbol_short!("setadmin"), old_admin);
    env.events().publish(topics, new_admin);
}

pub fn salary_update_event(env: &Env, admin: Address, employee: Address, old_salary: u64, new_salary: u64) {
    let topics = (symbol_short!("salary"), admin, employee);
    env.events().publish(topics, (old_salary, new_salary));
}

pub fn payment_event(env: &Env, employee: Address, amount: u64, ledger: u64) {
    let topics = (symbol_short!("payment"), employee);
    env.events().publish(topics, (amount, ledger));
}