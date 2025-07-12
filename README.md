# Week 1 Day 4 - Structs and Implementation Tasks

**Author:** Godswill  
**Date:** July 10, 2025  
**Course:** Web3bridge Rust Masterclass Cohort I

## 📋 Overview

This project demonstrates the implementation of two main Rust concepts:
1. **Structs and Enums** - Library Item Management System
2. **Implementation Blocks (impl)** - Shipping Box System

## 🎯 Task Requirements

### Task 1: Library Item Management System
- ✅ Create a `LibraryItem` struct with `quantity`, `id`, and `item_type` fields
- ✅ Implement an `ItemType` enum with multiple variants (Book, Magazine, Fiction, NonFiction, Reference)
- ✅ Create three functions: `display_quantity()`, `display_id()`, and `display_item_type()`
- ✅ Demonstrate usage with multiple library items

### Task 2: Shipping Box System with `impl`
- ✅ Create a `ShippingBox` struct with dimensions, weight, and color
- ✅ Implement a `BoxColor` enum for color options
- ✅ Use `impl` block to create a `new()` constructor function
- ✅ Use `impl` block to create a `print_characteristics()` method
- ✅ Bonus: Added `calculate_volume()` method for additional functionality

## 🚀 How to Run

### Option 1: Using Rustc (Recommended to avoid workspace conflicts)
```bash
rustc src/main.rs && ./main
```

### Option 2: Using Cargo (if no workspace conflicts)
```bash
cargo run
```

## 📊 Sample Output

```
🎯 === WEEK 1 DAY 4 TASKS - GODSWILL ===

📚 TASK 1: LIBRARY ITEM MANAGEMENT SYSTEM
==========================================

📖 Processing Library Item #1:
📚 Item Quantity: 15
🔢 Item ID: 1001
📖 Item Type: Book

📰 Processing Library Item #2:
📚 Item Quantity: 25
🔢 Item ID: 2001
📰 Item Type: Magazine

🎭 Processing Library Item #3:
📚 Item Quantity: 8
🔢 Item ID: 3001
🎭 Item Type: Fiction


📦 TASK 2: SHIPPING BOX SYSTEM
===============================

📦 Small Package:

📦 === SHIPPING BOX CHARACTERISTICS ===
📏 Dimensions: 20.00 x 15.00 x 10.00 cm
⚖️  Weight: 0.50 kg
🎨 Color: Brown
📊 Volume: 3000.00 cubic cm
=======================================

📦 Medium Package:

📦 === SHIPPING BOX CHARACTERISTICS ===
📏 Dimensions: 40.00 x 30.00 x 20.00 cm
⚖️  Weight: 1.20 kg
🎨 Color: Blue
📊 Volume: 24000.00 cubic cm
=======================================

📦 Large Package:

📦 === SHIPPING BOX CHARACTERISTICS ===
📏 Dimensions: 60.00 x 45.00 x 35.00 cm
⚖️  Weight: 2.80 kg
🎨 Color: Green
📊 Volume: 94500.00 cubic cm
=======================================

🔍 Additional Box Analysis:
Small box volume: 3000.00 cm³
Medium box volume: 24000.00 cm³
Large box volume: 94500.00 cm³
📊 Total volume of all boxes: 121500.00 cm³

✅ All tasks completed successfully! 🎉
```

## 🏗️ Code Structure

### Enums
- `ItemType`: Represents different types of library items
- `BoxColor`: Represents available shipping box colors

### Structs
- `LibraryItem`: Contains quantity, id, and item_type
- `ShippingBox`: Contains dimensions, weight, and color

### Functions
- `display_quantity()`: Prints library item quantity
- `display_id()`: Prints library item ID
- `display_item_type()`: Prints library item type with emoji

### Implementation Methods
- `ShippingBox::new()`: Constructor for creating new shipping boxes
- `ShippingBox::print_characteristics()`: Displays all box properties
- `ShippingBox::calculate_volume()`: Calculates and returns box volume

## 🛠️ Rust Best Practices Implemented

1. **Comprehensive Documentation**: All functions and structs have proper doc comments
2. **Derive Traits**: Using `#[derive(Debug, Clone, PartialEq)]` for better functionality
3. **Proper Naming**: Following Rust naming conventions (snake_case, PascalCase)
4. **Memory Safety**: Using references (`&`) to avoid unnecessary ownership transfers
5. **Pattern Matching**: Using `match` expressions for enum handling
6. **Modular Design**: Clear separation between different functionalities
7. **Type Safety**: Leveraging Rust's strong type system with enums and structs

## 📁 Project Structure

```
Godswill-Day4-Tasks/
├── Cargo.toml          # Project configuration
├── README.md           # This file
└── src/
    └── main.rs         # Main implementation
```

## 🔧 Technical Details

- **Rust Edition**: 2021
- **Dependencies**: None (uses only standard library)
- **Compilation Target**: Native binary
- **Language Features**: Structs, Enums, Implementation blocks, Pattern matching

## ✨ Additional Features

Beyond the basic requirements, this implementation includes:
- 📱 Emoji-enhanced output for better visual presentation
- 📊 Volume calculation for shipping boxes
- 🔍 Additional analysis functionality
- 📚 Comprehensive documentation
- 🎨 Clean, readable code structure

---

**Note**: This project demonstrates fundamental Rust concepts including structs, enums, implementation blocks, and Rust's ownership system through practical examples.
