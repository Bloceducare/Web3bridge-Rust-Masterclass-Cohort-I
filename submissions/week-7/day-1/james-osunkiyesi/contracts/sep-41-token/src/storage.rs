use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceData {
    pub amount: i128,
    pub live_until_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    Allowance(AllowanceKey),
    Balance(Address),
    Admin(Address),
    IsInitialized
}
