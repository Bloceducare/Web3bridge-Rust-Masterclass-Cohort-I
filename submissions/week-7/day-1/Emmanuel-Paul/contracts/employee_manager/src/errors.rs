use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EmployeeError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    EmployeeNotFound = 3,
    EmployeeSuspended = 4,
    InvalidRank = 5,
    EmployeeAlreadySuspended = 6,
}