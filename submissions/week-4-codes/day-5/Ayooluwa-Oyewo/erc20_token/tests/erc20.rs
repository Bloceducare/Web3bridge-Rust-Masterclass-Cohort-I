/* ===========================
            tests
=========================== */
use alloy_primitives::{address, Address, U256};
use erc20_token::{Erc20Error, StylusToken};
use stylus_sdk::testing::*;

fn setup() -> (TestVM, StylusToken, Address, Address, Address) {
    let vm = TestVM::default();
    let mut token = StylusToken::from(&vm);
    // initialize owner
    vm.set_sender(address!("0x1000000000000000000000000000000000000000"));
    token.init();
    let owner = token.owner();
    // two users
    let alice = address!("0xA11CE00000000000000000000000000000000000");
    let bob = address!("0xB0B0000000000000000000000000000000000000");
    (vm, token, alice, bob, owner)
}

#[test]
fn metadata_and_initial_state() {
    let (vm, token, _alice, _bob, _owner) = setup();
    assert_eq!(token.name(), "Stylus Token");
    assert_eq!(token.symbol(), "STYL");
    assert_eq!(token.decimals(), 18);
    assert_eq!(token.total_supply(), U256::ZERO);
    assert_ne!(token.owner(), Address::ZERO);
    drop(vm);
}

#[test]
fn owner_mints_and_users_transfer() {
    let (vm, mut token, alice, bob, owner) = setup();
    // owner mints to Alice
    vm.set_sender(owner);
    token
        .mint_to(alice, U256::from(1_000_000_000_000_000_000u128))
        .unwrap();
    assert_eq!(
        token.total_supply(),
        U256::from(1_000_000_000_000_000_000u128)
    );
    assert_eq!(
        token.balance_of(alice),
        U256::from(1_000_000_000_000_000_000u128)
    );

    // alice -> bob transfer 0.4 ether units
    vm.set_sender(alice);
    assert!(token
        .transfer(bob, U256::from(400_000_000_000_000_000u128))
        .unwrap());
    assert_eq!(
        token.balance_of(alice),
        U256::from(600_000_000_000_000_000u128)
    );
    assert_eq!(
        token.balance_of(bob),
        U256::from(400_000_000_000_000_000u128)
    );
    drop(vm);
}

#[test]
fn allowances_and_transfer_from() {
    let (vm, mut token, alice, bob, owner) = setup();
    vm.set_sender(owner);
    token.mint_to(alice, U256::from(1_000)).unwrap();

    // Alice approves Bob for 250
    vm.set_sender(alice);
    assert!(token.approve(bob, U256::from(250)));
    assert_eq!(token.allowance(alice, bob), U256::from(250));

    // Bob spends 200 from Alice to Bob
    vm.set_sender(bob);
    assert!(token.transfer_from(alice, bob, U256::from(200)).unwrap());
    assert_eq!(token.allowance(alice, bob), U256::from(50));
    assert_eq!(token.balance_of(alice), U256::from(800));
    assert_eq!(token.balance_of(bob), U256::from(200));
    drop(vm);
}

#[test]
fn reverts_on_insufficient_balance_and_allowance() {
    let (vm, mut token, alice, bob, owner) = setup();
    vm.set_sender(owner);
    token.mint_to(alice, U256::from(100)).unwrap();

    // insufficient balance on direct transfer
    vm.set_sender(bob);
    let err = token.transfer(alice, U256::from(1)).unwrap_err();
    match err {
        Erc20Error::InsufficientBalance(_) => {}
        _ => panic!("expected InsufficientBalance"),
    }

    // insufficient allowance on transfer_from
    vm.set_sender(bob);
    let err2 = token.transfer_from(alice, bob, U256::from(1)).unwrap_err();
    match err2 {
        Erc20Error::InsufficientAllowance(_) => {}
        _ => panic!("expected InsufficientAllowance"),
    }
    drop(vm);
}

#[test]
fn only_owner_can_mint() {
    let (vm, mut token, alice, _bob, owner) = setup();
    // Non-owner tries to mint
    vm.set_sender(alice);
    let err = token.mint(U256::from(10)).unwrap_err();
    match err {
        Erc20Error::InsufficientAllowance(_) => {}
        _ => panic!("expected owner check failure via error proxy"),
    }

    // Owner mints
    vm.set_sender(owner);
    token.mint(U256::from(10)).unwrap();
    assert_eq!(token.total_supply(), U256::from(10));
    assert_eq!(token.balance_of(owner), U256::from(10));
    drop(vm);
}
