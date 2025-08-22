#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;
mod erc20;

/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, msg, prelude::*};

use crate::erc20::{Erc20, Erc20Error, Erc20Params};

struct StylusTokenParams;

impl Erc20Params for StylusTokenParams {
    const NAME: &'static str = "StylusToken";
    const SYMBOL: &'static str = "STR";
    const DECIMALS: u8 = 18;
}

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    struct StylusToken {
        #[borrow]
        Erc20<StylusTokenParams> erc20
    }
}

#[public]
#[inherit(Erc20<StylusTokenParams>)]
impl StylusToken {
    //mint tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(msg::sender(), value)?;
        Ok(())
    }

    pub fn mint_to(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(msg::sender(), value);
        Ok(())
    }
}
