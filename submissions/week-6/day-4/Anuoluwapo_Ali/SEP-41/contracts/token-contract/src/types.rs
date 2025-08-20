use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    Balance(Address),
    Allowance(Address, Address),
    AllowanceExpiration(Address, Address),
    Frozen(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceData {
    pub amount: i128,
    pub expiration_ledger: u32,
}
