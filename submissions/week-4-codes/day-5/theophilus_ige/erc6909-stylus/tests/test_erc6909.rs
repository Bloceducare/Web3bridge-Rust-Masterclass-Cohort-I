use stylus::prelude::*;
use erc6909_stylus::ERC6909;

#[test]
fn test_mint_transfer_approve_operator() {
    let mut contract = ERC6909::default();

    let alice = Address::from_low_u64_be(1);
    let bob = Address::from_low_u64_be(2);
    let charlie = Address::from_low_u64_be(3);
    let token_id = U256::from(100);

    // Mint 100 tokens to Alice
    ctx::set_caller(alice);
    assert!(contract.mint(alice, token_id, U256::from(100)));
    assert_eq!(contract.balance_of(alice, token_id), U256::from(100));

    // Transfer 40 tokens to Bob
    assert!(contract.transfer_from(alice, bob, token_id, U256::from(40)));
    assert_eq!(contract.balance_of(alice, token_id), U256::from(60));
    assert_eq!(contract.balance_of(bob, token_id), U256::from(40));

    // Approve Charlie for 20 tokens
    assert!(contract.approve(charlie, token_id, U256::from(20)));
    assert_eq!(contract.allowance(alice, charlie, token_id), U256::from(20));

    // Transfer via Charlie
    ctx::set_caller(charlie);
    assert!(contract.transfer_from(alice, bob, token_id, U256::from(15)));
    assert_eq!(contract.balance_of(alice, token_id), U256::from(45));
    assert_eq!(contract.balance_of(bob, token_id), U256::from(55));
    assert_eq!(contract.allowance(alice, charlie, token_id), U256::from(5));

    // Set Bob as operator
    ctx::set_caller(alice);
    assert!(contract.set_operator(bob, true));
    assert!(contract.operator_approval(alice, bob));

    // Transfer via operator
    ctx::set_caller(bob);
    assert!(contract.transfer_from(alice, bob, token_id, U256::from(10)));
    assert_eq!(contract.balance_of(alice, token_id), U256::from(35));
    assert_eq!(contract.balance_of(bob, token_id), U256::from(65));
}
