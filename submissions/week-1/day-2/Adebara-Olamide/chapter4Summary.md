# Chapter 4 Recap: Understanding Ownership in Rust

In this chapter, I was introduced to one of Rust’s most important features **ownership**. It’s how Rust manages memory safely and efficiently without needing a garbage collector.

Here are the key takeaways:

- Every piece of data in Rust has an **owner** usually a variable that’s responsible for it.
- When the owner goes **out of scope**, Rust automatically frees the memory. You don’t need to manually clean it up.
- By default, when you assign a value to another variable or pass it to a function, the **ownership moves**. The original variable can no longer be used.
- If you want to **use data without taking ownership**, you can **borrow** it using an ampersand (`&`).
- This borrowing system helps you write safe, efficient code without worrying about things like double frees or memory leaks.

Ownership might feel strict at first, but it’s what gives Rust its power preventing bugs at compile time and making your programs more reliable.
