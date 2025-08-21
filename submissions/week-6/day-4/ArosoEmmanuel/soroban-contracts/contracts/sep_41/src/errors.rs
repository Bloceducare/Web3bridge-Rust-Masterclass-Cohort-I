use soroban_sdk::contracterror;
//-----------------------------------------------------------------------------
// Errors
//-----------------------------------------------------------------------------

/// Possible errors for the payroll contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TokenError {
    /// Returned if there are insufficient funds to complete a transfer.
    InsufficientBalance = 1,
    /// Returned if the allowance is insufficient for a transfer.
    InsufficientAllowance = 2,
    /// Returned if the amount is invalid (e.g., zero or negative).
    InvalidAmount = 3,
    /// Returned if the operation would cause an overflow.
    Overflow = 4,
    /// Returned if the caller is not authorized to perform the action.
    NotAuthorized = 5,
    /// Returned if the contract has already been initialized.
    AlreadyInitialized = 6,
    /// Returned if the expiration ledger is invalid.
    InvalidExpirationLedger = 7,
    /// Returned if the admin is not set.
    AdminNotSet = 8,
}
