use alloc::{vec, vec::Vec};
use alloy_primitives::aliases::B32;
// use alloy_primitives::{aliases::B32,Address,U256};
// use stylus_sdk::{call::MethodCall,evm,log,prelude::*,storage::{StorageMap,StorageU256}};
pub use sol::*;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::{Call, Error, MethodError},
    evm, msg,
    prelude::*,
    storage::{StorageMap, StorageU256},
};

mod sol {
    use alloy_sol_types::sol;

    sol! {
        #[derive(Debug)]
        #[allow(missing_docs)]
        event Transfer(address indexed from , address indexed to , uint256 value);

        #[derive(Debug)]
        #[allow(missing_docs)]
        event Approval(address indexed owner,address indexed spender,uint256 value);
    }

    sol! {
        #[derive(Debug)]
        #[allow(missing_docs)]
        error ERCInsufficientBalance(address sender, uint256 balance,uint256 needed);

        #[derive(Debug)]
        #[allow(missing_docs)]
        error ERC20InvalidSender(address sender);

        #[derive(Debug)]
        #[allow(missing_docs)]
        error ERC20InvalidReceiver(address receiver);

        #[derive(Debug)]
        #[allow(missing_docs)]
        error ERC20InsufficientAllowance(address spender, uint256 allowance, uint256 needed);

        #[derive(Debug)]
        #[allow(missing_docs)]
        error ERC20InsufficientApprover(address approver);

        #[derive(Debug)]
        #[allow(missing_docs)]
        error ERC20InvalidApprover(address approver);
    }
}

#[derive(SolidityError, Debug)]
pub enum Erc20Error {
    InsufficientBalance(ERCInsufficientBalance),
    InvalidSender(ERC20InvalidSender),
    InvalidReceiver(ERC20InvalidReceiver),
    InsufficientAllowance(ERC20InsufficientAllowance),
    InsufficientApprover(ERC20InsufficientApprover),
    InvalidApprover(ERC20InvalidApprover),
}

impl MethodError for Erc20Error {
    fn encode(self) -> alloc::vec::Vec<u8> {
        self.into()
    }
}

#[storage]
pub struct Erc20 {
    pub(crate) balances: StorageMap<Address, StorageU256>,
    pub(crate) allowances: StorageMap<Address, StorageMap<Address, StorageU256>>,
    pub(crate) total_supply: StorageU256,
}

unsafe impl TopLevelStorage for Erc20 {}

#[interface_id]
pub trait IErc20 {
    type Error: Into<alloc::vec::Vec<u8>>;

    fn total_supply(&self) -> U256;

    fn balance_of(&self, account: Address) -> U256;

    fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Self::Error>;

    fn allowance(&self, owner: Address, spender: Address) -> U256;

    fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Self::Error>;

    fn transfer_from(&mut self, from: Address, to: Address, value: U256);
}

#[public]
#[implements(IErc20<Error=Erc20Error>)]
impl Erc20 {}

#[public]
impl IErc20 for Erc20 {
    type Error = Erc20Error;

    fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    fn balance_of(&self, account: Address) -> U256 {
        self.balances.get(account)
    }

    fn transfer(&mut self, to: Address, value: U256) -> Result<bool, Self::Error> {
        let from = msg::sender();
        self._transfer(from, to, value)?;
        Ok(true)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.allowances.get(owner).get(spender)
    }

    fn approve(&mut self, spender: Address, value: U256) -> Result<bool, Self::Erc20Error> {
        let owner = msg::sender();
        self._approve(owner, spender, value, true);
    }
    fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<bool, Self::Erc20Error> {
        let spender = msg::sender();
        self._spend_allowance(from, spender, value)?;
        self._transfer(from, to, value)?;
        Ok(true)
    }
}

impl Erc20 {
    fn _approve(
        &mut self,
        owner: Address,
        spender: Address,
        value: U256,
        emit_event: bool,
    ) -> Result<bool, Erc20Error> {
        if owner.is_zero() {
            return Err(Erc20Error::InvalidApprover(ERC20InvalidApprover {
                approver: Address::ZERO,
            }));
        }
        if spender.is_zero() {
            return Err(Erc20Error::InvalidSender(ERC20InvalidSender {
                sender: Address::ZERO,
            }));
        }
        self.allowances.setter(owner).insert(spender, value);
        if emit_event {
            evm::log(Approval {
                owner,
                spender,
                value,
            });
        };
    }

    fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        if from.is_zero() {
            return Err(Erc20Error::InvalidSender(ERC20InvalidSender {
                sender: Address::ZERO,
            }));
        }
        if to.is_zero() {
            return Err(Erc20Error::InvalidReceiver(ERC20InvalidReceiver {
                receiver: Address::ZERO,
            }));
        }

        self._update(from, to, value)?;

        Ok(())
    }

    pub fn _mint(&mut self, account: Address, value: U256) -> Result<(), Erc20Error> {
        if account.is_zero() {
            return Err(Erc20Error::InvalidReceiver(ERC20InvalidReceiver {
                receiver: Address::ZERO,
            }));
        }
        self._update(Address::ZERO, account, value)
    }
    pub fn _update(&mut self, from: Address, to: Address, value: U256) -> Result<(), Erc20Error> {
        if from.is_zero() {
            // Mint operation. Overflow check required: the rest of the code
            // assumes that `total_supply` never overflows.
            self.total_supply
                .add_assign_checked(value, "should not exceed `U256::MAX` for `total_supply`");
        } else {
            let from_balance = self.balances.get(from);
            if from_balance < value {
                return Err(Erc20Error::InsufficientBalance(ERCInsufficientBalance {
                    sender: from,
                    balance: from_balance,
                    needed: value,
                }));
            }
            // Overflow not possible:
            // `value` <= `from_balance` <= `total_supply`.
            self.balances.setter(from).set(from_balance - value);
        }

        if to.is_zero() {
            // Overflow not possible:
            // `value` <= `total_supply` or
            // `value` <= `from_balance` <= `total_supply`.
            self.total_supply.sub_assign_unchecked(value);
        } else {
            // Overflow not possible:
            // `balance_to` + `value` is at most `total_supply`,
            // which fits into a `U256`.
            self.balances.setter(to).add_assign_unchecked(value);
        }

        evm::log(Transfer { from, to, value });

        Ok(())
    }

    pub fn _burn(&mut self, account: Address, value: U256) -> Result<(), Erc20Error> {
        if account == Address::ZERO {
            return Err(Erc20Error::InvalidSender(ERC20InvalidSender {
                sender: Address::ZERO,
            }));
        }
        self._update(account, Address::ZERO, value)
    }
}

impl IErc165 for Erc20 {
    fn supports_interface(&self, interface_id: B32) -> bool {
        <Self as IErc20>::interface_id() == interface_id
            || <Self as IErc165>::interface_id() == interface_id
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{uint, Address, U256};
    use alloy_sol_types::sol_data::Uint;
    use motsu::prelude::*;

    use super::*;

    #[motsu::test]
    fn mint(contract: Contract<Erc20>, alice: Address) {
        let one = Uint::from(1);
        let initial_balance = contract.sender(alice).balance_of(alice);

        let result = contract.sender(alice)._mint(alice, one);

        assert!(result.is_ok());

        assert_eq!(initial_balance + one, contract.sender(alice).total_supply())
    }
}
