// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std)]

#[macro_use]
extern crate alloc;

use alloc::{string::String, vec::Vec};
use alloy_primitives::{Address, U256};
use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

// Define some persistent storage using the Solidity ABI.
// `Counter` will be the entrypoint.
sol_storage! {
    #[entrypoint]
    pub struct ERC20 {
       address owner;
       uint256 total_supply;
       string name;
       string symbol;
       uint8 decimals;
       mapping(address => uint256) balances;
       mapping(address => mapping(address => uint256)) allowances;
    }
}

sol! {
    #[derive(Debug)]
    error ERC20InsufficientBalance(address sender, uint256 balance, uint256 needed);
    
    #[derive(Debug)]
    error ERC20InvalidSender(address sender);
    
    #[derive(Debug)]
    error ERC20InvalidReciver(address reciver);
    
    #[derive(Debug)]
    error ERC20InsufficientAllowance(address owner, uint256 allowance, uint256 needed);

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

// Define the Rust-equivalent of the Solidity errors
#[derive(SolidityError, Debug)]
pub enum ERC20Error {
    ERC20InsufficientBalance(ERC20InsufficientBalance),
    ERC20InvalidSender(ERC20InvalidSender),
    ERC20InvalidReciver(ERC20InvalidReciver),
    ERC20InsufficientAllowance(ERC20InsufficientAllowance),
}

impl ERC20 {
    fn _update(&mut self, from: Address, to: Address, value: U256) -> Result<(), ERC20Error> {
        if from.is_zero() {
            if to.is_zero() {
                return Err(ERC20Error::ERC20InvalidReciver(ERC20InvalidReciver {
                    reciver: to,
                }));
            }
            let mut to_balance = self.balances.setter(to);
            let old_to_balance = to_balance.get();
            to_balance.set(old_to_balance + value);
            
            // Update total supply for minting
            let current_supply = self.total_supply.get();
            self.total_supply.set(current_supply + value);
            
            log(self.vm(), Transfer { from, to, value });
            return Ok(());
        }
        
        if to.is_zero() {
            let mut from_balance = self.balances.setter(from);
            let old_from_balance = from_balance.get();
            if old_from_balance < value {
                return Err(ERC20Error::ERC20InsufficientBalance(
                    ERC20InsufficientBalance {
                        sender: from,
                        balance: old_from_balance,
                        needed: value,
                    },
                ));
            }
            from_balance.set(old_from_balance - value);
            
            // Update total supply for burning
            let current_supply = self.total_supply.get();
            self.total_supply.set(current_supply - value);
            
            log(self.vm(), Transfer { from, to, value });
            return Ok(());
        }
        
        // Handle normal transfers (both from and to are non-zero)
        let mut from_balance = self.balances.setter(from);
        let old_from_balance = from_balance.get();
        if old_from_balance < value {
            return Err(ERC20Error::ERC20InsufficientBalance(
                ERC20InsufficientBalance {
                    sender: from,
                    balance: old_from_balance,
                    needed: value,
                },
            ));
        }
        from_balance.set(old_from_balance - value);
        let mut to_balance = self.balances.setter(to);
        let old_to_balance = to_balance.get();
        to_balance.set(old_to_balance + value);

        log(self.vm(), Transfer { from, to, value });

        Ok(())
    }

    fn _transfer(&mut self, from: Address, to: Address, value: U256) -> Result<(), ERC20Error> {
        // check the sender is not the zero address
        if from.is_zero() {
            return Err(ERC20Error::ERC20InvalidSender(ERC20InvalidSender {
                sender: from,
            }));
        }
        // check the reciver is not the zero address
        if to.is_zero() {
            return Err(ERC20Error::ERC20InvalidReciver(ERC20InvalidReciver {
                reciver: to,
            }));
        }

        self._update(from, to, value)
    }

    fn _approve(&mut self, owner: Address, spender: Address, value: U256) -> Result<(), ERC20Error> {
        if owner.is_zero() {
            return Err(ERC20Error::ERC20InvalidSender(ERC20InvalidSender {
                sender: owner,
            }));
        }

        if spender.is_zero() {
            return Err(ERC20Error::ERC20InvalidReciver(ERC20InvalidReciver {
                reciver: spender,
            }));
        }

        self.allowances.setter(owner).insert(spender, value);
        log(self.vm(), Approval { owner, spender, value });
        Ok(())
    }

    fn _spend_allowance(&mut self, owner: Address, spender: Address, value: U256) -> Result<(), ERC20Error> {
      let mut old_allowance = self.allowances.setter(owner);
      let allowance = old_allowance.get(spender);
      if allowance < value {
        return Err(ERC20Error::ERC20InsufficientAllowance(ERC20InsufficientAllowance {
          owner,
          allowance: allowance,
          needed: value,
        }));
      }

      old_allowance.insert(spender, allowance - value);
      Ok(())
    }

    fn _mint(&mut self, to: Address, value: U256) -> Result<(), ERC20Error> {
      if to.is_zero() {
        return Err(ERC20Error::ERC20InvalidReciver(ERC20InvalidReciver {
          reciver: to,
        }));
      }

      self._update(Address::ZERO, to, value)
    }

    fn _burn(&mut self, from: Address, value: U256) -> Result<(), ERC20Error> {
      if from.is_zero() {
        return Err(ERC20Error::ERC20InvalidSender(ERC20InvalidSender {
          sender: from,
        }));
      }

      self._update(from, Address::ZERO, value)
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl ERC20 {
    #[constructor]
    pub fn constructor(&mut self, name: String, symbol: String) {
        self.name.set_str(name);
        self.symbol.set_str(symbol);
        self.owner.set(self.vm().msg_sender());
    }

    fn name(&self) -> String {
        self.name.get_string()
    }

    fn symbol(&self) -> String {
        self.symbol.get_string()
    }

    fn decimals(&self) -> u8 {
        u8::from(18)
    }

    fn total_supply(&self) -> U256 {
        self.total_supply.get()
    }

    fn balance_of(&self, owner: Address) -> U256 {
        self.balances.get(owner)
    }

    fn transfer(&mut self, to: Address, value: U256) -> Result<(), ERC20Error> {
        self._transfer(self.vm().msg_sender(), to, value)
    }

    fn allowance(&self, owner: Address, spender: Address) -> U256 {
        let allowance = self.allowances.getter(owner).get(spender);
        allowance
    }

    fn approve(&mut self, spender: Address, value: U256) -> Result<(), ERC20Error> {
        self._approve(self.vm().msg_sender(), spender, value)
    }

    fn transfer_from(&mut self, from: Address, to: Address, value: U256) -> Result<(), ERC20Error> {
        self._spend_allowance(from, self.vm().msg_sender(), value)?;
        self._transfer(from, to, value)
    }

    fn mint(&mut self, to: Address, value: U256) -> Result<(), ERC20Error> {
        if self.vm().msg_sender() != self.owner.get() {
            return Err(ERC20Error::ERC20InvalidSender(ERC20InvalidSender {
                sender: self.vm().msg_sender(),
            }));
        }
        self._mint(to, value)
    }

    fn burn(&mut self, value: U256) -> Result<(), ERC20Error> {
        self._burn(self.vm().msg_sender(), value)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn emit_log(_pointer: *const u8, _len: usize, _: usize) {}

    #[test]
    fn test_erc20() {
      use stylus_sdk::testing::*;
      let vm = TestVM::default();
      let mut contract = ERC20::from(&vm);
      contract.constructor(String::from("Test"), String::from("TT"));
      assert_eq!(contract.name(), "Test");
      assert_eq!(contract.symbol(), "TT");
      assert_eq!(contract.decimals(), 18);
      assert_eq!(contract.total_supply(), U256::from(0));
      assert_eq!(contract.balance_of(Address::ZERO), U256::from(0));
    }

    #[test]
    fn test_transfer() { 
      use stylus_sdk::testing::*;
      let vm = TestVM::default();
      let mut contract = ERC20::from(&vm);
      contract.constructor(String::from("Test"), String::from("TT"));
      
      let recipient = Address::from([2; 20]);
      let third_party = Address::from([3; 20]);
      
      println!("Contract owner: {}", contract.owner.get());
      println!("Current msg_sender: {}", contract.vm().msg_sender());
      
      // Test minting (only owner can mint)
      println!("Testing mint functionality...");
      let mint_result = contract.mint(recipient, U256::from(1000));
      println!("Mint result: {:?}", mint_result);
      assert!(mint_result.is_ok(), "Mint should succeed");
      
      // Check balances after mint
      let balance = contract.balance_of(recipient);
      println!("Recipient balance after mint: {}", balance);
      assert_eq!(balance, U256::from(1000));
      
      let total_supply = contract.total_supply();
      println!("Total supply after mint: {}", total_supply);
      assert_eq!(total_supply, U256::from(1000));
      
      // Test minting to another address
      println!("Testing mint to third party...");
      let mint_result2 = contract.mint(third_party, U256::from(500));
      println!("Second mint result: {:?}", mint_result2);
      assert!(mint_result2.is_ok(), "Second mint should succeed");
      
      let third_party_balance = contract.balance_of(third_party);
      println!("Third party balance after mint: {}", third_party_balance);
      assert_eq!(third_party_balance, U256::from(500));
      
      let total_supply_after = contract.total_supply();
      println!("Total supply after second mint: {}", total_supply_after);
      assert_eq!(total_supply_after, U256::from(1500));
      
      println!("All mint tests passed!");
     }

    #[test]
    fn test_burn_functionality() {
      use stylus_sdk::testing::*;
      let vm = TestVM::default();
      let mut contract = ERC20::from(&vm);
      contract.constructor(String::from("TestToken"), String::from("TEST"));
      
      let current_sender = contract.vm().msg_sender();
      println!("Contract owner: {}", contract.owner.get());
      println!("Current msg_sender: {}", current_sender);
      
      // Mint tokens to current sender (who is the owner)
      println!("Minting 1000 tokens to current sender...");
      let mint_result = contract.mint(current_sender, U256::from(1000));
      println!("Mint result: {:?}", mint_result);
      assert!(mint_result.is_ok(), "Mint should succeed");
      println!("Current sender balance: {}", contract.balance_of(current_sender));
      
      // Test burning (sender burning their own tokens)
      println!("Testing burn functionality...");
      let burn_result = contract.burn(U256::from(200));
      println!("Burn result: {:?}", burn_result);
      assert!(burn_result.is_ok(), "Burn should succeed");
      
      let sender_balance_after_burn = contract.balance_of(current_sender);
      println!("Sender balance after burn: {}", sender_balance_after_burn);
      assert_eq!(sender_balance_after_burn, U256::from(800));
      
      let total_supply_after_burn = contract.total_supply();
      println!("Total supply after burn: {}", total_supply_after_burn);
      assert_eq!(total_supply_after_burn, U256::from(800));
      
      // Test burn more than balance (should fail)
      println!("Testing burn more than balance...");
      let burn_fail_result = contract.burn(U256::from(1000));
      println!("Burn fail result: {:?}", burn_fail_result);
      assert!(burn_fail_result.is_err(), "Burn should fail when insufficient balance");
      
      println!("All burn tests passed!");
    }
}
