use colored::*;
use std::io;

fn main() {
    println!("Enter the first number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let a: u64 = input.trim().parse().expect("Please type a number!");

    println!("Enter the second number: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let b: u64 = input.trim().parse().expect("Please type a number!");

    add(a, b);
    sub(a, b);
    mul(a, b);
    div(a, b);
}

fn add(a: u64, b: u64) -> u64 {
    let result = a + b;
    println!(
        "The sum of {} and {} is {}",
        a,
        b,
        result.to_string().blue() // colorize only works on strings, hence the conversion
    );
    result
}

fn sub(a: u64, b: u64) -> u64 {
    let result = a - b;
    println!(
        "The difference of {} and {} is {}",
        a,
        b,
        result.to_string().red()
    );
    result
}

fn mul(a: u64, b: u64) -> u64 {
    let result = a * b;
    println!(
        "The product of {} and {} is {}",
        a,
        b,
        result.to_string().green()
    );
    result
}

fn div(a: u64, b: u64) -> u64 {
    let result = a / b;
    println!(
        "The division of {} by {} is {}",
        a,
        b,
        result.to_string().yellow()
    );
    result
}
