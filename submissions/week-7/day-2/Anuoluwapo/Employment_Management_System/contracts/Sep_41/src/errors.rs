use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    AlreadyInitialized = 1,
    NameTooLong = 2,
    SymbolTooLong = 3,
    DecimalsTooHigh = 4,
    InvalidTotalSupply = 5,
    InvalidAmount = 6,
    InsufficientBalance = 7,
    InsufficientAllowance = 8,
    AccountFrozen = 9,
    InvalidExpiration = 10,
    BalanceOverflow = 11,
    SupplyOverflow = 12,
    NotInitialized = 13,
    Unauthorized = 14,
    BalanceUnderflow = 15,
    DecimalTooLong = 16,
}