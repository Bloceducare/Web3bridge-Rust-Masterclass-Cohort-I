RUST BOOK

Chapter three
Scalar types are types that represent a single value
-Integers : - They represent a whole number and not part of a whole so they are called numbers without a fractional component, they can be unsigned or signed integers, the isize and usize types of integers depends on the computer arch you are running on
-Floating point numbers
-Booleans
-Characters

Compound Types can group multiple types into one type, rust has two primitive compound types arrays and tuples
- Tuple types - Ways of grouping together a number of values with variety of types into one compound type. tuples have a fixed length once declared cannit grow or shrink in size

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
you can access vales in a tuple using the indexes of the value assigned to the tuple

- Arrays - Elements in an array must be of the same type, arrays in rust have a fixed length 
fn main() {
    let a = [1, 2, 3, 4, 5];
}
Arrays are used when you are dealing with fixed data types or when you knoe the data will not grow or shrink
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5];

Arrays are used when you want your data to be allocated on a stack

Accessing Array Elements

An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:

fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

Functions
The main.rs is the entry point to a rust program
Functions are a set of instructions given to a program to execute and bring forth outputs

We define a function in Rust by entering the fn keyword followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.


    Statements are instructions that perform some action and do not return a value.
    Expressions evaluate to a resultant value. Let’s look at some examples.
In rust you have to state the type of return a value a function is expecting



OWNERSHIP
Ownership is a set of rules that govern how rust manages memory

The stack and the heap
Code uses both the stack and heap memory at runtime
Data added to a stack is done in an orderly fashion, data is pilled on top of another and when data is to be accessed on the stack it is done by the last in first out fashion. Data stored on the stack must be fixed sized data and must be known

Data stored on the heap is less organized and does not need to have a fixed size, when a data is to be  stored on a heap the memory allocator finds an empty space on the heap marks it as in use and returns the address of that location. The pointer is stored on the stack because it is fixed and known, the data is pushed to the heap and the pointer to the location of the data is stored on the stack

Understanding memory management is key to understanding how the rust programing language works

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Ownership Rules

    Each value in Rust has an owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.


