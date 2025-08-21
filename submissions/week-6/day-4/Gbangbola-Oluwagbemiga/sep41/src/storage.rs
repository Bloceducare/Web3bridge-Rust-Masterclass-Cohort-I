use crate::error::ContractError;
use crate::types::{AllowanceValue, DataKey};
use soroban_sdk::{Address, Env};

pub fn write_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let key = DataKey::Allowance(from, spender);
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };
    e.storage().persistent().set(&key, &allowance);
}

pub fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(from, spender);
    if let Some(allowance) = e
        .storage()
        .persistent()
        .get::<DataKey, AllowanceValue>(&key)
    {
        if allowance.expiration_ledger < e.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

pub fn spend_allowance(
    e: &Env,
    from: Address,
    spender: Address,
    amount: i128,
) -> Result<(), ContractError> {
    let allowance = read_allowance(e, from.clone(), spender.clone());
    if allowance.amount < amount {
        return Err(ContractError::InsufficientAllowanceError);
    }
    write_allowance(
        e,
        from,
        spender,
        allowance.amount - amount,
        allowance.expiration_ledger,
    );
    Ok(())
}

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    e.storage().persistent().get(&key).unwrap_or(0)
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128) -> Result<(), ContractError> {
    let balance = read_balance(e, addr.clone());

    // Check for overflow
    if balance.checked_add(amount).is_none() {
        return Err(ContractError::OverflowError);
    }

    let key = DataKey::Balance(addr);
    e.storage().persistent().set(&key, &(balance + amount));
    Ok(())
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) -> Result<(), ContractError> {
    let balance = read_balance(e, addr.clone());
    if balance < amount {
        return Err(ContractError::InsufficientBalanceError);
    }
    let key = DataKey::Balance(addr);
    e.storage().persistent().set(&key, &(balance - amount));
    Ok(())
}
