use soroban_sdk::{contracttype, Address, String};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    Admin,
    Institution,
    TokenContract,
    Employee(Address),
    EmployeeList,
    Initialized,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum EmployeeRank {
    Intern = 1,
    Junior = 2,
    Mid = 3,
    Senior = 4,
    Lead = 5,
    Manager = 6,
    Director = 7,
}

impl EmployeeRank {
    pub fn to_u32(&self) -> u32 {
        match self {
            EmployeeRank::Intern => 1,
            EmployeeRank::Junior => 2,
            EmployeeRank::Mid => 3,
            EmployeeRank::Senior => 4,
            EmployeeRank::Lead => 5,
            EmployeeRank::Manager => 6,
            EmployeeRank::Director => 7,
        }
    }
    pub fn from_u32(value: u32) -> Option<Self> {
        match value {
            1 => Some(EmployeeRank::Intern),
            2 => Some(EmployeeRank::Junior),
            3 => Some(EmployeeRank::Mid),
            4 => Some(EmployeeRank::Senior),
            5 => Some(EmployeeRank::Lead),
            6 => Some(EmployeeRank::Manager),
            7 => Some(EmployeeRank::Director),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum EmployeeStatus {
    Active,
    Suspended,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Employee {
    pub wallet_address: Address,
    pub name: String,
    pub salary: i128,
    pub rank: EmployeeRank,
    pub status: EmployeeStatus,
    pub last_salary_payment: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct InstitutionInfo {
    pub admin: Address,
    pub name: String,
    pub total_employees: u32,
    pub token_contract: Address,
}
