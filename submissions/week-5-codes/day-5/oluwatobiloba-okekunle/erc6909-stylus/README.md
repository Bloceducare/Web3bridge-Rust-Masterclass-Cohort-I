
# ERC6909 Multi-Token Implementation for Arbitrum Stylus

A Rust implementation of the ERC6909 multi-token standard for Arbitrum Stylus, providing efficient management of multiple fungible tokens within a single contract. Built using the [stylus-sdk](https://github.com/OffchainLabs/stylus-sdk-rs).

## Overview

ERC6909 is a multi-token standard that allows a single contract to manage multiple fungible tokens identified by unique IDs. This implementation includes:

- **Multiple token management** - Create and manage unlimited token types with unique IDs
- **Owner-controlled minting** - Only the contract owner can mint new tokens
- **Flexible approvals** - Per-token approvals and operator permissions
- **Comprehensive transfers** - Direct transfers and delegated transfers via allowances
- **Token burning** - Users can burn their own tokens

## Features

### Core Functionality

- ✅ **Constructor initialization** - Set token name, symbol, and owner on deployment
- ✅ **Multi-token support** - Each token identified by a unique `uint256` ID
- ✅ **Balance tracking** - Independent balances for each token ID per address
- ✅ **Allowance system** - Granular approval per spender, per token ID
- ✅ **Operator approvals** - Global operator permissions across all token IDs
- ✅ **Minting** - Owner-restricted minting to any address
- ✅ **Burning** - Users can burn their own tokens
- ✅ **Events** - Transfer, Approval, and OperatorSet events for tracking

### Security Features

- Zero address validation on all transfers and approvals
- Insufficient balance checks with detailed error messages
- Allowance spending validation
- Owner-only minting restrictions
- Comprehensive error handling with descriptive error types

## Quick Start 

Install [Rust](https://www.rust-lang.org/tools/install), and then install the Stylus CLI tool with Cargo

```bash
cargo install --force cargo-stylus cargo-stylus-check
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Then, clone the repository:

```
git clone <YOUR_REPO_URL> && cd erc6909-stylus
```

### Testnet Information

All testnet information, including faucets and RPC endpoints can be found [here](https://docs.arbitrum.io/stylus/reference/testnet-information).

## Contract Interface

### Public Functions

```rust
// Constructor - called on deployment
constructor(name: String, symbol: String)

// View functions
name() -> String
symbol() -> String  
decimals() -> u8  // Returns 18
balance_of(owner: Address, id: U256) -> U256
allowance(owner: Address, spender: Address, id: U256) -> U256
is_operator(owner: Address, spender: Address) -> bool

// State-changing functions
mint(to: Address, id: U256, value: U256) -> Result  // Owner only
transfer(receiver: Address, id: U256, value: U256) -> Result
transfer_from(sender: Address, receiver: Address, id: U256, value: U256) -> Result
approve(spender: Address, id: U256, value: U256) -> Result
set_operator(spender: Address, approved: bool) -> Result
burn(id: U256, value: U256) -> Result
```

### Events

```solidity
event Transfer(address indexed from, address indexed to, uint256 indexed id, uint256 value)
event Approval(address indexed owner, address indexed spender, uint256 indexed id, uint256 value)
event OperatorSet(address indexed owner, address indexed sender, bool approved)
```

### Error Types

```solidity
error ERC6909InsufficientBalance(address sender, uint256 balance, uint256 needed, uint256 id)
error ERC6909InvalidSender(address sender)
error ERC6909InvalidReceiver(address receiver)  
error ERC6909InvalidApprover(address approver)
error ERC6909InvalidSpender(address spender)
error ERC6909InsufficientAllowance(address owner, uint256 allowance, uint256 needed, uint256 id)
```

## ABI Export

Export the Solidity ABI for your program:

```bash
cargo stylus export-abi
```

This generates the Solidity interface that can be used to interact with the deployed contract.

## Testing

Run the comprehensive test suite:

```bash
cargo test
```

The test suite includes:
- Balance and allowance checks
- Transfer operations (direct and delegated)
- Approval mechanisms (token-specific and operator)
- Minting and burning operations
- Edge cases and error conditions
- Multi-token ID support

## Deploying

Check your program compiles to valid WASM for Stylus:

```bash
cargo stylus check
```

Estimate gas costs before deployment:

```bash
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH> \
  --estimate-gas
```

Deploy to Stylus:

```bash
cargo stylus deploy \
  --private-key-path=<PRIVKEY_FILE_PATH>
```

## Usage Example

After deployment, you can interact with the contract using any Ethereum tooling:

```javascript
// JavaScript example using ethers.js
const contract = new ethers.Contract(contractAddress, abi, signer);

// Mint tokens (owner only)
await contract.mint(userAddress, tokenId, amount);

// Transfer tokens
await contract.transfer(recipientAddress, tokenId, amount);

// Approve spending
await contract.approve(spenderAddress, tokenId, allowance);

// Set operator (allows all token transfers)
await contract.setOperator(operatorAddress, true);

// Burn tokens
await contract.burn(tokenId, amount);
```

## Architecture

The implementation follows a modular architecture:

1. **Storage Layer** (`sol_storage!`)
   - Owner address
   - Token metadata (name, symbol, decimals)
   - Balance mappings per token ID
   - Allowance mappings per token ID
   - Operator approval mappings

2. **Internal Functions** (prefixed with `_`)
   - `_approve`: Core approval logic
   - `_set_operator`: Operator management
   - `_spend_allowance`: Allowance consumption
   - `_update`: Core balance update logic
   - `_mint`: Internal minting
   - `_burn`: Internal burning
   - `_transfer`: Internal transfer logic

3. **Public Interface** (`#[public]`)
   - External functions that implement access control
   - Event emission
   - Error handling

## Build Options

Optimize your WASM binary size:

```toml
[profile.release]
opt-level = "z"
strip = true
lto = true
codegen-units = 1
panic = "abort"
```

See [cargo-stylus optimization guide](https://github.com/OffchainLabs/cargo-stylus/blob/main/OPTIMIZING_BINARIES.md) for more options.

## Security Considerations

1. **Access Control**: Only the contract owner can mint new tokens
2. **Zero Address Checks**: Prevents burning tokens by sending to address(0)
3. **Balance Validation**: All transfers check sufficient balance before execution
4. **Allowance Management**: Proper allowance spending and validation
5. **Operator System**: Careful consideration needed when setting operators as they gain full transfer rights

## Future Improvements

- [ ] Add batch operations for gas efficiency
- [ ] Implement permit functionality for gasless approvals  
- [ ] Add metadata URI support per token ID
- [ ] Consider upgradeability patterns
- [ ] Add total supply tracking per token ID

## License

This project is fully open source, including an Apache-2.0 or MIT license at your choosing under your own copyright.