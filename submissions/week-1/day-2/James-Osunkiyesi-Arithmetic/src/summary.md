## 📘 Chapter 3: Core Programming Concepts

This chapter covers Rust’s basic programming elements, highlighting its focus on safety and efficiency.

### 🔹 Variables and Mutability
- By default, variables in Rust cannot be changed.
- To allow changes, add the `mut` keyword.
- This approach helps prevent unexpected bugs.

### 🔹 Data Types
- Rust uses static typing, so types are set at compile time.
- **Scalar types** include integers, floating-point numbers, booleans, and characters.
- **Compound types**:
    - **Tuples**: Combine values of different types.
    - **Arrays**: Store a fixed number of values of the same type.

### 🔹 Functions
- Functions start with `fn`.
- All parameters need explicit type declarations.
- The last line in a function, if it lacks a semicolon, is returned automatically.

### 🔹 Control Flow
- Use `if`, `else if`, and `else` for branching.
- Loops:
    - `loop` for endless repetition
    - `while` for condition-based loops
    - `for` for iterating over collections
- The `match` statement enables pattern matching.

### 🔹 Comments
- Single-line comments use `//`.

---

## 📘 Chapter 4: Ownership Explained

Ownership is a key Rust feature that ensures memory safety without a garbage collector.

### 🔹 Ownership Principles
1. Every value has one owner.
2. When the owner leaves scope, the value is freed.
3. Values can be moved or borrowed.

### 🔹 Borrowing and References
- `&` creates an immutable reference.
- `&mut` creates a mutable reference.
- You can have either one mutable reference or multiple immutable ones at a time.

### 🔹 Slices
- Slices let you reference a part of a collection, like an array or string, without taking ownership.

### 🔹 The Borrow Checker
- Rust’s borrow checker enforces these rules at compile time, preventing issues like dangling references and data races.

---

## In Summary

- Chapter 3 introduces Rust’s syntax, types, functions, and control flow.
- Chapter 4 explains how Rust manages memory through ownership and borrowing.
- Together, these chapters lay the groundwork for writing reliable and efficient Rust programs.

---