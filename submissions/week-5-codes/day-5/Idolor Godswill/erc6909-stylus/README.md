# ERC-6909 Multi-Token Standard Implementation in Stylus

A comprehensive implementation of the ERC-6909 multi-token standard using Arbitrum Stylus and Rust.

## 🎯 Features

- **Multi-Token Support**: Handle both fungible and non-fungible tokens in a single contract
- **Per-Token Balances**: Track balances for each token ID and owner
- **Allowance System**: ERC-20 style approvals per token ID
- **Operator Approvals**: Allow operators to manage all tokens for an owner
- **Safe Transfers**: Comprehensive validation and error handling
- **Event Emission**: Full ERC-6909 compliant events
- **Batch Operations**: Transfer multiple token types in a single transaction

## 📋 Interface

### Core Functions

- `total_supply(id: U256) -> U256` - Get total supply of a token ID
- `balance_of(owner: Address, id: U256) -> U256` - Get balance for owner and token ID
- `allowance(owner: Address, spender: Address, id: U256) -> U256` - Get allowance amount
- `is_operator(owner: Address, operator: Address) -> bool` - Check operator status
- `transfer_from(from: Address, to: Address, id: U256, amount: U256) -> bool` - Transfer tokens
- `approve(spender: Address, id: U256, amount: U256) -> bool` - Approve token spending
- `set_operator(operator: Address, approved: bool) -> bool` - Set operator approval

### Additional Functions

- `mint(to: Address, id: U256, amount: U256) -> bool` - Mint new tokens
- `burn(from: Address, id: U256, amount: U256) -> bool` - Burn tokens
- `batch_transfer_from(from: Address, to: Address, ids: Vec<U256>, amounts: Vec<U256>) -> bool` - Batch transfer

## 🔧 Events

- `TransferSingle(operator, from, to, id, amount)` - Emitted on token transfers
- `ApprovalSingle(owner, spender, id, amount)` - Emitted on approvals
- `OperatorSet(owner, operator, approved)` - Emitted on operator changes

## 🚨 Error Types

- `InsufficientBalance` - Not enough tokens to transfer
- `InsufficientAllowance` - Allowance too low for transfer
- `InvalidOperator` - Invalid operator operation
- `TransferToZeroAddress` - Cannot transfer to zero address
- `TransferFromZeroAddress` - Cannot transfer from zero address

## 🛠️ Building and Testing

### Prerequisites

- Rust (latest stable)
- Stylus CLI tools
- Node.js (for testing)

### Setup

```bash
# Clone the repository
git clone https://github.com/7maylord/erc6909-stylus
cd erc6909-stylus

# Install Stylus CLI
cargo install --force cargo-stylus

# Build the contract
cargo stylus build
```

### Running Tests

```bash
# Run unit tests
cargo test

# Generate ABI
cargo stylus export-abi
```

### Deployment

```bash
# Deploy to local testnet
cargo stylus deploy --endpoint http://localhost:8547

# Deploy to Arbitrum Sepolia testnet
cargo stylus deploy --endpoint https://sepolia-rollup.arbitrum.io/rpc
```

## 📝 Usage Examples

### Minting Tokens

```rust
// Mint 1000 fungible tokens with ID 1
contract.mint(owner_address, U256::from(1), U256::from(1000))?;

// Mint 1 NFT with ID 12345
contract.mint(owner_address, U256::from(12345), U256::from(1))?;
```

### Token Transfers

```rust
// Transfer 100 tokens of ID 1 from Alice to Bob
contract.transfer_from(alice, bob, U256::from(1), U256::from(100))?;
```

### Setting Allowances

```rust
// Allow spender to use 50 tokens of ID 1
contract.approve(spender, U256::from(1), U256::from(50))?;
```

### Operator Approvals

```rust
// Set operator to manage all tokens
contract.set_operator(operator, true)?;
```

## 🔐 Security Considerations

- **Access Control**: The mint/burn functions in this example don't have access control - add proper permissions for production
- **Integer Overflow**: Uses Rust's built-in overflow protection and safe arithmetic
- **Zero Address Checks**: Prevents transfers to/from zero address
- **Allowance Validation**: Proper allowance checking and updates

## 🚀 Production Enhancements

For production use, consider adding:

- Role-based access control for minting/burning
- Metadata URI support (ERC-6909 extension)
- Pausable functionality
- Gas optimizations
- More comprehensive batch operations
- Royalty support for NFTs

## 📜 License

MIT OR Apache-2.0

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📚 Resources

- [ERC-6909 Specification](https://eips.ethereum.org/EIPS/eip-6909)
- [Stylus Documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Arbitrum Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs)