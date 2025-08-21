#![no_std]

mod admin;
mod allowance;
mod balance;
mod contract;
mod error;
mod events;
mod metadata;
mod storage;
mod test;

pub use crate::contract::{SepToken, SepTokenClient};
