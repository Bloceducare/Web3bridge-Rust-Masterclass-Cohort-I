## Functions in Rust

Functions in Rust are reusable blocks of code defined with `fn`, taking parameters and optionally returning values. The return value can be:

- **Explicitly** returned using the `return` keyword
- **Implicitly** returned as the last expression in the function

---

## Control Flow in Rust

Rust provides several control flow constructs to direct how and when code is executed:

- `if` – for conditionals
- `loop` – for infinite repetition (until `break`)
- `while` – for looping based on a condition
- `for` – for iterating over ranges or collections

---

## Ownership

**Ownership** is Rust’s system for managing memory:

- Each value in Rust has a single **owner** – the variable that holds it
- When the owner goes out of scope, Rust automatically frees the memory
- Ownership can be **transferred** by assigning the value to another variable or passing it to a function

---

## Borrowing

**Borrowing** allows a function or variable to access data without taking ownership:

- Use `&` to create a **reference** – a borrowed pointer to the value
- Borrowing enables temporary access without invalidating the original owner

---

## References

A **reference** is a pointer to a value in memory:

- Immutable reference: `&T`
- Mutable reference: `&mut T`
