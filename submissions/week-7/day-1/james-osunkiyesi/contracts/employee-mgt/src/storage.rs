use soroban_sdk::{contracttype, Address, String, Timepoint};

#[contracttype]
#[derive(PartialEq, Clone)]
pub enum EmployeeRank {
    INTERN = 1,
    JUNIOR = 2,
    INTERMEDIATE = 3,
    SENIOR = 4,
    MANAGER = 5
}

impl EmployeeRank {
    pub fn match_rank(r: u32) -> Option<Self> {
        match r {
            1 => Some(EmployeeRank::INTERN),
            2 => Some(EmployeeRank::JUNIOR),
            3 => Some(EmployeeRank::INTERMEDIATE),
            4 => Some(EmployeeRank::SENIOR),
            5 => Some(EmployeeRank::MANAGER),
            _ => None,
        }
    }

    pub fn get_pay(r: Self) -> i128 {
        match r {
            EmployeeRank::INTERN => 1_500,
            EmployeeRank::JUNIOR => 2_500,
            EmployeeRank::INTERMEDIATE => 4_500,
            EmployeeRank::SENIOR => 7_000,
            EmployeeRank::MANAGER => 12_000,
        }
    }
}

#[contracttype]
pub enum EmployeeDept {
    DESIGN,
    DEVELOPMENT,
    MARKETING,
    CONTENT,
    ADMINISTRATIVE
}

#[contracttype]
pub enum EmployeeStatus {
    ACTIVE,
    SUSPENDED(u64),
    OnLeave
}

impl EmployeeStatus {
    pub fn check_is_active(self: Self, current_time: u64) -> bool {
        match self {
            EmployeeStatus::ACTIVE => true,
            EmployeeStatus::SUSPENDED(x) => {
                current_time >= x
            },
            EmployeeStatus::OnLeave => false,
        }
    }
}



#[contracttype]
pub struct Employee {
    pub name: String,
    pub rank: EmployeeRank,
    pub dept: EmployeeDept,
    pub time_employed: Timepoint,
    pub time_since_last_pay: Timepoint,
    pub status: EmployeeStatus
}

#[contracttype]
pub enum DataKey {
    Admin,
    PaymentToken,
    Employee(Address),
}