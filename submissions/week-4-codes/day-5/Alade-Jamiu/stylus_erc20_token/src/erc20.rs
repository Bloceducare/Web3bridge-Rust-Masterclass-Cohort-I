use stylus_sdk::{
    alloy_primitives::{Address, U256},
    contract, event, external, storage::{StorageMap, StorageString, StorageU8, StorageU256},
};

#[event]
pub struct Transfer {
    from: Address,
    to: Address,
    value: U256,
}

#[event]
pub struct Approval {
    owner: Address,
    spender: Address,
    value: U256,
}

#[storage]
pub struct ERC20 {
    name: StorageString,
    symbol: StorageString,
    decimals: StorageU8,
    total_supply: StorageU256,
    balances: StorageMap<Address, StorageU256>,
    allowances: StorageMap<(Address, Address), StorageU256>,
}

#[contract]
impl ERC20 {
    // Constructor to initialize token parameters
    #[constructor]
    pub fn new(name: String, symbol: String, decimals: u8, initial_supply: U256) {
        let mut storage = Self::storage_mut();
        storage.name.set(name);
        storage.symbol.set(symbol);
        storage.decimals.set(decimals);
        storage.total_supply.set(initial_supply);
        storage.balances.insert(msg::sender(), initial_supply);
        Transfer {
            from: Address::default(),
            to: msg::sender(),
            value: initial_supply,
        }
        .emit();
    }

    // Read-only functions
    #[external]
    pub fn name(&self) -> String {
        self.storage().name.get()
    }

    #[external]
    pub fn symbol(&self) -> String {
        self.storage().symbol.get()
    }

    #[external]
    pub fn decimals(&self) -> u8 {
        self.storage().decimals.get()
    }

    #[external]
    pub fn total_supply(&self) -> U256 {
        self.storage().total_supply.get()
    }

    #[external]
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.storage().balances.get(&owner).unwrap_or_default()
    }

    #[external]
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.storage().allowances.get(&(owner, spender)).unwrap_or_default()
    }

    // Write functions
    #[external]
    pub fn transfer(&mut self, to: Address, value: U256) -> Result<(), String> {
        let sender = msg::sender();
        let mut storage = self.storage_mut();

        let sender_balance = storage.balances.get(&sender).unwrap_or_default();
        if sender_balance < value {
            return Err("Insufficient balance".into());
        }

        storage.balances.insert(sender, sender_balance - value);
        let to_balance = storage.balances.get(&to).unwrap_or_default();
        storage.balances.insert(to, to_balance + value);

        Transfer {
            from: sender,
            to,
            value,
        }
        .emit();

        Ok(())
    }

    #[external]
    pub fn approve(&mut self, spender: Address, value: U256) -> bool {
        let owner = msg::sender();
        let mut storage = self.storage_mut();

        storage.allowances.insert((owner, spender), value);

        Approval {
            owner,
            spender,
            value,
        }
        .emit();

        true
    }

    #[external]
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        value: U256,
    ) -> Result<(), String> {
        let spender = msg::sender();
        let mut storage = self.storage_mut();

        let allowance = storage.allowances.get(&(from, spender)).unwrap_or_default();
        if allowance < value {
            return Err("Insufficient allowance".into());
        }

        let from_balance = storage.balances.get(&from).unwrap_or_default();
        if from_balance < value {
            return Err("Insufficient balance".into());
        }

        storage.allowances.insert((from, spender), allowance - value);
        storage.balances.insert(from, from_balance - value);
        let to_balance = storage.balances.get(&to).unwrap_or_default();
        storage.balances.insert(to, to_balance + value);

        Transfer {
            from,
            to,
            value,
        }
        .emit();

        Ok(())
    }
}