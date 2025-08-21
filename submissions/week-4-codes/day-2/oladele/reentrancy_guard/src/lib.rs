#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::vec::Vec;

use alloy_sol_types::{sol, SolError};
/// Import items from the SDK. The prelude contains common traits and macros.
use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageBool};


sol!{
    error ReentrancyGuardReentrantCall();
}

#[storage]
#[entrypoint]
pub struct ReentrancyGuard{
    notEntered: StorageBool,
    entered:StorageBool,
    status:StorageBool
}
/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl ReentrancyGuard{
    #[constructor]
    pub fn constructor(&mut self){
        let notEntered = self.notEntered.get();
        let _ = self.status.set(notEntered);
    }

    pub fn _nonReentrantBefore(&mut self)->Result<(),Vec<u8>>{
        if self.status.get() == self.entered.get(){
            return Err(ReentrancyGuardReentrantCall{}.abi_encode());
        }
        let status = self.status.get();
        self.entered.set(status);
        Ok(())
    }

    pub fn _nonReentrantAfter(&mut self){
        self.entered.set(self.notEntered.get());
    }

    pub fn _reentrancyGuardEntered(&mut self){
        self.status.set(self.entered.get());
    }

   
}
