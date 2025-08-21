// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc20;

use alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;
use crate::erc20::Erc20Params;

/// Immutable definitions
struct StylusTokenParams;
impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STK";
    const DECIMALS: u8 = 18;
}

// Define the entrypoint as a Solidity storage object. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
sol_storage! {
    #[entrypoint]
    struct StylusToken {
        // Use the ERC20Token struct from the erc20 module
        #[borrow]
        erc20::ERC20Token erc20;
    }
}

#[public]
impl StylusToken {
    /// Returns the name of the token
    pub fn name(&self) -> Result<String, Vec<u8>> {
        self.erc20.name()
    }

    /// Returns the symbol of the token
    pub fn symbol(&self) -> Result<String, Vec<u8>> {
        self.erc20.symbol()
    }

    /// Returns the decimals of the token
    pub fn decimals(&self) -> Result<u8, Vec<u8>> {
        self.erc20.decimals()
    }

    /// Returns the total supply of tokens
    pub fn total_supply(&self) -> Result<U256, Vec<u8>> {
        self.erc20.total_supply()
    }

    /// Returns the balance of the given address
    pub fn balance_of(&self, owner: Address) -> Result<U256, Vec<u8>> {
        self.erc20.balance_of(owner)
    }

    /// Transfers tokens to another address
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Vec<u8>> {
        self.erc20.transfer(to, value)
    }

    /// Approves another address to spend tokens
    pub fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Vec<u8>> {
        self.erc20.approve(spender, value)
    }

    /// Transfers tokens from one address to another using allowance
    pub fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<bool, Vec<u8>> {
        self.erc20.transfer_from(from, to, value)
    }

    /// Returns the allowance of the given address
    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        self.erc20.allowance(owner, spender)
    }

    /// Mints tokens to the caller's address
    pub fn mint(&mut self, value: U256) -> Result<(), Vec<u8>> {
        self.erc20.mint(msg::sender(), value)
    }

    /// Burns tokens from the caller's address
    pub fn burn(&mut self, value: U256) -> Result<(), Vec<u8>> {
        self.erc20.burn(msg::sender(), value)
    }

    /// Returns the owner of the contract   
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        self.erc20.owner()
    }
}