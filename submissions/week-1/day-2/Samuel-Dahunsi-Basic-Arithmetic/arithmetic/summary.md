# Summary of Chapters 3 & 4 – *Rust Book*

---

## 📘 Chapter 3: Common Programming Concepts

Chapter 3 introduces Rust’s foundational programming principles, focusing on core syntax and behavior shared across many languages, but with Rust’s unique emphasis on safety and performance.

### 🔹 Variables and Mutability
- Variables are **immutable by default**.
- Use the `mut` keyword to make variables mutable.
- Encourages writing predictable and bug-free code.

### 🔹 Data Types
- Rust is statically typed; types must be known at compile time.
- **Scalar types**: integers (`i32`, `u32`, etc.), floats (`f32`, `f64`), booleans, and characters.
- **Compound types**:
    - **Tuples**: Group multiple values of different types.
    - **Arrays**: Fixed-size collections of values of the same type.

### 🔹 Functions
- Defined using the `fn` keyword.
- Parameters require type annotations.
- The final expression in a function is returned implicitly (no semicolon).

### 🔹 Control Flow
- `if`, `else if`, and `else` for conditional logic.
- Looping:
    - `loop` (infinite loop)
    - `while` (condition-based)
    - `for` (iterator-based)
- Powerful `match` construct for pattern matching.

### 🔹 Comments
- Single-line comments begin with `//`.

---

## 📘 Chapter 4: Understanding Ownership

Ownership is a core concept that sets Rust apart, providing memory safety without needing a garbage collector.

### 🔹 Ownership Rules
1. Each value has a single **owner**.
2. When the owner goes out of scope, the value is **dropped** (freed).
3. Values can be **moved** or **borrowed**.

### 🔹 References and Borrowing
- Use `&` to **borrow** a value (immutable by default).
- Use `&mut` for **mutable references**.
- Only **one mutable reference** or **multiple immutable references** at a time (no mixing).

### 🔹 Slices
- A **slice** is a reference to part of a collection (like an array or string).
- Lets you work with sections of data **without taking ownership**.

### 🔹 The Borrow Checker
- Rust’s compile-time tool that enforces ownership and borrowing rules.
- Prevents **dangling references** and **data races**.

---

## Summary

- Chapter 3 teaches **syntax and structure**: variables, types, functions, and control flow.
- Chapter 4 introduces **ownership and borrowing**, Rust’s unique memory management system.
- Together, they provide the building blocks for writing safe and efficient code in Rust.

---
