// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

mod erc20; // make erc20 visible outside the crate

pub use crate::erc20::{Erc20, Erc20Error, Erc20Params, InsufficientAllowance};
use alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*;

/// Immutable token metadata
struct TokenParams;
impl Erc20Params for TokenParams {
    const NAME: &'static str = "Stylus Token";
    const SYMBOL: &'static str = "STYL";
    const DECIMALS: u8 = 18;
}

// Contract entrypoint with owner + ERC-20 borrowed storage
sol_storage! {
    #[entrypoint]
    pub struct StylusToken {
        /// owner for minting
        address owner;
        /// ERC-20 implementation borrows our storage
        #[borrow]
        Erc20<TokenParams> erc20;
    }
}

#[public]
#[inherit(Erc20<TokenParams>)]
impl StylusToken {
    pub fn init(&mut self) {
        if self.owner.get() == Address::ZERO {
            self.owner.set(self.vm().msg_sender());
        }
    }
    pub fn approve(&mut self, spender: Address, amount: U256) -> bool {
        self.erc20.approve(spender, amount)
    }

    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.erc20.allowance(owner, spender)
    }

    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<bool, Erc20Error> {
        self.erc20.transfer_from(from, to, amount)
    }
    pub fn name(&self) -> String {
        self.erc20.name()
    }

    pub fn symbol(&self) -> String {
        self.erc20.symbol()
    }

    pub fn decimals(&self) -> u8 {
        self.erc20.decimals()
    }

    pub fn total_supply(&self) -> U256 {
        self.erc20.total_supply()
    }

    pub fn balance_of(&self, account: Address) -> U256 {
        self.erc20.balance_of(account)
    }

    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Erc20Error> {
        self.erc20.transfer(to, amount)
    }

    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self._only_owner()?;
        self.erc20.mint(self.owner.get(), value)
    }

    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self._only_owner()?;
        self.erc20.mint(to, value)
    }

    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(self.vm().msg_sender(), value)
    }

    pub fn owner(&self) -> Address {
        self.owner.get()
    }

    fn _only_owner(&self) -> Result<(), Erc20Error> {
        if self.vm().msg_sender() != self.owner.get() {
            return Err(Erc20Error::InsufficientAllowance(InsufficientAllowance {
                owner: self.owner.get(),
                spender: self.vm().msg_sender(),
                have: U256::ZERO,
                want: U256::ZERO,
            }));
        }
        Ok(())
    }
}
