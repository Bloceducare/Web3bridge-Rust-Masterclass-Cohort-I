
fn add() -> i32 {
    3 + 3
}

fn sub() -> i32 {
    3 - 2
}

fn multiply() -> i32 {
    3 * 2
}

fn divide() -> i32 {
    6 / 2
}

fn modulus() -> i32 {
    5 % 2
}

fn main() {
    println!("Hello, world!");

    println!("Addition: {}", add());

    let subtraction = sub();

    println!("Subtraction: {}", subtraction);


    println!("Multiply: {}", multiply());

    println!("Division: {}", divide());

    println!("Modulus: {}", modulus());

}

// cargo run 
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
//      Running `target/debug/basic_arithmetic`
// Hello, world!
// Addition: 6
// Subtraction: 1
// Multiply: 6
// Division: 3
// Modulus: 1
// amityclev@Amityclevs-MacBook-Pro basic_arithmetic % 