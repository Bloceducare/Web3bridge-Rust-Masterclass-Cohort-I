#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

pub mod erc20;
mod errors;
mod ierc20;
mod test;

// use alloc::vec::Vec;
//
// use stylus_sdk::{alloy_primitives::U256, prelude::*};
