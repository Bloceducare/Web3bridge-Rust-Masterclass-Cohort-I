### Chatpter 2 Rust Book

## To initlize a project

use

```rust
Cargo new <project name>
```

It will generate a new project with src folder and a Cargo.toml

The

Cargo.toml file look like this

```rust
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

We are building a Processing a Guess game

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

the std is a standard libray in rust which we get the io from,
The io allow us to accept the user input

```rust
fn main() {

}
```

Using the fn in rust allow us to declare a new fucntion which follows by the parentheses() which does not take any parameter followed by curly braces which house the body of function

```rust
    println!("Guess the number!");

    println!("Please input your guess.");
```

println! is a macro which is user to print on the screen which give the user clarity how the process of the game.

```rust
    let mut guess = String::new();
```

We declare a variable with let with a variable name guess which allow us to create a new string which is also a mutable variable, this meens we can modify the variable.

The :: syntax that follow the String is an indicator that we are creating new string to bind guess together.

For comment we can use this syntax // as a comment indicator.

```rust
io::stdin()
        .read_line(&mut guess)
```

the io comes from the std rust standard library. which give access to stdin, this stdin allow us to get input from a user. stdin then call read_line to read the input the user type in the line which is referenec to guess and not taking ownership and mutating it as well

```rust
.expect("Failed to read line");
```

We implement the .expect to handle any error during expression which can or might failed.

Result is a enumeration type which consist of Err and Ok, if it is success it is ok why failed is Err.

We can not only println! string but also print values as by adding curly braces to the

```rust
    println!("You guessed: {}", guess);
```

Generating secret number
We use a dependence provided by rust

```rust
rand crate
```

With the rand crate we can generate a random number with its function.

To add this dependence to your project. Open your Cargo.toml

```rust
[dependencies]
rand = "0.8.5"
```

rand is the package name why the = sign is to bind it with the version which is "0.8.5" which is written in semantic Versioning smae as ^0.8.5. This version must be compatible with 0.8.5

After that you run

```rust
cargo build
```

which cargo build will update your dependence from crate

to update the rand you can use

````rust
cargo update
``

generating random number

```rust
use rand::Reg
````

The Rng define the trait method we need to implement for the random generator.

```rust
rand::thread_rng
```

This function gives us the access to actual number we want to generate, which uses the Rng trait.

## Comparing the Guess to the secret nymber generated

```rust
use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

First we import Ordering format using the use keyword bring the typed call std::cmp::Ordering in to the scope. The Ordeing is anm enum tyoe which have different variants such as Less, Greater, and Equal. This compare outcome values together.

Following the match keyword in the code which is made up of two arms which consist of the pattern we are trying to match and the value which is given to match up with. The next on the line is the use of Ordering which comes from the standard library of std, which we use the Enum types to compare the value such as the Less, Greater and Equal. It checks if the value match the pattern we are comparing with, if not it skip untill it meet the value to the next line.

if we compile the code above will get runtime error, because the ordering expect the integer type but got a string which we need to modify.

```rust
    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

We create the variable guess which we have already created which rust allow us to shadow variables.And Bind it wuth the guess.trim().parse() this allow us to convert string to another type which is being converted to number which we declaration of the type with after the variable name : u32.

## Allowing Multiple Guesses with Looping

```rsut
    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

Loop allow us to read item in an array once instead of reading it one after the order or line by line. When user enter wrong number user can keep try even if the input is the correct guess. User can use control + c to stop the loop. But we can use break to stop the program from running after the use get the right answers.

```rust
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Handling Invalid Input

```rust
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // --snip--
```

The parse of the string to number returns the Return type as which is

Ok(num) => num,
Err(\_) => continue,

If the parse is able to parse the string to number it will return okay else it will return Erro
