use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenError {
    InternalError = 1,
    OperationNotSupported = 2,
    AlreadyInitializedError = 3,
    AlreadyApprovedError = 4,

    UnauthorizedError = 4,

    NegativeAmountError = 5, 
    AllowanceError = 9,
    BalanceError = 10,
    OverflowError = 11,

}