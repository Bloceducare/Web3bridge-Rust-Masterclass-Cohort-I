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
    let a = 10.0;
    let b = 2.0;
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
