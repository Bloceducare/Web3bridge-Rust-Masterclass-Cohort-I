# Rust - Chapter 3 - Common Programming Concept & Chapter 4 - Ownership Summary

# Rust Chapter 3 Summary - Common Programming Concepts

## Variables and Mutability

In Rust, variables are immutable by default. For example:

```rust
let x = 5;
```

This means x cannot be changed after we set it. To make a variable changeable, we use the `mut` keyword:

```rust
let mut x = 5;
x = 6; // This will work
```

Constants are different from variables, they're always immutable and we declare them with `const` and usually the capital case with underscore to separate them:

```rust
const MAX_POINTS: u32 = 100_000;
```

we can also shadow variables by declaring a new variable with the same name:

```rust
let x = 5;
let x = x + 1; // This creates a new variable x
```

## Data Types

Rust has two main data type categories: scalar and compound.

**Scalar types** include integers (`i32`, `u32`), floating-point (`f64`), booleans (`bool`), and characters (`char`).

**Compound types** group multiple values. The main ones are tuples and arrays:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let arr = [1, 2, 3, 4, 5];
```

## Functions

Functions are defined with `fn` and can take parameters and return values:

```rust
fn main() {
    let result = add_numbers(5, 3);
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y  // No semicolon means this is returned
}
```

## Control Flow

**If expressions** work like we'd expect:

```rust
let number = 6;
if number % 2 == 0 {
    println!("even");
} else {
    println!("odd");
}
```

**Loops** come in three types:
- `loop` - runs forever until we `break`
- `while` - runs while a condition is true
- `for` - iterates over collections

```rust
// loop
loop {
    println!("again!");
    break;
}

// while
let mut number = 3;
while number != 0 {
    number -= 1;
}

// for
let arr = [1, 2, 3];
for element in arr {
    println!("{}", element);
}
```

Rust is a language that would catch errors at compile time, so if what we write is in anyway wrong with the types or we try to use immutable variables in a wrong way, it'll tell we before we run the program.



# Rust - Chapter 4 - Ownership Summary

## What is Ownership?

Ownership is a way to manage memory in Rust.

## Stack vs Heap

**Stacks**: It is about last-in-first-out. It's all about the last function call or variable that goes in is the first one that comes out.

**Heaps**: Heaps takes up a space, registers it as in use, and then returns a pointer to the address. Another point, accessing data on the heap is slower than on the stack because we have to follow the pointer to get there.

## Ownership Rules

1. Each value has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

In ownership, the variable scope is being implemented where a scope is the range within a program for which an item is valid.

## String Type

To manage memory in Rust, we use the string type not string literals. For example:

```rust
let S = String::from("hello");
```

This way, this kind of string can be mutated. To change or modify a string after creating it, the string has to be declared as mutable.

## Memory Management

The way Rust handles memory allocation and garbage collection is by the variable scope, where allocation of memory starts at the beginning of the definition and ends at where the use of the function ends. Instead of having to free memory like other languages, Rust uses a function called drop.

## Copy vs Move

A variable can be copied or moved, e.g.:

```rust
let x = 5;
let y = x;
```

In this case, both x and y are 5 and this is because y copied the binded value of x.

But in the case of a string, we have:

```rust
let S1 = String::from("hello");
let S2 = S1;
// println!("{}", s1);  // There would be error here, because s1 is no longer valid
println!("{}", s2);  // This is the better approach
```

In this case, we have what other languages call a shallow copy to make this happen, which is called move in Rust, where S2 copied the pointer capacity and length of S1. This means S1 is no longer usable after the move. Rust does this to prevent having two variables trying to free the same memory.

If we want to copy S1 to S2 properly, we use the dot clone method:

```rust
let S2 = S1.clone();
```

The reason why we do not clone on integers is because integers have a definite size and are stored on stack.

## Referencing and Borrowing

A reference is like a pointer or an address pointing to the value of a stored data. To use that, we call it a `&variable` (using the ampersand symbol).

For mutable references, we can only have one mutable reference to a piece of data at a time. we also can't have both mutable and immutable references to the same data at the same time. If we want to use a mutable reference, make sure any other references end their scope first, or create the mutable reference in a different scope.

## Dangling References

Dangling references happen when we have a reference pointing to memory that has been freed or is no longer valid. Rust will not let this happen - it will catch this at compile time and throw an error. The best way to solve a dangling reference is to return the actual data directly instead of trying to return a reference to local data.
