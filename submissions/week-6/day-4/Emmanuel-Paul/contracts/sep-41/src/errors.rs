use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenError {
    AlreadyInitialized = 1,
    NegativeAmount = 2,
    InsufficientBalance = 3,
    InsufficientAllowance = 4,
    Unauthorized = 5,
    ExpiredAllowance = 6,
}
