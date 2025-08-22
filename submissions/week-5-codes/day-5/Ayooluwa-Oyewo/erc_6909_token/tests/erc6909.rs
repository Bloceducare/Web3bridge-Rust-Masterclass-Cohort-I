/* ===========================
            tests
=========================== */
use alloy_primitives::{address, Address, U256};
use erc6909_token::{Erc6909Error, MultiToken};
use stylus_sdk::testing::*;

fn setup() -> (TestVM, MultiToken, Address, Address, Address) {
    let vm = TestVM::default();
    let mut token = MultiToken::from(&vm);

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
fn initial_state() {
    let (vm, token, _alice, _bob, _owner) = setup();
    let id = U256::from(1);
    assert_eq!(token.total_supply(id), U256::ZERO);
    drop(vm);
}

#[test]
fn owner_mints_and_users_transfer_per_token() {
    let (vm, mut token, alice, bob, owner) = setup();
    let id = U256::from(1);

    vm.set_sender(owner);
    token.mint_to(alice, id, U256::from(1000)).unwrap();
    assert_eq!(token.total_supply(id), U256::from(1000));
    assert_eq!(token.balance_of(alice, id), U256::from(1000));

    vm.set_sender(alice);
    assert!(token.transfer_from(alice, bob, id, U256::from(400)).unwrap());
    assert_eq!(token.balance_of(alice, id), U256::from(600));
    assert_eq!(token.balance_of(bob, id), U256::from(400));
    drop(vm);
}

#[test]
fn allowances_and_transfer_from() {
    let (vm, mut token, alice, bob, owner) = setup();
    let id = U256::from(7);

    vm.set_sender(owner);
    token.mint_to(alice, id, U256::from(1000)).unwrap();

    vm.set_sender(alice);
    assert!(token.approve(bob, id, U256::from(250)));
    assert_eq!(token.allowance(alice, bob, id), U256::from(250));

    vm.set_sender(bob);
    assert!(token.transfer_from(alice, bob, id, U256::from(200)).unwrap());
    assert_eq!(token.allowance(alice, bob, id), U256::from(50));
    assert_eq!(token.balance_of(alice, id), U256::from(800));
    assert_eq!(token.balance_of(bob, id), U256::from(200));
    drop(vm);
}

#[test]
fn operator_approvals() {
    let (vm, mut token, alice, bob, owner) = setup();
    let id = U256::from(42);

    vm.set_sender(owner);
    token.mint_to(alice, id, U256::from(500)).unwrap();

    vm.set_sender(alice);
    assert!(token.set_operator(bob, true));
    assert!(token.operator_approval(alice, bob));

    vm.set_sender(bob);
    assert!(token.transfer_from(alice, bob, id, U256::from(500)).unwrap());
    assert_eq!(token.balance_of(alice, id), U256::ZERO);
    assert_eq!(token.balance_of(bob, id), U256::from(500));
    drop(vm);
}

#[test]
fn reverts_on_insufficient_balance_and_allowance() {
    let (vm, mut token, alice, bob, owner) = setup();
    let id = U256::from(5);

    vm.set_sender(owner);
    token.mint_to(alice, id, U256::from(100)).unwrap();

    vm.set_sender(bob);
    let err = token.transfer_from(bob, alice, id, U256::from(1)).unwrap_err();
    match err {
        Erc6909Error::InsufficientBalance(_) => {}
        _ => panic!("expected InsufficientBalance"),
    }

    let err2 = token.transfer_from(alice, bob, id, U256::from(1)).unwrap_err();
    match err2 {
        Erc6909Error::InsufficientAllowance(_) => {}
        _ => panic!("expected InsufficientAllowance"),
    }
    drop(vm);
}

#[test]
fn only_owner_can_mint_and_burn() {
    let (vm, mut token, alice, _bob, owner) = setup();
    let id = U256::from(9);

    vm.set_sender(alice);
    let err = token.mint(id, U256::from(10)).unwrap_err();
    match err {
        Erc6909Error::ZeroAddress(_) => {}
        _ => panic!("expected owner check failure via error proxy"),
    }

    vm.set_sender(owner);
    token.mint(id, U256::from(10)).unwrap();
    assert_eq!(token.total_supply(id), U256::from(10));
    assert_eq!(token.balance_of(owner, id), U256::from(10));

    token.burn(id, U256::from(5)).unwrap();
    assert_eq!(token.total_supply(id), U256::from(5));
    assert_eq!(token.balance_of(owner, id), U256::from(5));
    drop(vm);
}

/* ===========================
     🔹 EXTRA COVERAGE
=========================== */

#[test]
fn cannot_mint_or_burn_zero_amount() {
    let (vm, mut token, _alice, _bob, owner) = setup();
    let id = U256::from(11);

    vm.set_sender(owner);
    let err1 = token.mint_to(owner, id, U256::ZERO).unwrap_err();
    match err1 {
        Erc6909Error::ZeroAmount(_) => {}
        _ => panic!("expected ZeroAmount"),
    }

    let err2 = token.burn(id, U256::ZERO).unwrap_err();
    match err2 {
        Erc6909Error::ZeroAmount(_) => {}
        _ => panic!("expected ZeroAmount"),
    }
    drop(vm);
}

#[test]
fn operator_revocation_works() {
    let (vm, mut token, alice, bob, owner) = setup();
    let id = U256::from(15);

    vm.set_sender(owner);
    token.mint_to(alice, id, U256::from(100)).unwrap();

    vm.set_sender(alice);
    token.set_operator(bob, true);
    assert!(token.operator_approval(alice, bob));

    token.set_operator(bob, false);
    assert!(!token.operator_approval(alice, bob));
    drop(vm);
}
