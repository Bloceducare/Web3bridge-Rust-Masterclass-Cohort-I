use soroban_sdk::{ contracttype, Address };

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    Allowance(AllowanceKey),
    Balance(Address),
    Name,
    Symbol,
    Decimals,
    TotalSupply,
    Admin,
}
