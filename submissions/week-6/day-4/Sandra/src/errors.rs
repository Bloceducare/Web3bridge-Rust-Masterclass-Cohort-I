use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    NegativeAmount = 3,
    InsufficientBalance = 4,
    InsufficientAllowance = 5,
    AllowanceError = 6,
    Overflow = 7,
    Unauthorized = 8,
    InvalidInput = 9,
    InvalidAddress = 10,
}

impl TokenError {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn description(&self) -> &'static str {
        match self {
            TokenError::AlreadyInitialized => "Contract already initialized",
            TokenError::NotInitialized => "Contract not initialized",
            TokenError::NegativeAmount => "Amount cannot be negative",
            TokenError::InsufficientBalance => "Insufficient balance",
            TokenError::InsufficientAllowance => "Insufficient allowance",
            TokenError::AllowanceError => "Allowance error",
            TokenError::Overflow => "Arithmetic overflow",
            TokenError::Unauthorized => "Unauthorized operation",
            TokenError::InvalidInput => "Invalid input provided",
            TokenError::InvalidAddress => "Invalid address",
        }
    }
}