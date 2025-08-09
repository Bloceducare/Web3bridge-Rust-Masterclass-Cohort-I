# Supply Order Tracker (Rust CLI Project)

This is a simple **command-line application** written in Rust to manage supply orders for a business. It's built as part of a learning project to practice important Rust concepts such as:

- `Vec` and `HashMap`
- `Enum` and `Option`
- Matching and error handling
- Modular function-based design
- Reading from user input with `std::io`

---

## Features

This project progresses through **3 stages**:

### Stage 1: Add & View Orders
- Add a new order (item name, quantity, supplier)
- View all existing orders

### Stage 2: Remove Orders
- Remove fulfilled orders by ID

### Stage 3: Edit & Cancel Orders
- Edit an existing order (any field)
- Cancel edits if not confirmed

---

## Key Implementations

- **Ownership & Borrowing**: Passing data around safely
- **Enums**: Managing user menu choices
- **HashMap**: Storing orders with unique IDs for fast lookups
- **Pattern Matching**: Handling user input cleanly
- **Modularization**: Breaking logic into separate, readable functions

---

## How to Run

Make sure you have Rust and Cargo installed.  
To run the program:

```bash
cargo run
````

It will compile and launch the interactive menu in your terminal.

---

## Project Structure

```
src/
├── main.rs               # Entry point
├── order_actions.rs      # All order-related logic (add, view, remove, edit)
├── orders.rs             # Main Struct order for the tracker 
└── utils.rs              # Helper functions like reading input
```

---

## Example Usage

```bash

Choose an option:
1. Add Order
2. View Orders
3. Remove Order
4. Edit Order
5. Exit

> 1
Enter item name: Pens
Enter quantity: 100
Enter supplier: OfficePro
Order added with ID: 1
```

