## What I Understand

After going through the rest of Chapter 3 and all of Chapter 4 of _The Rust Book_, here’s what I feel I’ve been able to grasp pretty well:

- I understand that variables are **immutable by default** in Rust, and you have to use `mut` to make them changeable.
- Rust has clear **data types**, like integers, floats, booleans, chars, arrays, and tuples, and you usually have to be explicit about what you’re working with.
- The way **functions** work makes sense to me how parameters are passed, how expressions can return values without using the `return` keyword.
- I get the basic idea of **ownership** every value has one owner, and when ownership is transferred, the previous variable becomes invalid.
- I understand **borrowing** through references using `&` to avoid ownership transfer, and how **mutable references** (`&mut`) allow us to modify borrowed data.
- The rules around **only one mutable reference at a time**, or multiple immutable ones, help avoid bugs even though strict, they’re understandable.
- I also learned how **string slices** (`&str`) work and how they let us refer to parts of a string without taking ownership.

## What I Don’t Understand and Still Struggle With

Even though I understand the concepts on the surface, there are still some parts that feel very much confusing or hard to wrap my head around.

- I still don’t fully understand **why Rust is so strict** about having only one mutable reference or no mixing of mutable and immutable ones I get the rule, but not the deep reasoning behind it.
- When I see **borrow checker errors**, I sometimes struggle to make sense of the messages especially when the scope or lifetime involved isn’t obvious.
- The book briefly mentions **lifetimes** (like `'static`) when talking about dangling references, but I don’t understand what lifetimes really are or how to use them properly yet.
- The fact that you can’t slice a string randomly because of **UTF-8 character boundaries** is a bit scary I’m not sure how to safely handle string slicing with non-ASCII characters.
- I also still get a bit confused about when to use `&String` vs `&str` in function parameters I know `&str` is more flexible, but when exactly should I use each?
