use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EmployeeError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    EmployeeNotFound = 4,
    InstitutionNotFound = 5,
    EmployeeAlreadyExists = 6,
    InstitutionNotActive = 7,
    InvalidSalary = 8,
    InvalidRank = 9,
    EmployeeAlreadySuspended = 10,
    EmployeeNotActive = 11,
    InsufficientFunds = 12,
    TokenError = 13,
}



