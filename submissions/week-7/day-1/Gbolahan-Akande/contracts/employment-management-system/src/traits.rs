use soroban_sdk::{Address, Env, String};

pub trait EmployeeManagement {
    fn initialize(env: Env, admin: Address, token_contract: Address);
    
    fn add_employee(
        env: Env, 
        employee: Address, 
        rank: String, 
        salary: i128
    );
    
    fn remove_employee(env: Env, employee: Address);
    
    fn update_salary(env: Env, employee: Address, new_salary: i128);
    
    fn promote_employee(env: Env, employee: Address, new_rank: String, new_salary: i128);
    
    fn suspend_employee(env: Env, employee: Address);
    
    fn reactivate_employee(env: Env, employee: Address);
    
    fn pay_salary(env: Env, employee: Address);
    
    fn get_employee(env: Env, employee: Address) -> Option<EmployeeInfo>;
    
    fn is_active(env: Env, employee: Address) -> bool;
}

pub use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeInfo {
    pub rank: String,
    pub salary: i128,
    pub is_active: bool,
    pub last_paid: u64,
}
