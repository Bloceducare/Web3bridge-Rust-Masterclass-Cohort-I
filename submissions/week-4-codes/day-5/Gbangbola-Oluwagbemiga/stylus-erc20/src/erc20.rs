use crate::storage::ERC20Token;
use alloc::{string::String, vec::Vec};
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    evm, msg,
    prelude::*,
};

// Define interface
stylus_sdk::sol_interface! {
    interface IERC20 {
        function name() external view returns (string memory);
        function symbol() external view returns (string memory);
        function decimals() external view returns (uint8);
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
        function allowance(address owner, address spender) external view returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address from, address to, uint256 amount) external returns (bool);
    }
}

// Define events
stylus_sdk::sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

#[public]
impl IERC20 for ERC20Token {
    fn name(&self) -> Result<String, Vec<u8>> {
        Ok(self.name.get_string())
    }

    fn symbol(&self) -> Result<String, Vec<u8>> {
        Ok(self.symbol.get_string())
    }

    fn decimals(&self) -> Result<u8, Vec<u8>> {
        Ok(self.decimals.get())
    }

    fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    fn balance_of(&self, account: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(account).get())
    }

    fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let from = msg::sender();
        self.internal_transfer(from, to, amount)?;
        Ok(true)
    }

    fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        Ok(self.allowances.get(owner).get(spender).get())
    }

    fn approve(&mut self, spender: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let owner = msg::sender();
        self.internal_approve(owner, spender, amount)?;
        Ok(true)
    }

    fn transfer_from(&mut self, from: Address, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let spender = msg::sender();

        let mut owner_allowances = self.allowances.setter(from);
        let current_allowance = owner_allowances.get(spender).get();

        if current_allowance < amount {
            return Err(b"ERC20: insufficient allowance".to_vec());
        }

        let new_allowance = current_allowance - amount;
        owner_allowances.insert(spender, StorageU256::from(new_allowance));

        self.internal_transfer(from, to, amount)?;

        Ok(true)
    }
}

impl ERC20Token {
    pub fn init(
        &mut self,
        name: String,
        symbol: String,
        decimals: u8,
        initial_supply: U256,
    ) -> Result<(), Vec<u8>> {
        self.name.set_str(&name);
        self.symbol.set_str(&symbol);
        self.decimals.set(decimals);
        self.total_supply.set(initial_supply);

        let deployer = msg::sender();
        self.balances
            .insert(deployer, StorageU256::from(initial_supply));

        evm::log(Transfer {
            from: Address::ZERO,
            to: deployer,
            value: initial_supply,
        });

        Ok(())
    }

    fn internal_transfer(
        &mut self,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<(), Vec<u8>> {
        if from == Address::ZERO {
            return Err(b"ERC20: transfer from zero address".to_vec());
        }
        if to == Address::ZERO {
            return Err(b"ERC20: transfer to zero address".to_vec());
        }

        let from_balance = self.balances.get(from).get();
        if from_balance < amount {
            return Err(b"ERC20: transfer amount exceeds balance".to_vec());
        }

        self.balances
            .insert(from, StorageU256::from(from_balance - amount));
        let to_balance = self.balances.get(to).get();
        self.balances
            .insert(to, StorageU256::from(to_balance + amount));

        evm::log(Transfer {
            from,
            to,
            value: amount,
        });

        Ok(())
    }

    fn internal_approve(
        &mut self,
        owner: Address,
        spender: Address,
        amount: U256,
    ) -> Result<(), Vec<u8>> {
        if owner == Address::ZERO {
            return Err(b"ERC20: approve from zero address".to_vec());
        }
        if spender == Address::ZERO {
            return Err(b"ERC20: approve to zero address".to_vec());
        }

        let mut owner_allowances = self.allowances.setter(owner);
        owner_allowances.insert(spender, StorageU256::from(amount));

        evm::log(Approval {
            owner,
            spender,
            value: amount,
        });

        Ok(())
    }

    pub fn mint(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        if to == Address::ZERO {
            return Err(b"ERC20: mint to zero address".to_vec());
        }

        let current_supply = self.total_supply.get();
        self.total_supply.set(current_supply + amount);

        let balance = self.balances.get(to).get();
        self.balances
            .insert(to, StorageU256::from(balance + amount));

        evm::log(Transfer {
            from: Address::ZERO,
            to,
            value: amount,
        });

        Ok(())
    }

    pub fn burn(&mut self, from: Address, amount: U256) -> Result<(), Vec<u8>> {
        if from == Address::ZERO {
            return Err(b"ERC20: burn from zero address".to_vec());
        }

        let balance = self.balances.get(from).get();
        if balance < amount {
            return Err(b"ERC20: burn amount exceeds balance".to_vec());
        }

        self.balances
            .insert(from, StorageU256::from(balance - amount));

        let current_supply = self.total_supply.get();
        self.total_supply.set(current_supply - amount);

        evm::log(Transfer {
            from,
            to: Address::ZERO,
            value: amount,
        });

        Ok(())
    }
}

// Constructor function
#[external]
pub fn constructor(
    &mut self,
    name: String,
    symbol: String,
    decimals: u8,
    initial_supply: U256,
) -> Result<(), Vec<u8>> {
    self.init(name, symbol, decimals, initial_supply)?;
    Ok(())
}
