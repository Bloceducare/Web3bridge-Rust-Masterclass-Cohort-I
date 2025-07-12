# THINGS I KNOW
## Summary Of Chapter 3
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


## Summary Of Chapter 4
### **Ownership**: 
Ownership is Rust’s way of managing memory to keep programs safe and fast, like rules for who gets to use a toy. Every value (like a number or string) has one owner (a variable), and only one owner can have it at a time. When the owner goes out of scope, the value is cleaned up. A “move” happens when you give a value to another variable, so the first variable can’t use it anymore. For example,

```
let s1 = String::from("hello");
let s2 = s1;
```
means s1 loses the value, and s2 owns it, so 
```
println!("{}", s1); 
```
would cause an error. Ownership is like passing a toy to a friend, you can’t play with it anymore unless you get it back.

### **References and Borrowing**: 
Borrowing lets you use a value without taking it away from its owner, like looking at a friend’s toy without keeping it. A reference (&) lets you “look at” a value,
```
let s1 = String::from("hello"); 
let s2 = &s1;
```
so both s1 and s2 can be used. A mutable reference (&mut) lets you change the value, like 
```
let mut s = String::from("hello"); 
let r = &mut s; 
r.push_str("!");
```
Only one &mut borrow can exist at a time to avoid conflicts. Rust rules say you can have many immutable & borrows or one &mut borrow. Borrowing is used to pass strings to functions without moving them. Borrowing is like borrowing a library book, you can read or write in it, but you follow strict rules to keep things safe.

### **Slice Type**: 
Slices let you look at part of a string or array without owning it, like cutting a piece of cake to share. A slice is a reference to a chunk of data, like &str for strings or &[i32] for arrays. For example, 
```
let s = String::from("hello world"); 
let slice = &s[0..5]; 
```
gives you “hello” as a slice. Slices are useful for functions that need to work with part of a value, like checking the first word in a string. Slices are borrowed, so they don’t take ownership.

# THINGS I DONT KNOW
 Plenty things.
  