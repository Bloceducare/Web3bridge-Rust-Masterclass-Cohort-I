# Summaries

## Chapter 3 Common Programming Concepts

### Learnings
* Data Types: 
Rust is a statically typed language and has scalar types(integers, floating-point numbers, Booleans, and characters). integers can be signed or unsigned. integers have sizes and if you use u8 and try to store value greater than 256, it overflows and it is wrapped, say you try to store 256, it becomes 0, 257 becomes 1 and so on.

* Compound Types:
This can group multiple values into one type. Rust has two primitive compound types: tuples and arrays. A tuple is a general way of grouping together a number of values with a variety of types into one compound type while array is a collection of values that are of same type. Tuples have a fixed length.

* Functions: The main is the entry point of rust code. differences between parameters and arguments. functions can have return value.

* Comments: this are extra explanation
* Control flow: There are a number of control flows from "if", "loop", "while", "for"


## Chapter 4 Understanding Ownership

### Learnings
* Ownership is a set of rules that govern how a Rust program manages memory. Each value has a single owner, which is the variable that holds it. There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.

* Variable Scope: A scope is the range within a program for which an item is valid.

* Move and Copy: Simple, fixed-size types like integers implement the Copy trait, so assignment duplicates the bits and both variables remain valid. Complex types like String don’t implement Copy. Assigning them moves ownership: only the destination remains valid. 
Rust avoids implicit deep copies to maintain performance; moves are inexpensive and safe.

* Memory Safety via Ownership: After a move, the compiler prohibits use of the original variable, preventing double-free and use-after-free issues.
* References and Borrowing: Clear ownership & borrowing—references borrow without ownership. Mutable exclusivity—one mutable or many immutable borrows at once.
Safety by default—no dangling pointers, no data races, enforced at compile time.
