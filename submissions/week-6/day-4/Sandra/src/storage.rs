use soroban_sdk::{Address, Symbol, symbol_short};

use crate::AllowanceDataKey;

pub trait TokenStorage {
    fn get_balance_key(address: &Address) -> Symbol;
    fn get_allowance_key(key: &AllowanceDataKey) -> Symbol;
    fn get_admin_key() -> Symbol;
    fn get_metadata_key(field: &str) -> Symbol;
}

pub struct StorageKeys;

impl TokenStorage for StorageKeys {
    fn get_balance_key(_address: &Address) -> Symbol {
        symbol_short!("BALANCE")
    }

    fn get_allowance_key(_key: &AllowanceDataKey) -> Symbol {
        symbol_short!("ALLOW")
    }

    fn get_admin_key() -> Symbol {
        symbol_short!("ADMIN")
    }

    fn get_metadata_key(field: &str) -> Symbol {
        match field {
            "name" => symbol_short!("NAME"),
            "symbol" => symbol_short!("SYMBOL"), 
            "decimals" => symbol_short!("DECIMALS"),
            _ => panic!("Invalid metadata field"),
        }
    }
}

pub const BALANCE: fn(&Address) -> Address = |addr| addr.clone();
pub const ALLOWANCE: fn(&AllowanceDataKey) -> AllowanceDataKey = |key| key.clone();
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const TOTAL_SUPPLY: Symbol = symbol_short!("SUPPLY");
pub const TOKEN_NAME: Symbol = symbol_short!("NAME");
pub const TOKEN_SYMBOL: Symbol = symbol_short!("SYMBOL");
pub const TOKEN_DECIMALS: Symbol = symbol_short!("DECIMALS");