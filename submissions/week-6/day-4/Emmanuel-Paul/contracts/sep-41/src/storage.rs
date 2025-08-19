use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug)]
pub struct AllowanceData {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
pub enum StorageKey {
    Balance(Address),
    Allowance(Address, Address),
    Metadata,
    Admin,
}

pub struct Storage;

impl Storage {
    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&StorageKey::Admin, admin);
    }

    pub fn get_admin(env: &Env) -> Option<Address> {
        env.storage().instance().get(&StorageKey::Admin)
    }

    pub fn set_metadata(env: &Env, decimals: u32, name: String, symbol: String) {
        let metadata = (decimals, name, symbol);
        env.storage()
            .instance()
            .set(&StorageKey::Metadata, &metadata);
    }

    pub fn get_metadata(env: &Env) -> Option<(u32, String, String)> {
        env.storage().instance().get(&StorageKey::Metadata)
    }

    pub fn get_balance(env: &Env, address: &Address) -> i128 {
        env.storage()
            .persistent()
            .get(&StorageKey::Balance(address.clone()))
            .unwrap_or(0)
    }

    pub fn set_balance(env: &Env, address: &Address, amount: i128) {
        env.storage()
            .persistent()
            .set(&StorageKey::Balance(address.clone()), &amount);
    }

    pub fn get_allowance(env: &Env, from: &Address, spender: &Address) -> Option<AllowanceData> {
        env.storage()
            .persistent()
            .get(&StorageKey::Allowance(from.clone(), spender.clone()))
    }

    pub fn set_allowance(env: &Env, from: &Address, spender: &Address, allowance: AllowanceData) {
        env.storage().persistent().set(
            &StorageKey::Allowance(from.clone(), spender.clone()),
            &allowance,
        );
    }

    pub fn remove_allowance(env: &Env, from: &Address, spender: &Address) {
        env.storage()
            .persistent()
            .remove(&StorageKey::Allowance(from.clone(), spender.clone()));
    }
}
