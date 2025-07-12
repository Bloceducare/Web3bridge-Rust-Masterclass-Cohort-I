
CHAPTER THREE
The remaining of Chapter three deals with functions, comments and control structures.

Function
Are used to break down a program into smaller, more manageable pieces. It contains signatures and body. Function signatures are name, parameters and return type while function body contains business logic or block of code.

We define function using fn keyword followed by its name, a list of parameters enclosed in parentheses, return type(optional) is specified after an arrow and a block of code enclosed in curly braces. Below is basic structure:

fn my_age(age: u8) -> return type {
    block of codes
    
}

This chapter also explained statement and expression. A statement is like a command that performs an action but does not return a value. For example, variable or function declaration is a statement.

While expression evaluates to a value. That is, it would return a value. X + Y  or a function that return value is an expression.

Comments
Are ways to give explanation to the code. It comes in two ways.
1. Single-line comment which starts with double back slash (//) to the end of the line.
2. Muitiple-line comment which starts with single back slash together with asterik(/*) and closes with asterik followed by single slash(*/). This is usually more than a line.
This likes:
/* Below codes are
for calculating profit rate
for the period of a year
*/

Control Structures.
These are used for decision making in the code. Rust has the following control structures.
1. if expression that contains if statement, if else statement, and else if statement.
2. Loop which contains loop, while loop and for loop.

CHAPTER FOUR

This chapter is all about ownership, references and string slice type. Before explanation of ownership, memory type are explained. Stack and heap.

Stack is a linear data structure that stores data in order. That is, it follows first in last out (FILO). Only fixed sized data are stored here such as scalar data type. That is, data that the size is known at compile time.

While heap stores dynamic sized data types such as string, vector etc. That is, data that are dynamically changed at runtime.

Ownership in rust means memory safety and safety is all about absence of undefined behaviors. It is like a gatekeeper to ensure safety in rust program. Put differently, ownership is a set of rules that compiler enforce at a compile time not at runtime. In summary, here are the ownership rules:
1. A value must have a signer owner.
2. There can only be one owner at a time.
3. The value drop when the owner goes out of scope (dangling)
4. No mutable and immutable reference at a time(data race)

Reference is a kind of pointer and can also be called borrowing. It is way to have access to the value without copying or cloning it. This is like when A borrow something from B. The ownership is still with B.  It can be immutable or mutable.

It is also learnt from this chapter that every variable possess three permission namely read, write and own permission. Read and own are default permissions while write is only granted when variable is mutable.

For example, when variable A points to B, variable B is ready-only variable automatically. That is, B can neither be mutable nor ownable but readable by variable C. This points to rule number 4 above.


Strings and Slices Types. 
Firstly, Strings is a collection of bytes, growable-type, and heap-allocated data structures stores in a Vec<u8>. 
e.g let greeting = String::from("Hello World!").

There are also String literal which are sequence of charaters enclosed in a double quotes and have type &str. They are valid throughout the program runtime because they are hardcorded into program binary. For example,
let first_name = "Olawale"

whereas Slice is a reference or part of collection types and like string literals, has type &str. It helps to work with part of them without copying it. 
We have Strings slices and vector or array slices.

Example of Strings slice

    let my_string = String::from("Hello, World!");
 
    let hello = &my_string[0..5];

    println!("{}", hello);

This will print: Hello.


for array slice example

let numbers = [1, 2, 3, 4, 5];
 
let slice = &numbers[1..3];

println!("{:?}", slice);

It will print: 2, 3

