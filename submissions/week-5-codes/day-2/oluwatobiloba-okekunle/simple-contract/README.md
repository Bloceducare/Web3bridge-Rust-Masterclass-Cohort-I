# Simple Todo Contract - Arbitrum Stylus

A basic Todo list smart contract built with Arbitrum Stylus and Rust. This contract demonstrates CRUD operations, event logging, and access control patterns in Stylus.

## Features

- ✅ Create new todos
- ✅ Mark todos as completed
- ✅ Update todo text
- ✅ Delete todos
- ✅ Access control (only creator or owner can modify todos)
- ✅ Event logging for all operations
- ✅ Ownership management

## Quick Start

### Prerequisites

Install Rust and the Stylus CLI:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Stylus CLI
cargo install --force cargo-stylus cargo-stylus-check

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### Contract Interface

The contract provides the following functions:

```rust
// Constructor
pub fn constructor(&mut self)

// Todo operations
pub fn create_todo(&mut self, text: String) -> Result<U256, TodoError>
pub fn get_todo(&self, id: U256) -> Result<(U256, String, bool, Address, U256), TodoError>
pub fn complete_todo(&mut self, id: U256) -> Result<(), TodoError>
pub fn update_todo(&mut self, id: U256, new_text: String) -> Result<(), TodoError>
pub fn delete_todo(&mut self, id: U256) -> Result<(), TodoError>

// Utility functions
pub fn get_total_todos(&self) -> U256
pub fn get_next_id(&self) -> U256
pub fn get_owner(&self) -> Address
pub fn todo_exists(&self, id: U256) -> bool
pub fn transfer_ownership(&mut self, new_owner: Address) -> Result<(), TodoError>
```

### Events

The contract emits the following events:

- `TodoCreated(uint256 indexed id, string text, address indexed creator)`
- `TodoCompleted(uint256 indexed id, address indexed completer)`
- `TodoUpdated(uint256 indexed id, string new_text, address indexed updater)`
- `TodoDeleted(uint256 indexed id, address indexed deleter)`

### Testing

Run the test suite:

```bash
cargo test
```

This runs 16 comprehensive tests covering all functionality and edge cases.

### Export ABI

Generate the Solidity ABI:

```bash
cargo stylus export-abi
```

### Deployment

1. **Check compilation:**

```bash
cargo stylus check
```

2. **Deploy to testnet:**

```bash
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH>
```

3. **Estimate gas costs:**

```bash
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH> --estimate-gas
```

### Testnet Information

All testnet information, including faucets and RPC endpoints: [Arbitrum Stylus Testnet Info](https://docs.arbitrum.io/stylus/reference/testnet-information)

## Usage Example

Once deployed, you can interact with the contract like any Ethereum smart contract:

## Error Handling

The contract includes comprehensive error handling:

- `TodoNotFoundError` - Todo with given ID doesn't exist
- `EmptyTextError` - Todo text cannot be empty
- `AlreadyCompletedError` - Todo is already marked as completed
- `UnauthorizedError` - Caller not authorized to perform action
- `NotOwnerError` - Only contract owner can perform action

## Access Control

- **Todo Creator**: Can modify their own todos (complete, update, delete)
- **Contract Owner**: Can modify any todo and transfer ownership
- **Others**: Can only view todos, cannot modify

## License

This project is licensed under MIT OR Apache-2.0.
