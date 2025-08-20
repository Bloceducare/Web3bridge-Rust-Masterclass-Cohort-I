use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn read_allowance(env: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = env
        .storage()
        .temporary()
        .get::<DataKey, AllowanceValue>(&key)
    {
        if allowance.expiration_ledger < env.ledger().sequence() {
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

pub fn write_allowance(
    env: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    env.storage()
        .temporary()
        .set(&key, &allowance);
    
    if amount > 0 && expiration_ledger < env.ledger().sequence() {
        panic!(
            "expiration_ledger: {} is less than ledger: {}",
            expiration_ledger,
            env.ledger().sequence()
        );
    }

    if amount > 0 {
        let live_for = expiration_ledger
            .checked_sub(env.ledger().sequence())
            .unwrap();

        env.storage().temporary().extend_ttl(
            &key,
            BALANCE_LIFETIME_THRESHOLD,
            BALANCE_BUMP_AMOUNT.min(live_for),
        );
    }
}

pub fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = read_allowance(env, from.clone(), spender.clone());
    if allowance.amount < amount {
        panic!(
            "insufficient allowance: {} < {}",
            allowance.amount, amount
        );
    }
    write_allowance(
        env,
        from,
        spender,
        allowance.amount - amount,
        allowance.expiration_ledger,
    );
}