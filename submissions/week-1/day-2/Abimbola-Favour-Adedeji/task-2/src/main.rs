fn main() {
    println!("Addition of {} and {} = {}", 5, 3, add(5, 3));
    println!("Subtraction of {} and {} = {}", 10, 4, subtract(10, 4));
    println!("Multiplication of {} and {} = {}", 6, 7, multiply(6, 7));
    println!("Division of {} and {} = {}", 20, 4, divide(20, 4));
    println!("Division of {} and {} = {}", 20, 0, divide(20, 0));
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

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Error: Division by zero is not allowed.")
    } else {
        a / b
    }
}
