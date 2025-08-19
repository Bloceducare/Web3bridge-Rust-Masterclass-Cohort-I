use soroban_sdk::{contracttype, Address, String};

/// Employee rank hierarchy
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum EmployeeRank {
    Intern,
    Junior,
    Mid,
    Senior,
    Lead,
    Manager,
    Director,
    VP,
    CEO,
}

impl EmployeeRank {
    /// Get the base salary multiplier for each rank
    pub fn salary_multiplier(&self) -> u32 {
        match self {
            EmployeeRank::Intern => 1,
            EmployeeRank::Junior => 2,
            EmployeeRank::Mid => 3,
            EmployeeRank::Senior => 5,
            EmployeeRank::Lead => 7,
            EmployeeRank::Manager => 10,
            EmployeeRank::Director => 15,
            EmployeeRank::VP => 20,
            EmployeeRank::CEO => 30,
        }
    }

    /// Get the next rank for promotion
    pub fn next_rank(&self) -> Option<EmployeeRank> {
        match self {
            EmployeeRank::Intern => Some(EmployeeRank::Junior),
            EmployeeRank::Junior => Some(EmployeeRank::Mid),
            EmployeeRank::Mid => Some(EmployeeRank::Senior),
            EmployeeRank::Senior => Some(EmployeeRank::Lead),
            EmployeeRank::Lead => Some(EmployeeRank::Manager),
            EmployeeRank::Manager => Some(EmployeeRank::Director),
            EmployeeRank::Director => Some(EmployeeRank::VP),
            EmployeeRank::VP => Some(EmployeeRank::CEO),
            EmployeeRank::CEO => None, // CEO is the highest rank
        }
    }
}

/// Employee status
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum EmployeeStatus {
    Active,
    Suspended,
    Terminated,
}

/// Employee information
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Employee {
    pub id: u64,
    pub address: Address,
    pub name: String,
    pub rank: EmployeeRank,
    pub status: EmployeeStatus,
    pub base_salary: i128,
    pub hire_date: u64,
    pub last_promotion: Option<u64>,
    pub department: String,
}

impl Employee {
    /// Calculate the current salary based on rank and base salary
    pub fn current_salary(&self) -> i128 {
        self.base_salary * (self.rank.salary_multiplier() as i128)
    }

    /// Check if employee is eligible for promotion (active and not recently promoted)
    pub fn is_promotion_eligible(&self, current_time: u64, min_promotion_interval: u64) -> bool {
        if self.status != EmployeeStatus::Active {
            return false;
        }

        if self.rank == EmployeeRank::CEO {
            return false; // CEO is the highest rank
        }

        match self.last_promotion {
            Some(last_promo) => current_time >= last_promo + min_promotion_interval,
            None => current_time >= self.hire_date + min_promotion_interval,
        }
    }
}

/// Institution information
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Institution {
    pub name: String,
    pub admin: Address,
    pub token_contract: Address,
    pub base_salary_amount: i128,
    pub min_promotion_interval: u64, // in ledger sequences
}

/// Events emitted by the contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeAddedEvent {
    pub employee_id: u64,
    pub employee_address: Address,
    pub rank: EmployeeRank,
    pub salary: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeePromotedEvent {
    pub employee_id: u64,
    pub old_rank: EmployeeRank,
    pub new_rank: EmployeeRank,
    pub new_salary: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmployeeStatusChangedEvent {
    pub employee_id: u64,
    pub old_status: EmployeeStatus,
    pub new_status: EmployeeStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SalaryPaidEvent {
    pub employee_id: u64,
    pub amount: i128,
    pub period: u64,
}
