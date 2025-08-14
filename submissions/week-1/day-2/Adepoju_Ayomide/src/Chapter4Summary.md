## OWNERSHIP
Ownership in Rust is a set of rules that govern memory management, enforced by the compiler to ensure safe and efficient code.

## STACK AND HEAP
In Rust, the stack stores data of fixed size known at compile time, while heap stores data of unknow size at compile time.

## OWNWERSHIP RULES
Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

Each value in Rust has an owner. (e.g)
// owner of s is s
let s = string::from("rust")

There can only be one owner at a time. (e.g)
let s = string::from("dog")
// owner s is s1
let s1 = s

When the owner goes out of scope, the value will be dropped. (meaning you cant referance it again)
let s = string::from("cat");
{
    s;
    s is dropped
} scope ends here
this will not compile.

## Borrowing
Borrow - Temporarily use a value without taking ownership
--> Create a Reference (either mutable or immutable)
let s = string::from("rust")
s1, s2, s3 have read only access to s
let s1 = &s;
let s2 = &s;
let s3 = s2;

// Mutable Borrow
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);


--> Doesn't have ownership
--> Immutable reference-: any number of read only access to a value
--> Mutable Reference-: only one read and write access to a value at a time

--> Either Mutable or Immutable borrow, but not both simultaneously
let mut s = string::from("rust")
s1, s2, s3 have read only access to s
let s1 = &s;
let s2 = &s;
let s3 = &mut s;

--> Reference must not outline the value
let s = string::from("rust")
let s1 = &s;
{
    let s2 = s;
} // s2 no longer exist
    // s1 reference s

## The Rules of References
At any given time, you can have either one mutable reference or any number of immutable references.
References must always be valid.
