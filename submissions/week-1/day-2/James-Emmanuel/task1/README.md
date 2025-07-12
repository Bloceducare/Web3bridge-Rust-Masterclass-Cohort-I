# Task 1: Study Chapters of The Rust Book

Chapter 3 covers on data types, variables declaration and managing mutability then functions. By design in rust, variables are immutable (cannot be modified) except `mut` flag is included when variable is declared.

Basically there are two types categories: **Scalar** and **Compound** types.

1. Scalar:
   - integer
   - float
   - boolean
   - charater
2. Compound:
   - Tuples (objects)
   - Arrays

Also learnt about `shadowing` where one can declare a new variable with the same name as a previous variable. This is generally not the case in other programming languages where one is not allowed to redeclare a variable with same name except within a differnt scope.

`Ownership` is a core concept in Rust that governs how memory is managed. It's one of the key features that allows Rust to provide memory safety without needing a garbage collector.

Ownership rules
1. Each value has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

However ownership doesn't move for data types that implement the `copy` trait. Here are some of the types that implement Copy:

- integer
- float
- boolean
- charater
- tuples (if they only contain types that also implement Copy) e.g `(i32, i32)` implements Copy, but `(i32, String)` does not.

On the other hand, `borrowing` enables to temporarily use a value without taking ownership. achieved by creating a reference (either mutable or immutable) to the needed variable with `&`.
