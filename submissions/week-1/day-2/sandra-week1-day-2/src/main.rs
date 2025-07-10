use std::io;
use colored::*;

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

fn div(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        println!("Error: Division by zero");
        return 0.0;
    }
    a / b
}

fn main() {
    println!("Enter the first number: ");
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    let a: f32 = a.trim().parse().expect("Please type a number!");

    println!("Enter the second number: ");
    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");
    let b: f32 = b.trim().parse().expect("Please type a number!");

    println!();
    println!("{}", format!("Add: {}", add(a, b)).green());
    println!();
    
    println!("{}", format!("Sub: {}", sub(a, b)).blue());
    println!();
    
    println!("{}", format!("Mul: {}", mul(a, b)).yellow());
    println!();
    
    println!("{}", format!("Div: {}", div(a, b)).magenta());
    println!();
}
