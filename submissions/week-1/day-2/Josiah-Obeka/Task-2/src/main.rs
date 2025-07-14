use std::io;

fn main() {
    // Take input for the first number
    let mut input_a = String::new();
    println!("Enter the first number:");
    io::stdin()
        .read_line(&mut input_a)
        .expect("Failed to read input");
    let a: i32 = input_a.trim().parse().expect("Please enter a valid number");

    // Take input for the second number
    let mut input_b = String::new();
    println!("Enter the second number:");
    io::stdin()
        .read_line(&mut input_b)
        .expect("Failed to read input");
    let b: i32 = input_b.trim().parse().expect("Please enter a valid number");

    // Functions
    fn addition(x: i32, y: i32) -> i32 {
        x + y
    }
    fn subtract(x: i32, y: i32) -> i32 {
        x - y
    }
    fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }
    fn divide(x: f32, y: f32) -> f32 {
        x / y
    }

    // Calculations
    let sum = addition(a, b);
    let sub = subtract(a, b);
    let mul = multiply(a, b);
    let div = divide(a as f32, b as f32);

    // Output
    println!(
        "The first number is: {}, and the second number is: {}",
        a, b
    );
    println!("The sum of {} and {} is: {}", a, b, sum);
    println!("The result of subtracting {} from {} is: {}", a, b, sub);
    println!("The multiplication of {} and {} results in: {}", a, b, mul);
    println!("The result of dividing {} by {} is: {}", a, b, div);
}
