


#[cfg(test)]
mod test {
    // use super::*;
    use alloc::string::ToString;
    use alloy_primitives::U160;
    use crate::erc20::ERC20;
    use stylus_sdk::{alloy_primitives::Address, testing::*};
    use crate::ierc20::IERC20;

    fn create_erc20_instance() -> (ERC20, Address, TestVM) {
        let vm: TestVM = TestVM::default();
        let mut erc20 = ERC20::from(&vm);
        erc20
            .init("Stylus".to_string(), "sty".to_string(), 1_000_000_000)
            .unwrap();
        (erc20, vm.msg_sender(), vm)
    }

    #[test]
    fn test_get_decimals() {
        let (erc20, owner, _) = create_erc20_instance();
        assert_eq!(erc20.decimals(), 18);
        assert_eq!(erc20.name(), "Stylus");
        // assert_eq!(erc20.owner.get(), owner);
        assert_eq!(erc20.total_supply(), 1_000_000_000);
        assert_eq!(erc20.balance_of(owner), 1_000_000_000);
    }

    #[test]
    // #[should_panic(expected = "[110, 111, 116, 32, 114, 101, 97, 100, 121]")]
    fn test_transfer() {
        let (mut erc20, owner, _) = create_erc20_instance();
        let to: Address = Address::from(U160::from(0x0000000000000000000000000000000000000001));

        erc20.transfer(to, 100).unwrap();
        assert_eq!(erc20.balance_of(owner), 999_999_900);
        assert_eq!(erc20.balance_of(to), 100);
    }

    #[test]
    #[should_panic]
    fn test_transfer_error() {
        let (mut erc20, owner, vm) = create_erc20_instance();
        let to: Address = Address::from(U160::from(0x0000000000000000000000000000000000000001));
        // cannot transfer to self
        erc20.transfer(owner, 100).unwrap();

        // cannot send 0
        erc20.transfer(to, 0).unwrap();
        // assert_eq!(erc20.balance_of(owner), 1_000_000_000);

        //cannot transfer from 0 balance
        vm.set_sender(to);
        erc20.transfer(owner, 90).unwrap();
    }

    #[test]
    fn test_transfer_from_and_approve() {
        let (mut erc20, owner, vm) = create_erc20_instance();
        let to: Address = Address::from(U160::from(0x0000000000000000000000000000000000000001));

        erc20.approve(to, 100).unwrap();
        assert_eq!(erc20.allowance(owner, to), 100);

        vm.set_sender(to);
        erc20
            .transfer_from(owner, vm.contract_address(), 100)
            .unwrap();
        assert_eq!(erc20.balance_of(owner), 999_999_900);
        assert_eq!(erc20.balance_of(vm.contract_address()), 100);
        assert_eq!(erc20.balance_of(to), 0);
    }

    #[test]
    #[should_panic]
    fn test_cannot_approve_with_insufficient_balance() {
        let (mut erc20, owner, vm) = create_erc20_instance();
        let to: Address = Address::from(U160::from(1));
        vm.set_sender(to);
        erc20.approve(owner, 100).unwrap();
        // assert_eq!(erc20.allowance(owner, to), 100);
    }
}
