use soroban_sdk::contracterror;


//contract error types for the sep14 contract

#[contracterror]

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord)]
#[reprl(u32)]


pub enum TokenError {
    InsufficientBalance = 1,
    InsufficientAllowance = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    NotInitialized = 5,
    AlreadyInitialized = 6,
    InvalidAddress = 7,
    AllowanceExpired = 8,
    InvalidExpiration = 9,
    Overflow = 10,
    Underflow = 11,
}