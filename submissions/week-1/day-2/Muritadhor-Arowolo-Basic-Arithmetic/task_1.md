## Summary of what I learnt in Chapters 3 and 4 of the rust book

### Chapter 3: Common Programming Concepts
- **Variables and Mutability**: Variables are immutable by default, but can be made mutable with the `mut` keyword.
- **Data Types**: Rust has scalar types (integers, floating-point numbers, booleans, characters) and compound types (tuples, arrays).
- **Functions**: Functions are defined using the `fn` keyword, and can take parameters and return values.
- **Control Flow**: Rust supports `if` expressions, loops (`loop`, `while`, `for`), and pattern matching with `match`.
- **Comments**: Single-line comments start with `//`.

### Chapter 4: Understanding Ownership
- **Ownership**: Each value in Rust has a single owner, and when the owner goes out of scope, the value is dropped.
- **References**: References allow you to borrow a value without taking ownership. They are immutable by default, but can be made mutable with `&mut`.
- **Slices**: Slices are a reference to a contiguous sequence of elements in a collection, allowing you to work with parts of collections without taking ownership.
- **The Borrow Checker**: The borrow checker enforces rules that ensure references do not outlive the data they point to, preventing dangling references