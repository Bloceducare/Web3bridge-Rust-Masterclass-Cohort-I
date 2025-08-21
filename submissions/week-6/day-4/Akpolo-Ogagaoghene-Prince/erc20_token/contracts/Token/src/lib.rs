#![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod event;
mod metadata;
mod storage_types;

#[cfg(test)]
mod test;

pub use crate::contract::TokenClient;