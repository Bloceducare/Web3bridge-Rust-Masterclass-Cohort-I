use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {

    InsufficientBalance = 1,
   
    InsufficientAllowance = 2,
   
    Unauthorized = 3,
   
    InvalidAmount = 4,

    AlreadyInitialized = 5,

    NotInitialized = 6,

    InvalidExpiration = 7,

    BalanceOverflow = 8,

    SupplyOverflow = 9,
}
