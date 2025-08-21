use alloc::string::String;
use alloc::vec::Vec;
use alloy_primitives::Address;
use stylus_sdk::ArbResult;

pub trait IERC20 {
    fn init(&mut self, name: String, symbol: String, initial_supply: u128)
    -> Result<bool, Vec<u8>>;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn decimals(&self) -> u8;
    fn total_supply(&self) -> u128;
    fn balance_of(&self, owner: Address) -> u128;
    fn allowance(&self, owner: Address, spender: Address) -> u128;
    fn transfer(&mut self, to: Address, value: u128) -> ArbResult;
    fn transfer_from(&mut self, from: Address, to: Address, value: u128) -> ArbResult;
    fn approve(&mut self, spender: Address, value: u128) -> ArbResult;
}
