Chapter 3 
Function
functions are denoted in rust with fn, 
it has curly bracket  ‘{}’that indicates the beginning and end of it 

A function can call another function 

```
fn main() {
    greetings();
}

fn greetings() {
    println!("Hello, world!");
}

```

A function can take in parameter’s of any data types which needs to be declared 

```
fn main() {
    greeting(“John”);
}


fn greeting(name: &str) {
    println!("Hello, {}", name);
}


```

Function bodies are made up of a series of statements optionally ending in an expression. 

* Statements are instructions that perform some action and do not return a value.

```
let y = 6;
```
Statements do not return values


* Expressions evaluate to a resultant value. 


Comment 

// hello, world


 the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line. For comments that extend beyond a single line, you’ll need to include // on each line, like this:

// So we're doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what's going on.

Comments can also be placed at the end of lines containing code:

Control Flow
The ability to run some code depending on whether a condition is true and to run some code repeatedly while a condition is true are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

if Expressions
An if expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”


All if expressions start with the keyword if, followed by a condition. we can also include an else expression,

Handling Multiple Conditions with else if
You can use multiple conditions by combining if and else in an else if expression. 


Repetition with Loops
The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.


```
fn main() {
    loop {
        println!("again!");
    }
}
```

Conditional Loops with while
A program will often need to evaluate a condition within a loop. While the condition is true, the loop runs. When the condition ceases to be true, the program calls break, stopping the loop. 


Looping Through a Collection with for
You can also use the while construct to loop over the elements of a collection, such as an array. 


Chapter 4
Ownership
Ownership is a set of rules that governs how a Rust program manages memory. It ensures memory safety without needing a garbage collector.
There are three main ownership rules:
1. Each value in Rust has a variable that’s its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value is dropped (memory is freed).
Stack and Heap
* The Stack stores values with a known, fixed size at compile time.
* The Heap is used for values that are dynamic or whose size is unknown at compile time.
Think of the stack like a stack of books: the last one in is the first one out (LIFO — Last In, First Out).
Pushing means adding data, popping means removing it.

```
fn main() {
    let x = 5; // Stored on the stack
    let y = String::from("hello"); // Stored on the heap
}
```
Ownership in Action

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s1); // This will throw an error!
}
```
In the above code, ownership of s1 is moved to s2. After the move, s1 is no longer valid.
To clone the value and keep both variables:

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // Works fine
}
```

Functions and Ownership
When passing a value to a function, ownership can also move.

````
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s is moved here
    // println!("{}", s); // Error: s no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
```

For simple types like integers, Rust uses Copy, not Move:

```
fn main() {
    let x = 5;
    makes_copy(x); // x is still valid here
    println!("{}", x);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}
```

References and Borrowing
You can borrow a value using a reference, which allows you to access the value without taking ownership.

```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
Here, &s1 means we are borrowing s1 without giving up ownership.

Mutable References
You can borrow a value mutably, but only one mutable reference is allowed in a particular scope to avoid data races.

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
❗ You can either have one mutable reference OR any number of immutable references in a scope, but not both at the same time.

The Slice Type
A slice lets you reference a portion of a collection (like an array or a string) without taking ownership.

```
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);
}
```
Slices are references, so the original string is still accessible and not moved.
