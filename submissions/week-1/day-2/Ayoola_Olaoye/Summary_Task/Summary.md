# Summary of Chapter 2: Programming a Guessing Game

## I understand chapter 2 of the book very well. Probably becuase i've read it more than once. 😁

- Concepts covered in Chapter 2 include:

  - Setting up a new project using Cargo. I understand this to be Rust package manager, just like npm for Node.js. Generates a cargo.toml, cargo.lock etc.
  - Assigning values to variables using `let` and `mut`.
  - The println! macro for printing to the console.
  - Using external crates, like the `rand` crate to generate random numbers.
  - Bringing some crates into scope using `use` like std::io.
  - Using match, loop etc.

  * I had a question on why we had to initialize 'guess' as a string and then convert it into a number using trim and parse. This has been addressed in class. Also, on further reading, I learnt, input into the terminal comes with extra characters like newlines, spaces etc. So, we need to trim it before parsing it into a number. Also, using String::new() allows us to get a growable space on the heap, which is useful for storing user input.




  # Summary of Chapter 3: Common Programming Concepts

  ## Chapter 3 covers common programming concepts like variables, functionsm comments, control flow etc

  - Most of the concepts in this chapter are similar to those in other programming languages, so I found it easy to understand. However, there were some that were Rust-specific like shadowing.
  - I also find it strange that passing a value into a function actually moves the value into the function, rather than passing a reference to it. This is different from other languages like JavaScript, where values are passed by reference. But, I get the idea and it makes sense. Since you can always pass a reference if you want to avoid moving it.
  - There is also a different between a String and a string literal. As a string literal is not growable, while a String is.
