use soroban_sdk::{contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    TotalSupply,
    Balance(Address),
    Allowance(AllowanceDataKey),
    Metadata(MetadataKey),
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum MetadataKey {
    Name,
    Symbol,
    Decimals,
}

const DAY_IN_LEDGERS: u32 = 17280; // Assuming 5-second ledger times
const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

pub fn set_total_supply(e: &Env, supply: i128) {
    e.storage().instance().set(&DataKey::TotalSupply, &supply);
}

pub fn get_total_supply(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&DataKey::TotalSupply)
        .unwrap_or(0)
}

pub fn set_balance(e: &Env, addr: &Address, balance: i128) {
    let key = DataKey::Balance(addr.clone());
    if balance > 0 {
        e.storage().persistent().set(&key, &balance);
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    } else {
        e.storage().persistent().remove(&key);
    }
}

pub fn get_balance(e: &Env, addr: &Address) -> i128 {
    let key = DataKey::Balance(addr.clone());
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

pub fn set_allowance(
    e: &Env,
    from: &Address,
    spender: &Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });

    if amount > 0 && expiration_ledger >= e.ledger().sequence() {
        let allowance = AllowanceValue {
            amount,
            expiration_ledger,
        };
        e.storage().temporary().set(&key, &allowance);

        if expiration_ledger < e.ledger().sequence() + BALANCE_LIFETIME_THRESHOLD {
            e.storage().temporary().extend_ttl(
                &key,
                expiration_ledger - e.ledger().sequence(),
                expiration_ledger - e.ledger().sequence(),
            );
        } else {
            e.storage().temporary().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
    } else {
        e.storage().temporary().remove(&key);
    }
}

pub fn get_allowance(e: &Env, from: &Address, spender: &Address) -> i128 {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });

    if let Some(allowance) = e.storage().temporary().get::<DataKey, AllowanceValue>(&key) {
        if allowance.expiration_ledger < e.ledger().sequence() {
            e.storage().temporary().remove(&key);
            0
        } else {
            allowance.amount
        }
    } else {
        0
    }
}

pub fn set_metadata(e: &Env, key: MetadataKey, value: String) {
    e.storage().instance().set(&DataKey::Metadata(key), &value);
}

pub fn get_metadata(e: &Env, key: MetadataKey) -> String {
    e.storage().instance().get(&DataKey::Metadata(key)).unwrap()
}

pub fn set_decimals(e: &Env, decimals: u32) {
    e.storage()
        .instance()
        .set(&DataKey::Metadata(MetadataKey::Decimals), &decimals);
}

pub fn get_decimals(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&DataKey::Metadata(MetadataKey::Decimals))
        .unwrap()
}
