use crate::error::ContractError;

pub fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::NegativeAmountError)
    } else {
        Ok(())
    }
}
