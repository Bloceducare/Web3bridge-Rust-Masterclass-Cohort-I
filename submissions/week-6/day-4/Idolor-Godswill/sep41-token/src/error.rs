use soroban_sdk::contracterror;

/// Custom error types for the SEP-41 token contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    /// Insufficient balance for the requested operation
    InsufficientBalance = 1,
    
    /// Insufficient allowance for the requested operation
    InsufficientAllowance = 2,
    
    /// The caller is not authorized to perform this operation
    Unauthorized = 3,
    
    /// Invalid amount (e.g., negative amount where positive expected)
    InvalidAmount = 4,
    
    /// The contract has not been initialized
    NotInitialized = 5,
    
    /// The contract has already been initialized
    AlreadyInitialized = 6,
    
    /// Invalid address provided
    InvalidAddress = 7,
    
    /// Allowance has expired
    AllowanceExpired = 8,
    
    /// Invalid expiration ledger
    InvalidExpiration = 9,
    
    /// Arithmetic overflow occurred
    Overflow = 10,
    
    /// Arithmetic underflow occurred
    Underflow = 11,
}
