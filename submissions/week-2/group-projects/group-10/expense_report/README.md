# 🧾 Expense Manager CLI

A simple, interactive command-line tool written in Rust for tracking and managing personal or team expenses.

## ✨ Features

- ✅ Add new expenses with name, category, and amount
- 📋 View all existing expenses
- 🛠️ Edit expense details (name, category, amount, status)
- ❌ Remove expenses (only if Approved or Rejected)
- 🔐 Status management: Pending, Approved, Rejected
- 🧪 Fully unit-tested core logic
- 📦 Clean separation between library and CLI app

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021+)

### Installation

```bash
git clone https://github.com/your-username/expense-manager-cli.git
cd expense-manager-cli
cargo build --release
```

## Usage
```bash
cargo run
```

## 📁 Project Structure

```
src/
├── main.rs        # CLI interface
├── lib.rs         # Library entrypoint
├── model.rs       # Data models (Expense, Status)
├── manager.rs     # ExpenseManager logic
└── errors.rs      # Custom error types
```

## 🧪 Running Tests

```bash
cargo test
```

### 📚 Example

```bash
Welcome to Expense Manager!
1. Add Expense
2. View Expenses
3. Edit Expense
4. Remove Expense
5. Exit

Enter your choice:
```

### Group 10
- Sogobanwo
- Ayo 
- Emarc 