# Rust Notes

A summary of Rust concepts including variables, constants, shadowing, and data type

## Variables

By default, variables are immutable. You can make them mutable by adding `mut`.  
This will indicate that other parts of the code will be changing that value.

Example:  
let mut x = 5  
println!("x is {}", x)  
x = 1  
This will compile. It will not compile without adding `mut`.



## Constants

- Like immutables, constants are values that are bound to a name and are not allowed to change.  
- Don't use `mut` with constants.  
- Constants are not just immutable by default — they are always immutable.  
- Declare constant using `const` keyword.  
- Must annotate the value type.  
- Use all uppercase for naming convention.



## Shadowing

Declaring a new variable with the same name as a previous variable.  
The second `let` will shadow the first.

Example:  
let x = 5  
let x = x + 1  
The value of x will be 6.  
The compiler uses the most recent value.

- When shadowing, use the `let` keyword.  
- Don't use `mut`, the compiler will throw an error.



## Summary

In Rust, mutability means the value may change, not the type may change.  
That’s why the example below won’t compile:

let mut space = " "  
space = space.len()  

The compiler sees the first as a string and the second assignment as a number.  
Use shadowing to make it work:

let space = " "  
let space = space.len()



## Data Types

Rust is a statically typed language — it must know the types of all variables at compile time.

- Scalar  
- Compound



## Scalar Types

- Integers  
- Floating-point  
- Boolean  
- Character

### Integers

We have signed and unsigned types. Each can store numbers from -(2^n) to 2^n - 1.  
For example, i8 ranges from -128 to 127.

- Integer types default to i32.  
- Integers are numbers without a fractional component.



### Floating Point

They are numbers with decimal points, like 0.25, 3.61, -4.2, or 9999.99.  
They are not whole numbers, so we can’t use u32 or i8. Instead, we use f32 or f64.

- Rust defaults to f64 (the bigger one).  
- All floating points are signed — they can hold both positive and negative values.



### Boolean

They have 2 possible values: true and false.

- Boolean is one byte in size.  
- Use `bool` to annotate the type in Rust.



## Characters (char)

- Specify char literals with single quotes, as opposed to string literals which use double quotes.  
- `char` are not strings.  
- They take 4 bytes, equivalent to 32 bits.



## Compound Types

Compound types can group multiple values into one type.  
Rust has 2 primitive compound types:

- Tuples  
- Arrays



### Tuples

A way of grouping together multiple values with a variety of types into one compound type.

Example:  
let tup = (500, 6.4, 1)  
let (x, y, z) = tup  

You can also access a tuple using a period followed by the index:  
let x = (500, 6.4, 1)  
let five_hundred = x.0  
let six_point_four = x.1

You can also mutate a tuple by adding `mut`:  
let mut x = (1, 2)  
x.0 = 0  
x.1 = x.1 + 5



### Arrays

Used for multiple values. Unlike tuples, the elements or values must have the same type.

- Array is not flexible as the vector type (`Vec`) provided by the standard library.  
- Arrays are fixed in size. In uncertain cases, use `Vec`.

Example:  
let arr = [1, 2, 3]

### **Functions**: 
Functions in Rust are like a set of code instruction that let youto do specific tasks, they are reusable. Functions can return values by specifying a type with ->, where the last line (without a semicolon) is the return value, or you can use return explicitly. 

### **Comments**: 
Comments are notes you write in your code to explain what it does, and when rust see them it ignores them when running the program. 
- **Single-line comments**: Start with `//`
- **Multi-line comments**: Use `/*... */` or multiple `//` lines
- **Documentation comments**: Use `///` or `//!` for generating API documentation.

### **Control Flow**: 
Control flow is how Rust lets you make decisions or repeat actions in your program. It is like a rulebook that tells your program when to do things or repeat them.
- **if**: You can use if to check conditions, with else or else if for other cases. 
- **loop**: The loop keyword repeats code forever until you say break. 
- **while**: The while loop repeats while something is true. The for loop goes over a list. 
- **match**: The match expression compares values and picks a path, like in the guessing game to check if a guess is too big, too small, or correct. 
