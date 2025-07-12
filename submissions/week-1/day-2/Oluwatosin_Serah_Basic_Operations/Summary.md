## Summary of what I understand from Chapter 3 & 4

### Control Flow

- I understood the **if** expression
- Repeating actions with loops using : **loop**, **for**, and **while**
    - **loop** runs forever until you explicitly break out of it. Good for gaming programs
    - **for** is good for collection
    - **while** runs while condition is true


### How Stack and Heap Work

Stack and heap are both available to Rust programs at runtime, but they work very differently. Stack follows "Last In, First Out" order, so accessing elements is fast. When you add data to the stack, it goes right on top, and when you need something, you just pop from the top. Every data on the stack must have a fixed size that's known when you compile your program.

The heap, on the other hand, is good for data that might grow or shrink over time (like vectors) or when you don't know the size at compile time.

When you store data on the heap, the system searches through available memory to find a space large enough to fit your data. Once found, it returns a pointer (basically the memory address where your data is stored).

To access your data later, you follow this pointer to the actual location on the heap. This lookup process makes heap access slower than stack access since you have to "follow the pointer" to find your data.

### The Heap Management Problem

Since the heap isn't organized like the stack, programs struggle with fundamental questions: When should we free up this data? Who's responsible for cleaning it up? What if we accidentally make expensive copies of large data? These problems lead to memory leaks, expensive copies, and performance issues.

### Ownership Rules

This is where the Rust ownership model comes in. It is a set of rules that the compiler checks before the program runs. These ownership rules address the heap management problem by making it clear who owns what data, when the data gets cleaned up, and minimizing data copies on the heap so one doesn't run out of space.

#### Key Ownership Concepts:

- The reason why the Rust ownership rules were created was mainly because of the heap management issue
- Ownership rules solve the problem of how data is being managed
- The Rust ownership rules that the compiler follows ensure memory safety
- When a String data is copied, we copy the pointer, length, and capacity that are on the stack, not the heap data the pointer points to, because copying the heap data could be very expensive in terms of performance if the data on the heap were large
- Since Rust calls the drop function when a variable goes out of scope, both the copied variable and the variable that copied it will try to free the same memory, which could lead to vulnerabilities. To ensure memory safety, once the variable is copied, Rust considers the variable being copied as no longer valid, so there's no double free error happening anymore
- The `.clone()` method is used to copy deeply (expensive action) data not just the stack data but the heap data of the String
- A type that implements a Copy trait means variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable

### Copy Trait Rules:

- If a type implements drop, Rust won't allow the Copy trait on it
- Types that implement Copy traits include all scalar types and tuples if they contain types that also implement Copy
- If a type implements the Copy trait and its variable is being copied, it is still valid within the scope and not moved
- Returning values can also transfer ownership aside from ownership being moved during assignment
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable

### Reference and Borrowing

The concept of reference is being able to use a variable without taking ownership of it. A reference, like the name implies, refers to a location where we can access particular data we want to use without owning it, represented with the ampersand sign (&).

#### Reference Rules:

- The opposite of referencing by using & is dereferencing (*)
- When functions have references as parameters instead of the actual values, we won't need to return the values in order to give back ownership, because we never had ownership
- Creating a reference is called borrowing
- Mutable reference allows us to modify a borrowed value
- **Borrowing rules**: At any given time, you can have either:
  - One mutable reference, OR
  - Any number of immutable references
- You cannot have a mutable reference while you have an immutable one to the same value (this prevents data races)
- References must always be valid (no dangling references)

### The Slice Type

A slice is a view into a segment of a collection, a reference to a portion of the data (arrays, vectors, strings, etc.) without owing it. Its length is not known at compile time; it is determined at runtime. 

- Common examples include string slices (&str) and array slices (&[T])

###S String vs &str

- **String**: An owned, growable string type stored on the heap
- **&str**: A string slice, which is a reference to a portion of a string (either in a String or string literal)

### Question
Clarity on the use of the dangling pointer