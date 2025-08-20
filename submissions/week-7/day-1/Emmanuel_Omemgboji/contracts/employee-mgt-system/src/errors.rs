use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EmployeeError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    EmployeeNotFound = 4,
    EmployeeAlreadyExists = 5,
    InvalidSalary = 6,
    InvalidRank = 7,
    EmployeeSuspended = 8,
    EmployeeAlreadyActive = 9,
    EmployeeAlreadySuspended = 10,
    SameRank = 11,
    TokenTransferFailed = 12,
    InsufficientFunds = 13,
    InvalidName = 14,
    TokenContractError = 15,
    StorageError = 16,
}
