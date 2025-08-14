# Day 2 - Rust Book Chapters 3 & 4 Summary and Questions

## Summary of Understanding:

- The Rust compiler is very helpful for understanding errors early, which makes development easier.
- Rust's compiler guarantees that values declared as immutable will not change, simplifying reasoning about the code.
- Shadowing differs from `mut` in that attempting to reassign a shadowed variable without `let` results in a compile-time error. Using `let` with shadowing allows for initial transformations before the variable becomes immutable.
- Data types in Rust are categorized into scalar (integers, floating-point numbers, Booleans, characters) and compound.
- Signed integers range from -(2^n-1) to 2^n-1 - 1, while unsigned integers range from 0 to 2^n - 1, where 'n' is the number of bits.
- Arrays are collections where all elements must have the same type, useful when the number of elements is fixed.
- Rust code conventionally uses snake_case for function and variable names.
- Function definitions can be placed anywhere in a visible scope.
- Function parameters require type annotations, which helps the compiler provide better error messages.
- Function definitions are statements and do not return values.
- Ownership is a set of rules governing memory management in Rust.

## Points of Confusion:

- The exact difference between `mut` and shadowing.
- A clear understanding of integer overflow.
- A deeper understanding of control flow.
