use stylus::prelude::*;

#[derive(Default, Storage)]
pub struct ERC6909 {
    total_supply: Mapping<U256, U256>,
    balances: Mapping<(Address, U256), U256>,
    allowances: Mapping<(Address, Address, U256), U256>,
    operators: Mapping<(Address, Address), bool>,
}

#[event]
pub struct TransferSingle {
    pub operator: Address,
    pub from: Address,
    pub to: Address,
    pub token_id: U256,
    pub amount: U256,
}

#[event]
pub struct ApprovalSingle {
    pub owner: Address,
    pub spender: Address,
    pub token_id: U256,
    pub amount: U256,
}

#[contract]
impl ERC6909 {
    #[storage_read]
    pub fn total_supply(&self, token_id: U256) -> U256 {
        self.total_supply.get(&token_id).unwrap_or(U256::ZERO)
    }

    #[storage_read]
    pub fn balance_of(&self, owner: Address, token_id: U256) -> U256 {
        self.balances.get(&(owner, token_id)).unwrap_or(U256::ZERO)
    }

    #[storage_write]
    pub fn approve(&mut self, spender: Address, token_id: U256, amount: U256) -> bool {
        let owner = ctx::caller();
        self.allowances.insert(&(owner, spender, token_id), &amount);

        Self::emit(ApprovalSingle {
            owner,
            spender,
            token_id,
            amount,
        });

        true
    }

    #[storage_read]
    pub fn allowance(&self, owner: Address, spender: Address, token_id: U256) -> U256 {
        self.allowances.get(&(owner, spender, token_id)).unwrap_or(U256::ZERO)
    }

    #[storage_write]
    pub fn set_operator(&mut self, operator: Address, approved: bool) -> bool {
        let owner = ctx::caller();
        self.operators.insert(&(owner, operator), &approved);
        true
    }

    #[storage_read]
    pub fn operator_approval(&self, owner: Address, operator: Address) -> bool {
        self.operators.get(&(owner, operator)).unwrap_or(false)
    }

    #[storage_write]
    pub fn transfer_from(
        &mut self,
        from: Address,
        to: Address,
        token_id: U256,
        amount: U256,
    ) -> bool {
        let caller = ctx::caller();

        let is_owner = caller == from;
        let is_operator = self.operator_approval(from, caller);
        let allowed_amount = self.allowance(from, caller, token_id);

        if !is_owner && !is_operator && allowed_amount < amount {
            panic!("ERC6909: Insufficient allowance or not authorized");
        }

        let from_balance = self.balance_of(from, token_id);
        if from_balance < amount {
            panic!("ERC6909: Insufficient balance");
        }

        self.balances.insert(&(from, token_id), &(from_balance - amount));
        let to_balance = self.balance_of(to, token_id);
        self.balances.insert(&(to, token_id), &(to_balance + amount));

        if !is_owner && !is_operator {
            self.allowances
                .insert(&(from, caller, token_id), &(allowed_amount - amount));
        }

        Self::emit(TransferSingle {
            operator: caller,
            from,
            to,
            token_id,
            amount,
        });

        true
    }

    #[storage_write]
    pub fn mint(&mut self, to: Address, token_id: U256, amount: U256) -> bool {
        let balance = self.balance_of(to, token_id);
        self.balances.insert(&(to, token_id), &(balance + amount));

        let supply = self.total_supply(token_id);
        self.total_supply.insert(&token_id, &(supply + amount));

        Self::emit(TransferSingle {
            operator: ctx::caller(),
            from: Address::zero(),
            to,
            token_id,
            amount,
        });

        true
    }

    #[storage_write]
    pub fn burn(&mut self, from: Address, token_id: U256, amount: U256) -> bool {
        let balance = self.balance_of(from, token_id);
        if balance < amount {
            panic!("ERC6909: Insufficient balance to burn");
        }

        self.balances.insert(&(from, token_id), &(balance - amount));
        let supply = self.total_supply(token_id);
        self.total_supply.insert(&token_id, &(supply - amount));

        Self::emit(TransferSingle {
            operator: ctx::caller(),
            from,
            to: Address::zero(),
            token_id,
            amount,
        });

        true
    }
}
