use stylus_sdk::prelude::*;
use stylus_sdk::alloy_primitives::{Address, U256, Uint};
// Events
sol! {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

sol_storage! {
    #[entrypoint]
    pub struct ERC20Token {
        
        string name;
        string symbol;
        uint8 decimals;
        uint256 total_supply;
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
        address owner;
    }
}

#[public]
impl ERC20Token {
    /// Constructor - initializes the token
    pub fn init(
        &mut self, 
        name: String, 
        symbol: String, 
        decimals: u8, 
        initial_supply: U256
    ) -> Result<(), Vec<u8>> {
        let sender = self.vm().msg_sender();
        
        self.name.set_str(name);
        self.symbol.set_str(symbol);
        self.decimals.set(decimals);
        self.total_supply.set(initial_supply);
        self.owner.set(sender);
        
        // Give initial supply to contract deployer
        self.balances.setter(sender).set(initial_supply);

        // Emit Transfer event from zero address
        self.vm().raw_log(Transfer {
            from: Address::ZERO,
            to: sender,
            value: initial_supply,
        });
        
        Ok(())
    }

    /// Returns the name of the token
    pub fn name(&self) -> Result<String, Vec<u8>> {
        Ok(self.name.get_string())
    }

    /// Returns the symbol of the token
    pub fn symbol(&self) -> Result<String, Vec<u8>> {
        Ok(self.symbol.get_string())
    }

    /// Returns the number of decimals
    pub fn decimals(&self) -> Result<u8, Vec<u8>> {
        Ok(self.decimals.get())
    }

    /// Returns the total supply of tokens
    pub fn total_supply(&self) -> Result<U256, Vec<u8>> {
        Ok(self.total_supply.get())
    }

    /// Returns the balance of the given address
    pub fn balance_of(&self, owner: Address) -> Result<U256, Vec<u8>> {
        Ok(self.balances.get(owner))
    }

    /// Transfers tokens from sender to recipient
    pub fn transfer(&mut self, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let from = self.vm().msg_sender();
        self._transfer(from, to, amount)?;
        Ok(true)
    }

    /// Returns the allowance of spender for owner's tokens
    pub fn allowance(&self, owner: Address, spender: Address) -> Result<U256, Vec<u8>> {
        Ok(self.allowances.getter(owner).get(spender))
    }

    /// Approves spender to spend amount of sender's tokens
    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let owner = self.vm().msg_sender();
        self._approve(owner, spender, amount)?;
        Ok(true)
    }

    /// Transfers tokens from owner to recipient using allowance
    pub fn transfer_from(&mut self, from: Address, to: Address, amount: U256) -> Result<bool, Vec<u8>> {
        let spender = self.vm().msg_sender();
        // Check allowance
        let current_allowance = self.allowances.getter(from).get(spender);
        if current_allowance < amount {
            return Err("ERC20: insufficient allowance".as_bytes().to_vec());
        }

        // Update allowance (unless it's max value, which represents unlimited approval)
        if current_allowance != U256::MAX {
            let new_allowance = current_allowance - amount;
            self.allowances.setter(from).setter(spender).set(new_allowance);
        }

        // Execute transfer
        self._transfer(from, to, amount)?;
        Ok(true)
    }

    /// Mint new tokens (owner only)
    pub fn mint(&mut self, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        let sender = self.vm().msg_sender();
        if sender != self.owner.get() {
            return Err("ERC20: caller is not the owner".as_bytes().to_vec());
        }

        let new_total_supply = self.total_supply.get() + amount;
        self.total_supply.set(new_total_supply);

        let current_balance = self.balances.get(to);
        self.balances.setter(to).set(current_balance + amount);

        // Emit Transfer event from zero address
        self.vm().raw_log(Transfer {
            from: Address::ZERO,
            to,
            value: amount,
        });

        Ok(())
    }

    /// Burn tokens from sender's balance
    pub fn burn(&mut self, amount: U256) -> Result<(), Vec<u8>> {
        let sender = self.vm().msg_sender();
        let current_balance = self.balances.get(sender);

        if current_balance < amount {
            return Err("ERC20: burn amount exceeds balance".as_bytes().to_vec());
        }

        self.balances.setter(sender).set(current_balance - amount);
        self.total_supply.set(self.total_supply.get() - amount);

        // Emit Transfer event to zero address  
        self.vm().raw_log(Transfer {
            from: sender,
            to: Address::ZERO,
            value: amount,
        });

        Ok(())
    }

    /// Returns the owner of the contract
    pub fn owner(&self) -> Result<Address, Vec<u8>> {
        Ok(self.owner.get())
    }
}

impl ERC20Token {
    /// Internal transfer function
    fn _transfer(&mut self, from: Address, to: Address, amount: U256) -> Result<(), Vec<u8>> {
        if from == Address::ZERO {
            return Err("ERC20: transfer from the zero address".as_bytes().to_vec());
        }
        if to == Address::ZERO {
            return Err("ERC20: transfer to the zero address".as_bytes().to_vec());
        }

        let from_balance = self.balances.get(from);
        if from_balance < amount {
            return Err("ERC20: transfer amount exceeds balance".as_bytes().to_vec());
        }

        // Update balances
        self.balances.setter(from).set(from_balance - amount);
        let to_balance = self.balances.get(to);
        self.balances.setter(to).set(to_balance + amount);

        // Emit Transfer event
        self.vm().raw_log(Transfer {
            from,
            to,
            value: amount,
        });

        Ok(())
    }

    /// Internal approve function
    fn _approve(&mut self, owner: Address, spender: Address, amount: U256) -> Result<(), Vec<u8>> {
        if owner == Address::ZERO {
            return Err("ERC20: approve from the zero address".as_bytes().to_vec());
        }
        if spender == Address::ZERO {
            return Err("ERC20: approve to the zero address".as_bytes().to_vec());
        }

        self.allowances.setter(owner).setter(spender).set(amount);

        // Emit Approval event
        self.vm().raw_log(Approval {
            owner,
            spender,
            value: amount,
        });

        Ok(())
    }
}