#Functions in Rust
Functions in Rust are defined with the fn keyword, followed by a name, parameters in parentheses, and an optional return type marked with ->.
Example:

```rust
fn add(a: i32, b: i32) -> i32 { 
    a + b 
}
```

This creates a function that adds two integers and returns their sum. If there's no return type, it implicitly returns an empty tuple (). You call functions like add(5, 3), and Rust's strict type system ensures you pass the right types.

#Comments   
Comments come in two flavors:

// for single-line notes
/* */ for multi-line or inline remarks
/// for documentation comments that generate API docs

These are great for explaining your code or temporarily disabling chunks.

#Control Flow
Control flows in Rust are straightforward:
Conditionals:

if, else if, and else handle conditionals
Example: if x > 0 { println!("Positive!") }

#Loops:
loop for infinite loops (broken with break)
while for condition-based loops
for for iterating over collections
Example: for i in 0..5 { println!("{}", i) }

#Heap and Stack

Stack
Stores values in a fixed manner (LIFO - Last In, First Out)
Think of it like a pile of stacked phones: when you add a phone on top, you can only remove the one at the top, not from the middle or bottom
Ownership ensures the stack is cleared up automatically using the RAII model

Heap
Less organized and structured
When you request space, the memory allocator finds an empty space that's good enough, marks it as used, and gives you a pointer that points to the address

#Rust Ownership Rules

Three Main Rules:
Every value in Rust has a variable that is called its owner
There can only be one owner at a time
When the owner goes out of scope (the part of code where the variable values end), the variable is dropped and the memory is automatically freed

Alternative Four-Rule Approach:
Each object can be used exactly once - When you use an object, it is moved to the new location and is no longer usable in the old
When an object passes out of scope, it is destroyed and is no longer usable
Blocks can produce a value which goes up one level of scope
All objects have a lifetime which constrains which scopes they may be moved out of

#Question

What happens when you try to use a variable after it has been moved to another variable? 

If a function takes ownership of a parameter, what happens to the original variable that was passed in? 

How does Rust's ownership system eliminate the need for both garbage collection and manual memory management? 