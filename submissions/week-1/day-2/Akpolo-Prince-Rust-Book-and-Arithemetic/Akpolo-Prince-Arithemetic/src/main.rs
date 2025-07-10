
fn main() {

    let num1 = 10;
    let num2 = 3;
    
    println!("=== AKPOLO RUST CALCULATOR ===");
    println!("Using numbers: {} and {}", num1, num2);
    println!("------------------------");
    
    
    let sum = add(num1, num2);
    println!(" Addition: {} + {} = {}", num1, num2, sum);
    
    let difference = subtract(num1, num2);
    println!("Subtraction: {} - {} = {}", num1, num2, difference);
    
    let product = multiply(num1, num2);
    println!("  Multiplication: {} × {} = {}", num1, num2, product);
    
    let quotient = divide(num1, num2);
    println!(" Division: {} ÷ {} = {}", num1, num2, quotient);
    

    println!("Brain racked completed! 🎉");
}

fn add(a: i32, b: i32) -> i32 {
    a + b 
}


fn subtract(a: i32, b: i32) -> i32 {
    a - b
}


fn multiply(a: i32, b: i32) -> i32 {
    a * b
}


fn divide(a: i32, b: i32) -> f64 {
    
    a as f64 / b as f64
}