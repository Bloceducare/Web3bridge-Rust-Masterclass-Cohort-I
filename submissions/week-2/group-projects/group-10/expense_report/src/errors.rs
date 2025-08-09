#[derive(Debug)]
pub enum RemoveError {
    NotFound,
    CannotRemovePending,
}

#[derive(Debug)]
pub enum EditError {
    NotFound,
    InvalidAmount,
    InvalidStatus,
    Cancelled,
}
