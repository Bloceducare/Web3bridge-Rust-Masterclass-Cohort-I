use std::io;

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Error: Division by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn main() {
    println!("=== Basic Arithmetic Program ===\n");
    
    let x = get_number("Enter the first number:");
    let y = get_number("Enter the second number:");
    
    println!("\nUsing values: {} and {}\n", x, y);
    
    println!("Addition:");
    let sum = add(x, y);
    println!("   {} + {} = {}", x, y, sum);
    println!();
    
   
    println!("Subtraction:");
    let difference = subtract(x, y);
    println!("   {} - {} = {}", x, y, difference);
    println!();
    

    println!("Multiplication:");
    let product = multiply(x, y);
    println!("   {} × {} = {}", x, y, product);
    println!();
    
    
    println!("Division:");
    match divide(x, y) {
        Ok(quotient) => println!("   {} ÷ {} = {}", x, y, quotient),
        Err(error) => println!("   {}", error),
    }
    
    println!("\n=== Operation successfully! ===");
}