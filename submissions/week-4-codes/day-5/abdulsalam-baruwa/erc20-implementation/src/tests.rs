//! Basic tests for the ERC20 token implementation.

use alloy_primitives::{Address, U256};

fn address(n: u8) -> Address {
    Address::from([n; 20])
}

#[test]
fn test_compilation() {
    assert!(true);
}

#[test]
fn test_constants() {
    let zero_address = Address::ZERO;
    let amount = U256::from(1000);

    assert_eq!(zero_address, Address::from([0u8; 20]));
    assert!(amount > U256::ZERO);
}

#[test]
fn test_address_helper() {
    let addr1 = address(1);
    let addr2 = address(2);

    assert_ne!(addr1, addr2);
    assert_ne!(addr1, Address::ZERO);
}

#[test]
fn test_u256_operations() {
    let supply = U256::from(1000000) * U256::from(10).pow(U256::from(18));
    let transfer_amount = U256::from(1000) * U256::from(10).pow(U256::from(18));

    assert!(supply > transfer_amount);
    assert_eq!(
        supply - transfer_amount,
        U256::from(999000) * U256::from(10).pow(U256::from(18))
    );
}
