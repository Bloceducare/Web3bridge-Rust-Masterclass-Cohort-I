```python

Rust uses an ownership model to manage memory safely without needing a garbage collector.
Each value has one owner — when that owner goes out of scope, the value is automatically dropped.
Ownership can be transferred when assigning variables or passing to functions.
Borrowing lets you use data without taking ownership, using references (&T or &mut T).
Rust enforces strict rules to prevent data races and dangling references, all checked at compile time.

## The Three Rules of Ownership
Each value in Rust has an owner – There’s always exactly one variable that owns a value.
There can only be one owner at a time – When you assign a value to another variable or pass it to a function, ownership is transferred.
When the owner goes out of scope, the value is dropped – Rust automatically calls the drop function to clean up the memory.

## The Two Types of Borrowing
Immutable borrows (&T) – Allow reading but not modifying the borrowed value.
Mutable borrows (&mut T) – Allow modifying the borrowed value.

When ownership is moved, the original variable can no longer be used. This prevents multiple variables from trying to free the same memory.​

using the .clone() method creates a deep copy of the data, allowing both variables to own their respective copies.​

Borrowing allows functions to access data without taking ownership


stack memory is cheaper faster and easy to access thats why scalar values like integers, booleans string literals because their size is fixed and known at compile time e.t.c are stored on it

heap memory is father more expensive and bigger memory  that string types are able to access simply because the size of the string type is not known and can change 

```