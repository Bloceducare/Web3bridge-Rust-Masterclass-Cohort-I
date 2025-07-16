use std::io::stdin;

fn add(a: i32, b: i32) -> i64 {
    a as i64 + b as i64
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i64 {
    a as i64 * b as i64
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    println!("Input a number: ");

    let mut number1 = String::new();

    stdin().read_line(&mut number1).expect("Failed to read line");

    let number1: i32 = match number1.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("Bad input"),
    };

    println!("Input another number: ");

    let mut number2 = String::new();

    stdin().read_line(&mut number2).expect("Failed to read line");

    let number2: i32 = match number2.trim().parse() {
        Ok(x) => x,
        Err(_) => panic!("Bad input"),
    };

    println!("Addition: {}", add(number1, number2));
    println!("Subtraction: {}", subtract(number1, number2));
    println!("Multiplication: {}", multiply(number1, number2));
    println!("Division: {}", divide(number1, number2));
}
