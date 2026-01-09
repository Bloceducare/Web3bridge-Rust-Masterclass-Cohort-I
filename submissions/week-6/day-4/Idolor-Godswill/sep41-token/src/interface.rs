use soroban_sdk::{contracttrait, Address, Env, String};

/// SEP-41 Token Interface
/// 
/// This trait defines the standard interface for tokens on Stellar Soroban
/// as specified in SEP-41. All compliant tokens should implement this interface
/// to ensure interoperability with other contracts and applications.
#[contracttrait]
pub trait TokenInterface {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens to be drawn from.
    /// * `spender` - The address being authorized to spend the tokens held by `from`.
    /// * `amount` - The tokens to be made available to `spender`.
    /// * `expiration_ledger` - The ledger number where this allowance expires.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address, spender: Address]`,
    /// data = `[amount: i128, expiration_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The address for which a balance is being queried.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address]`,
    /// data = `amount: i128`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the transfer.
    /// * `from` - The address holding the balance of tokens which will be withdrawn from.
    /// * `to` - The address which will receive the transferred tokens.
    /// * `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["transfer", from: Address, to: Address]`,
    /// data = `amount: i128`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// * `from` - The address holding the balance of tokens which will be burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address]`, data = `amount: i128`
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// * `spender` - The address authorizing the burn.
    /// * `from` - The address holding the balance of tokens which will be burned from.
    /// * `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["burn", from: Address]`, data = `amount: i128`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Returns the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    fn symbol(env: Env) -> String;
}

/// Administrative interface for token management
/// 
/// This trait defines additional functions for token administration
/// that are not part of the core SEP-41 interface but are commonly needed.
#[contracttrait]
pub trait TokenAdminInterface {
    /// Initialize the token contract with metadata and admin.
    ///
    /// # Arguments
    ///
    /// * `admin` - The address that will have administrative privileges.
    /// * `name` - The name of the token.
    /// * `symbol` - The symbol of the token.
    /// * `decimals` - The number of decimal places for the token.
    fn initialize(env: Env, admin: Address, name: String, symbol: String, decimals: u32);

    /// Mint new tokens to a specified address.
    ///
    /// # Arguments
    ///
    /// * `to` - The address to mint tokens to.
    /// * `amount` - The amount of tokens to mint.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["mint", to: Address]`, data = `amount: i128`
    fn mint(env: Env, to: Address, amount: i128);

    /// Set a new admin for the contract.
    ///
    /// # Arguments
    ///
    /// * `new_admin` - The address of the new admin.
    fn set_admin(env: Env, new_admin: Address);

    /// Get the current admin address.
    fn admin(env: Env) -> Address;
}
