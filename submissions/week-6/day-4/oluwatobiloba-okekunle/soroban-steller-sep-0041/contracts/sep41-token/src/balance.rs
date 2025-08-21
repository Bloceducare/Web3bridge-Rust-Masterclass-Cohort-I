use soroban_sdk::{Address, Env};

use crate::{error::TokenError, storage};

pub fn read_balance(e: &Env, addr: &Address) -> i128 {
    storage::get_balance(e, addr)
}

pub fn receive_balance(e: &Env, addr: &Address, amount: i128) -> Result<(), TokenError> {
    if amount < 0 {
        return Err(TokenError::InvalidAmount);
    }
    if amount == 0 {
        return Ok(());
    }

    let balance = read_balance(e, addr);
    let new_balance = balance
        .checked_add(amount)
        .ok_or(TokenError::BalanceOverflow)?;

    storage::set_balance(e, addr, new_balance);
    Ok(())
}

pub fn spend_balance(e: &Env, addr: &Address, amount: i128) -> Result<(), TokenError> {
    if amount < 0 {
        return Err(TokenError::InvalidAmount);
    }
    if amount == 0 {
        return Ok(());
    }

    let balance = read_balance(e, addr);
    if balance < amount {
        return Err(TokenError::InsufficientBalance);
    }

    storage::set_balance(e, addr, balance - amount);
    Ok(())
}

pub fn transfer_balance(
    e: &Env,
    from_addr: &Address,
    to_addr: &Address,
    amount: i128,
) -> Result<(), TokenError> {
    spend_balance(e, from_addr, amount)?;
    receive_balance(e, to_addr, amount)?;
    Ok(())
}
