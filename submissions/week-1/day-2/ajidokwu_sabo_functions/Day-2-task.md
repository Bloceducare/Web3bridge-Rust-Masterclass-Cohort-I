# 📚 Rust Study Summary

This document captures my understanding and questions after studying:
- 📖 **Remaining of Chapter 2** (Guessing Game) and
- 📖 **All of Chapter 3** (Common Programming Concepts)  
from [The Rust Programming Language](https://doc.rust-lang.org/book/).

---

## ✅ What I Understand

### 📝 Chapter 2 - Guessing Game
- Learned how to use `io` to read user input from the terminal.
- Understood how `rand` crate is used to generate random numbers.
- `match` is used to handle the `Result` returned by `parse` and control what happens on success vs error.
- Using `loop` to keep asking for input until the correct guess.
- `Ordering` is used with `.cmp()` to compare values and match on `Less`, `Greater`, or `Equal`.

---

### 📝 Chapter 3 - Common Programming Concepts
- `let` creates immutable variables by default. Use `let mut` for mutable ones.
- Rust infers types but can use explicit annotations like `let x: i32 = 5;`.
- Basic data types: integers, floats, booleans, characters, tuples, and arrays.
- Control flow:
  - `if`, `else if`, `else` for branching.
  - `loop` for infinite loops, `break` to exit and optionally return a value.
  - `while` loops run as long as a condition is true.
  - `for` loops are used to iterate over ranges or collections.
- Functions declared with `fn`, parameters require explicit types.
- Functions return the last expression automatically without `return`, unless an early return is needed.

---

## ❌ What I Do **Not Yet** Fully Understand
- Why does `parse` sometimes need syntax like `guess.trim().parse::<u32>()`?
- The nuances of when to explicitly write `return` vs just relying on the last expression.

