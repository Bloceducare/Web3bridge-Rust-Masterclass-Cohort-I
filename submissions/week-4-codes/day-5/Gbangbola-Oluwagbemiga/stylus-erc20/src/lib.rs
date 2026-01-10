#![no_main]
#![no_std]

// Re-export the main contract
pub use erc20::ERC20Token;

// Library no_std setup
extern crate alloc;
use alloc::vec::Vec;
use stylus_sdk::prelude::*;

// Import modules
mod erc20;
mod storage;

// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
