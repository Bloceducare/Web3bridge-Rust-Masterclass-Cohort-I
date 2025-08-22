use soroban_sdk::{contracttype, Address};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum EmployeeRank {
    Junior = 1,
    Mid = 2,
    Senior = 3,
    Lead = 4,
    Manager = 5,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Employee {
    pub address: Address,
    pub employee_id: u32,
    pub rank: EmployeeRank,
    pub weekly_salary: u64,
    pub is_suspended: bool,
    pub is_active: bool
}