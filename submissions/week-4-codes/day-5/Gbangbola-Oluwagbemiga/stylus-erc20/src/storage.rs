use stylus_sdk::{
    alloy_primitives::Address,
    prelude::*,
    storage::{StorageMap, StorageString, StorageU256, StorageU8},
};

#[storage]
#[entrypoint]
pub struct ERC20Token {
    pub name: StorageString,
    pub symbol: StorageString,
    pub decimals: StorageU8,
    pub total_supply: StorageU256,
    pub balances: StorageMap<Address, StorageU256>,
    pub allowances: StorageMap<Address, StorageMap<Address, StorageU256>>,
}
