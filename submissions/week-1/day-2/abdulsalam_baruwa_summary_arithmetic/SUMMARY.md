# Chapters 3 and 4 of The Rust Book

### ✅ What I Understand So Far
- The `fn` keyword is used to declare new functions.
- The `main` function is the entry point of Rust programs.
- Functions can be defined before or after the `main()`, the order doesn't matter as long as they're in scope.
- Functions can take parameters and return values and you must explicitly annotate types in the signature.
- Rust offers multiple ways to control flow: `if`, `loop`, `while`, and `for`.
- In if expressions, Rust requires the condition to be strictly a bool type.
- You can assign the result of an if expression to a variable.
- All branches must return the same type.
- Loop runs indefinitely unless explicitly stopped.
- You can return a value from a loop using break, with an expression.
- Ownership ensures memory safety without needing a garbage collector.
- Each value has an owner, and there can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
- Borrowing lets me read or mutate data without moving it.

### ❓ What I Still Don’t Understand
- Can two mutable borrows happen if they’re in different scopes?
- How exactly does slicing avoid copying the entire array?
- Why do some types automatically implement the `Copy` trait and others don’t?