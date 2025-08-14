# Rust Book Chapters 3 & 4 Summary

## Understanding Common Programming Concepts and Ownership

---

## Chapter 3: Common Programming Concepts (Continued)

### Functions

Functions in Rust are defined using the `fn` keyword and typically use snake_case naming conventions.

#### Key Characteristics:

- **Type Annotations**: Parameters require explicit type annotations in the function signature
- **Statements vs Expressions**: Rust distinguishes between:
  - **Statements**: Instructions that perform an action but don't return a value (e.g., `let y = 6;`)
  - **Expressions**: Units of code that evaluate to a value (e.g., `5 + 6`)
- **Return Values**: Functions implicitly return the value of their final expression
- **Early Exit**: `return` keyword can be used for early exit

#### Example:

```rust
fn calculate_sum(x: i32, y: i32) -> i32 {
    x + y  // Expression - no semicolon = return value
}

fn early_return(x: i32) -> i32 {
    if x < 0 {
        return 0;  // Early exit
    }
    x * 2  // Implicit return
}
```

### Comments

Rust supports multiple comment styles:

- **Single-line comments**: Start with `//`
- **Multi-line comments**: Use `/*... */` or multiple `//` lines
- **Documentation comments**: Use `///` or `//!` for generating API documentation

```rust
// This is a single-line comment

/*
This is a
multi-line comment
*/

/// This is a documentation comment
/// Used for generating API docs
fn documented_function() {
    // Implementation
}
```

### Control Flow

#### if Expressions

Execute code conditionally with boolean conditions.

```rust
let number = 7;

if number < 5 {
    println!("number is less than 5");
} else if number == 5 {
    println!("number is 5");
} else {
    println!("number is greater than 5");
}

// if in let statement (all arms must return same type)
let result = if number < 5 { "small" } else { "large" };
```

#### loop

Creates infinite loops with control mechanisms.

```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // Return value from loop
    }
};
```

#### while Loops

Execute code as long as a condition remains true.

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

#### for Loops

Iterate over collections safely and concisely.

```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

// Using ranges
for number in 1..4 {
    println!("{}!", number);
}
```

### Primitive Data Types

#### Scalar Types

Represent single values.

##### Integers

- **Signed**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- **Unsigned**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- **Default**: `i32`
- **Architecture-dependent**: `isize`/`usize` for indexing

```rust
let x: i32 = 42;     // Signed 32-bit integer
let y: u32 = 42;     // Unsigned 32-bit integer
let z = 42;          // Defaults to i32
```

##### Floating-Point Numbers

- **f32**: Single-precision
- **f64**: Double-precision (default)

```rust
let x = 2.0;         // f64 by default
let y: f32 = 3.0;    // f32
```

##### Booleans

- **bool**: `true` or `false`

```rust
let t = true;
let f: bool = false;
```

##### Characters

- **char**: Four bytes, represents Unicode Scalar Value

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

#### Compound Types

Group multiple values.

##### Tuples

Fixed-size, can group values of different types.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;  // Destructuring

println!("The value of y is: {}", y);
println!("First element: {}", tup.0);  // Access by index
```

##### Arrays

Fixed-size, all elements must be the same type, allocated on stack.

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March", "April", "May", "June",
              "July", "August", "September", "October", "November", "December"];

let first = a[0];
let second = a[1];
```

---

## Chapter 4: Understanding Ownership

This chapter introduces Rust's unique memory management system that ensures safety without a garbage collector.

### What is Ownership?

Ownership is a set of compile-time rules that govern how Rust manages memory, ensuring safety without a garbage collector.

### Three Core Rules

1. **Each value in Rust has an owner**
2. **There can only be one owner at a time**
3. **When the owner goes out of scope, the value will be dropped (memory reclaimed)**

### Memory: Stack vs Heap

#### Stack

- **Fast**, LIFO (Last-In, First-Out)
- For fixed-size data known at compile time
- Automatic memory management

#### Heap

- **Slower**, for data of unknown or variable size at compile time
- Accessed via pointers
- Ownership rules primarily manage heap data

### The String Type

Used to illustrate ownership because it's a growable, mutable, heap-allocated string, unlike immutable string literals.

```rust
let s1 = String::from("hello");  // Heap-allocated, growable
let s2 = "hello";                // String literal, immutable
```

### Data Interaction: Move vs Copy

#### Copy Semantics

For simple, stack-only types (integers, booleans, char, and tuples of Copy types), assignment creates a copy.

```rust
let x = 5;
let y = x;  // x is copied to y, both are valid
println!("x = {}, y = {}", x, y);  // Works fine
```

#### Move Semantics

For heap-allocated types (like String), assignment moves ownership.

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1's value moves to s2
// println!("{}", s1);  // This would ERROR! s1 is no longer valid
println!("{}", s2);  // This works
```

#### clone()

The explicit method to create a deep copy of heap-allocated data.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Deep copy
println!("s1 = {}, s2 = {}", s1, s2);  // Both valid
```

### References and Borrowing

#### References (&)

Allow you to refer to a value without taking ownership. Guaranteed by the compiler to always point to valid data.

#### Borrowing

The act of creating a reference.

#### Immutable References (&T)

Allow reading data; multiple immutable references can exist simultaneously.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1 borrows s1
    println!("Length of '{}' is {}.", s1, len);  // s1 is still valid!
}

fn calculate_length(s: &String) -> usize {  // s is a reference
    s.len()
}
```

#### Mutable References (&mut T)

Allow modifying data; only one mutable reference to a value can exist at a time.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

#### Reference Rules Summary

At any given time, you can have either:

- One mutable reference, OR
- Any number of immutable references

References must always be valid.

```rust
let mut s = String::from("hello");

let r1 = &s;  // OK
let r2 = &s;  // OK
// let r3 = &mut s;  // ERROR! Can't have mutable reference when immutable ones exist

println!("{} and {}", r1, r2);  // r1 and r2 are no longer used after this point

let r3 = &mut s;  // OK now
println!("{}", r3);
```

### The Slice Type (&str, &[T])

A type of reference that points to a contiguous sequence of elements within a collection without taking ownership.

#### String Slices

```rust
let s = String::from("hello world");
let hello = &s[0..5];  // "hello"
let world = &s[6..11]; // "world"

// These are equivalent:
let slice = &s[0..2];
let slice = &s[..2];
let slice = &s[3..];
let slice = &s[..];
```

#### Array Slices

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // [2, 3]
```

### Key Distinction: Mutability vs Shadowing

#### Mutability (mut)

Allows changing the value of an existing variable. The variable's type cannot change.

```rust
let mut x = 5;
x = 6;  // OK
// x = "hello";  // ERROR! Can't change type
```

#### Shadowing (let)

Declares a new variable with the same name, effectively hiding the old one. Allows changing the type.

```rust
let x = 5;
let x = "hello";  // OK - new variable, different type
```

---

## Summary

### Chapter 3 Key Takeaways:

- Functions require explicit type annotations
- Control flow includes `if`, `loop`, `while`, and `for`
- Primitive types include integers, floats, booleans, and characters
- Compound types include tuples and arrays

### Chapter 4 Key Takeaways:

- Ownership is Rust's unique memory management system
- Three core rules govern ownership
- References allow borrowing without taking ownership
- Slices provide safe access to parts of collections
- Understanding move vs copy semantics is crucial

