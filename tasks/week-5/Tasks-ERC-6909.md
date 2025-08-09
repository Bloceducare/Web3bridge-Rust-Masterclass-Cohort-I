

# 🧪 Assignment: Implement ERC-6909 Multi-Token Standard with Stylus

## 🎯 Objective

Create a Stylus smart contract implementing the **ERC-6909** multi-token standard. This contract should:

* Support **per-token fungible and non-fungible tokens**
* Maintain **total supply per token ID**
* Allow **balance queries** for each token ID and owner
* Support **per-token allowances** for spenders (like ERC-20 approvals)
* Support **operator approvals** that allow operators to manage all tokens of an owner
* Allow **safe transfers** of tokens respecting allowances and operator approvals
* Emit **TransferSingle** and **ApprovalSingle** events according to the standard

---

## 📂 Deliverables

* A Stylus Rust contract implementing the ERC-6909 interface
* Proper storage for token balances, allowances, total supply, and operator approvals
* Public methods including:

  * `total_supply(token_id: U256) -> U256`
  * `balance_of(owner: Address, token_id: U256) -> U256`
  * `transfer_from(from: Address, to: Address, token_id: U256, amount: U256) -> bool`
  * `approve(spender: Address, token_id: U256, amount: U256) -> bool`
  * `set_operator(operator: Address, approved: bool) -> bool`
  * `operator_approval(owner: Address, operator: Address) -> bool`
* Event definitions and emitting events per transfer and approval actions
* Error handling for insufficient balances and allowances

---

## 🛠️ Steps to Complete

1. Fork or create a new Stylus Rust project
2. Define the storage structs and mappings for balances, allowances, total supply, and operators
3. Implement the methods as specified
4. Emit appropriate events per action
5. Write tests to verify all major behaviors (transfers, approvals, operator usage, error cases)
6. Compile the contract to WASM using Stylus CLI
7. (Optional) Deploy and test on a local Stylus dev node or testnet

---

## 📎 Submission Format

```markdown
### Task: erc6909-stylus

**GitHub Repo:** https://github.com/YOUR_USERNAME/erc6909-stylus

**Summary:**  
Implemented ERC-6909 multi-token standard in Rust using Stylus SDK.  
Supports per-token balances, allowances, operator approvals, and safe transfers.  
Includes event emission and error handling.

**Testing:**  
- Unit tests for transfer, approve, and operator flows  
- Compiled and deployed locally using Stylus CLI  
- Verified on-chain behavior with sample calls

**Notes:**  
- Optional metadata extension not implemented  
- Can extend with URI support or batch operations if needed
```
