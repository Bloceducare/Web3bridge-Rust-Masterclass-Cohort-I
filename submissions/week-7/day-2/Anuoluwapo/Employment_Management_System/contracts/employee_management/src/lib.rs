#![no_std]

mod employee_contract;
pub mod errors;

mod imports;

#[cfg(test)]
mod test;

pub use employee_contract::*;
pub use errors::*;
