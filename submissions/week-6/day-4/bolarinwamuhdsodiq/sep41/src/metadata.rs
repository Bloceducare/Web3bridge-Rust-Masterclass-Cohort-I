use soroban_sdk::{Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

pub fn read_decimal(e: &Env) -> u32 {
    let utils = TokenUtils::new(e);
    utils.metadata().get_metadata().decimal
}

pub fn read_name(e: &Env) -> String {
    let utils = TokenUtils::new(e);
    utils.metadata().get_metadata().name
}

pub fn read_symbol(e: &Env) -> String {
    let utils = TokenUtils::new(e);
    utils.metadata().get_metadata().symbol
}

pub fn write_metadata(e: &Env, metadata: TokenMetadata) {
    let utils = TokenUtils::new(e);
    utils.metadata().set_metadata(&metadata);
}
