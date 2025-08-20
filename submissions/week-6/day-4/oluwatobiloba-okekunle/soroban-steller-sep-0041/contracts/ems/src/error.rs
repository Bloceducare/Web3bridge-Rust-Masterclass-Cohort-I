use soroban_sdk::contracterror;


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EmsError {
    /// The administrator is not authorized for this operation
    Unauthorized = 1,
    /// The user is not authorized for this operation
    UnauthorizedUser = 2,
    /// The user is not registered
    NotRegistered = 3,
    /// The user is already registered
    AlreadyRegistered = 4,
    /// The user is suspended
    Suspended = 5,
    /// The user is already suspended
    AlreadySuspended = 6,
    /// The user is already promoted
    AlreadyPromoted = 7,
    /// The user is already demoted
    AlreadyDemoted = 8,
    /// The user is already deleted
    AlreadyDeleted = 9,

    /// The user is not active
    NotActive = 10,
    /// The user is not due for payment
    NotDueForPayment = 11,
    /// The user is not suspended
    NotSuspended = 12,
    /// The user is not registered
    NotInitialized = 13,
}