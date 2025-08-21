use soroban_sdk::contracterror;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[contracterror]
#[repr(u32)]
pub enum ContractError {
    InternalError = 1,
    AlreadyInitializedError = 3,
    UnauthorizedError = 4,
    NegativeAmountError = 8,
    BalanceError = 10,
    OverflowError = 12,
    InsufficientAllowanceError = 13,
    InsufficientBalanceError = 14,
    NotInitializedError = 15,
}
