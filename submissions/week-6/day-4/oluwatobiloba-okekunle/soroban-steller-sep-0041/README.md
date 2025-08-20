# SEP-41 Token & Employee Management System on Soroban

A comprehensive smart contract suite for the Stellar Soroban platform, featuring a fully compliant SEP-41 token implementation and an Employee Management System (EMS) for payroll automation.

## Overview

This project consists of two integrated smart contracts:

1. **SEP-41 Token Contract**: A fully compliant implementation of the Stellar Ecosystem Proposal 41 (SEP-41) token standard, which defines the interface for fungible tokens on Soroban. The implementation includes all standard token operations plus administrative features for token management.

2. **Employee Management System (EMS)**: A comprehensive payroll management contract that integrates with the SEP-41 token for automated salary payments. It handles employee registration, salary management, promotions, suspensions, and weekly payment automation.

## Features

### Core Token Functionality
- **Transfer**: Send tokens between addresses
- **Balance Tracking**: Query token balances for any address
- **Total Supply Management**: Track and manage the total token supply
- **Allowance System**: Approve third parties to spend tokens on your behalf
- **Transfer From**: Enable approved spenders to transfer tokens

### Administrative Features
- **Minting**: Create new tokens (admin only)
- **Burning**: Destroy tokens to reduce supply
- **Clawback**: Reclaim tokens from any address (admin only)
- **Admin Management**: Transfer administrative control

### Token Metadata
- Customizable token name
- Token symbol
- Decimal places for fractional amounts

### Employee Management System Features
- **Employee Registration**: Add employees with specific ranks and weekly salaries
- **Employee Removal**: Remove employees from the system
- **Salary Management**: Update employee salaries
- **Promotion System**: Promote employees to different ranks (Junior, Mid, Senior, Lead, Manager)
- **Suspension/Unsuspension**: Temporarily suspend or reinstate employees
- **Automated Payments**: Pay employees their weekly salaries using SEP-41 tokens
- **Payment Tracking**: Track last payment dates and check if payments are due
- **Employee Queries**: Get employee details and check existence

## Project Structure

```text
.
├── contracts
│   ├── sep41-token
│   │   ├── src
│   │   │   ├── lib.rs          # Main library module
│   │   │   ├── contract.rs     # Core contract implementation
│   │   │   ├── admin.rs        # Administrator functionality
│   │   │   ├── allowance.rs    # Allowance management
│   │   │   ├── balance.rs      # Balance operations
│   │   │   ├── storage.rs      # Storage utilities
│   │   │   ├── metadata.rs     # Token metadata handling
│   │   │   ├── events.rs       # Event emissions
│   │   │   ├── error.rs        # Error definitions
│   │   │   └── test.rs         # Comprehensive test suite
│   │   ├── test_snapshots      # Test verification snapshots
│   │   ├── Cargo.toml
│   │   └── Makefile
│   └── ems                     # Employee Management System
│       ├── src
│       │   ├── lib.rs          # Main library module
│       │   ├── contract.rs     # Core EMS implementation
│       │   ├── admin.rs        # Administrator functionality
│       │   ├── error.rs        # Error definitions
│       │   ├── events.rs       # Event emissions
│       │   ├── storage.rs      # Storage utilities
│       │   ├── import.rs       # SEP-41 token import
│       │   ├── types.rs        # Employee types and ranks
│       │   └── test.rs         # Test suite
│       └── Cargo.toml
├── Cargo.toml
├── .gitignore
└── README.md
```

## Add Another Contract

```bash
stellar contract init soroban-steller-sep-0041 --name ems
```
This generates the ems contract in the `contracts/ems` directory, it would skip workspace generation as it already exists(same name).

## Getting Started

### Prerequisites

- Rust with `wasm32-unknown-unknown` target
- Soroban CLI
- Stellar account with testnet XLM


### Testing

Run the comprehensive test suite:
```bash
cargo test
```

The test suite covers:
- Token initialization
- Minting and burning operations
- Transfer mechanics
- Allowance management
- Admin functions
- Edge cases and error handling
- Large amount operations
- Zero amount operations


## Contract Interfaces

### SEP-41 Token Contract

The token contract implements the full SEP-41 interface:

- `initialize(admin, decimal, name, symbol)` - Initialize the token
- `mint(to, amount)` - Mint new tokens (admin only)
- `transfer(from, to, amount)` - Transfer tokens
- `transfer_from(spender, from, to, amount)` - Transfer on behalf of another address
- `approve(from, spender, amount, expiration_ledger)` - Approve spending allowance
- `allowance(from, spender)` - Check spending allowance
- `balance(id)` - Get token balance
- `burn(from, amount)` - Burn tokens
- `burn_from(spender, from, amount)` - Burn tokens on behalf of another address
- `clawback(from, amount)` - Reclaim tokens (admin only)
- `set_admin(new_admin)` - Transfer admin rights
- `decimals()` - Get token decimals
- `name()` - Get token name
- `symbol()` - Get token symbol
- `total_supply()` - Get total token supply

### Employee Management System (EMS) Contract

The EMS contract provides comprehensive employee management functionality:

- `initialize(admin, token_address)` - Initialize the EMS with admin and SEP-41 token address
- `add_employee(employee_address, rank, weekly_salary)` - Register a new employee (admin only)
- `remove_employee(employee_address)` - Remove an employee from the system (admin only)
- `update_salary(employee_address, new_salary)` - Update employee salary (admin only)
- `promote_employee(employee_address, new_rank)` - Promote employee to new rank (admin only)
- `suspend_employee(employee_address)` - Suspend an employee (admin only)
- `unsuspend_employee(employee_address)` - Reinstate a suspended employee (admin only)
- `pay_employee(employee_address)` - Pay weekly salary to employee using tokens (admin only)
- `get_employee(employee_address)` - Get employee details
- `employee_exists(employee_address)` - Check if employee is registered
- `is_payment_due(employee_address)` - Check if employee payment is due
- `get_payment_info(employee_address)` - Get salary and payment status
- `get_admin()` - Get current admin address
- `set_admin(new_admin)` - Transfer admin rights

#### Employee Ranks
The system supports five employee ranks:
- Junior (1)
- Mid (2)
- Senior (3)
- Lead (4)
- Manager (5)

## Security Features

### Token Contract
- **Authorization Checks**: All sensitive operations require proper authentication
- **Admin-Only Operations**: Minting and clawback restricted to administrator
- **Overflow Protection**: Safe arithmetic operations prevent integer overflow
- **State Extension**: Automatic storage lifetime management
- **Error Handling**: Comprehensive error types for all failure cases

### EMS Contract
- **Admin Authorization**: All management operations require admin authentication
- **Employee Status Validation**: Checks for active/suspended status before operations
- **Payment Due Verification**: Prevents duplicate payments within the same week
- **Token Integration Security**: Validates token contract initialization before use
- **Duplicate Prevention**: Prevents registering the same employee twice
- **State Consistency**: Maintains consistent state across all operations

## Development

### Building from Source
```bash
make build
```

### Running Tests
```bash
make test
```

### Formatting Code
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Integration Example

To use the EMS with SEP-41 token:

1. Deploy and initialize the SEP-41 token contract
2. Deploy and initialize the EMS contract with the token address
3. Mint tokens to the admin address for payroll
4. Register employees with their ranks and salaries
5. Pay employees weekly using the automated payment system

```rust
// Example workflow
// 1. Initialize token
token.initialize(admin, 7, "Company Token", "COMP");

// 2. Initialize EMS with token address
ems.initialize(admin, token_address);

// 3. Mint tokens for payroll
token.mint(admin, 1000000);

// 4. Add employee
ems.add_employee(employee_addr, EmployeeRank::Senior, 5000);

// 5. Pay employee (weekly)
ems.pay_employee(employee_addr);
```