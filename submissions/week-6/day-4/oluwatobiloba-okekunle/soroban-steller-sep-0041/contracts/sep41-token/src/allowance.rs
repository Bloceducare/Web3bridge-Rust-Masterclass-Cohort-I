use soroban_sdk::{Address, Env};

use crate::{error::TokenError, storage};

pub fn read_allowance(e: &Env, from: &Address, spender: &Address) -> i128 {
    storage::get_allowance(e, from, spender)
}

pub fn write_allowance(
    e: &Env,
    from: &Address,
    spender: &Address,
    amount: i128,
    expiration_ledger: u32,
) -> Result<(), TokenError> {
    if amount < 0 {
        return Err(TokenError::InvalidAmount);
    }

    if amount > 0 && expiration_ledger < e.ledger().sequence() {
        return Err(TokenError::InvalidExpiration);
    }

    storage::set_allowance(e, from, spender, amount, expiration_ledger);
    Ok(())
}

pub fn spend_allowance(
    e: &Env,
    from: &Address,
    spender: &Address,
    amount: i128,
) -> Result<(), TokenError> {
    let allowance = read_allowance(e, from, spender);
    if allowance < amount {
        return Err(TokenError::InsufficientAllowance);
    }

    // Find the expiration ledger for updating allowance
    let current_expiration = storage::get_allowance(e, from, spender);
    if current_expiration > 0 {
        // Get the current allowance data to preserve expiration
        let key = storage::DataKey::Allowance(storage::AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        if let Some(allowance_data) = e
            .storage()
            .temporary()
            .get::<storage::DataKey, storage::AllowanceValue>(&key)
        {
            write_allowance(
                e,
                from,
                spender,
                allowance - amount,
                allowance_data.expiration_ledger,
            )?;
        }
    }

    Ok(())
}
