use soroban_sdk::{
    contracttype, Address};


// Token metadata
pub const DECIMAL: u32 = 7;
pub const NAME: &str = "EmarcToken";
pub const SYMBOL: &str = "EMARC";

// Data keys for storage
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Admin,
    TotalSupply,
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