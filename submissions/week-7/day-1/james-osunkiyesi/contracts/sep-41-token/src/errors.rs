use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum TokenError {
    AlreadyInitialized = 1,
    NegativeAmount = 2,
    InsufficientBalance = 3,
    InsufficientAllowance = 4,
}