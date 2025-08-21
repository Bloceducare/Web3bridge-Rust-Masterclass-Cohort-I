use soroban_sdk::{contracttype, Address, String};

pub const DECIMAL: u32 = 7;
pub const NAME: &str = "EmarcToken";
pub const SYMBOL: &str = "EMARC";
pub const ZERO_ADDRESS: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";

// Data keys for storage
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey), // AllowanceDataKey struct ->
    Balance(Address),            //
    Admin,
    TotalSupply,
    Owner,
    Metadata,
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

#[derive(Clone, Debug, PartialEq, Eq)]
#[contracttype]
pub struct Metadata {
    pub decimals: u32,
    pub name: String,
    pub symbol: String,
}
