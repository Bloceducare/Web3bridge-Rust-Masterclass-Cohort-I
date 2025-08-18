### Task: stylus-erc20
**GitHub:** https://github.com/gboigwe/stylus-erc20
**Summary:**  
Implemented a complete ERC-20 token standard in Rust for Arbitrium Stylus. The contract follows the OpenZeppelin ERC-20 specification with all required functions and events. Updated implementation to work with the latest Stylus SDK v0.9.0, addressing API changes including deprecated msg::sender() and evm::log() functions.
**Features:**  
- Core ERC-20 functions: `name`, `symbol`, `decimals`, `totalSupply`, `balanceOf`, `transfer`, `approve`, `allowance`, `transferFrom`
- Events: `Transfer(from, to, value)`, `Approval(owner, spender, value)`
- Optional `mint` function for token creation
- Comprehensive unit tests covering transfers, allowances, and insufficient balance scenarios
- Compatible with Stylus SDK 0.9.0 using `sol_storage!` macro and `self.vm()` methods
**Commands:**
```bash
cargo stylus new erc20_token
cargo stylus build
cargo stylus check
cargo test

### Task: erc6909-stylus
**GitHub:** https://github.com/gboigwe/stylus-erc6909
**Summary:**  
Successfully implemented ERC-6909 multi-token standard in Rust for Arbitrium Stylus. The contract manages multiple token types within a single deployment, supporting per-token balances, allowances, and operator approvals. Built with Stylus SDK 0.9.0, addressing API changes including proper event emission using `evm::log()` and updated storage macros with `sol_storage!`.
**Features:** Core ERC-6909 functions (total_supply, balance_of, transfer_from, approve, allowance, operator_approval, set_operator), Multi-token support with separate balances per token ID, Event emission (TransferSingle, ApprovalSingle), Basic mint function for token creation, Stylus SDK 0.9.0 compatibility
