fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let num1 = 15;
    let num2 = 3;
    
    println!("ADDITION: {}", add(num1, num2));
    println!("SUBTRACTION: {}", subtract(num1, num2));
    println!("MULTIPLICATION: {}", multiply(num1, num2));
    println!("DIVISION: {}", divide(num1, num2));
}