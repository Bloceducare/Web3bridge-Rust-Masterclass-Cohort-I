use soroban_sdk::{Env, String};
use soroban_token_sdk::metadata::TokenMetadata;

use crate::storage::{self, MetadataKey};

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    storage::set_metadata(e, MetadataKey::Name, metadata.name);
    storage::set_metadata(e, MetadataKey::Symbol, metadata.symbol);
    storage::set_decimals(e, metadata.decimal);
}

pub fn read_decimal(e: &Env) -> u32 {
    storage::get_decimals(e)
}

pub fn read_name(e: &Env) -> String {
    storage::get_metadata(e, MetadataKey::Name)
}

pub fn read_symbol(e: &Env) -> String {
    storage::get_metadata(e, MetadataKey::Symbol)
}
