# Summary of chapter 3 and 4

## chapter 3 talks about Variables and Mutability, Data Types, Functions, Comments and Control Flow

- we define functions with fn, put in the parameters, and return values.
- we use `//` for comments.
- control flow. if, else, loops, while, and for.
- we use while in place of combine loops with if, else, break, and continue.
- we use the for loop to iterate over a range of values or a collection.

## chapter 4 talks about ownership
- ownership is how rust manages memory. 

- there are three rules of ownership:
1. Each value in Rust has an *owner*.
2. A value can have only one owner at a time.
3. When the owner of a value goes out of scope, Rust will automatically clean up that value.

- learnt about scope, cloning, and references.
- the scope of a piece of code can be defined with curly braces `{}`. 
- cloning can be done by using the `.clone()` method, which creates a deep copy of the value.
- references is how you take ownership of a value without transferring ownership. the `&` symbol is used to create a reference.
- the action of creating a reference is called borrowing

- The Rules of References
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

- slices are like the name implies slices of data. when you reference a slice, you are borrowing a part of the data. Slices are created using the `&` symbol followed by the range of indices you want to borrow, like `&s[0..2]`.
