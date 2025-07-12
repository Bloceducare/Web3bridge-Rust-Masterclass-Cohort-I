// function to add 2 numbers
fn add_two_numbers(x: i32, y: i32) -> i32 {
    x + y
}

// function to subtract 2 numbers
fn subtract_two_numbers(x: i32, y: i32) -> i32 {
    x - y
}

// function to multiply 2 numbers
fn multiply_two_numbers(x: i32, y: i32) -> i32 {
    x * y
}

// function to divide 2 numbers
fn divide_two_numbers(x: i32, y: i32) -> f64 {
    x as f64 / y as f64
}

// main function
fn main() {
    let a = 64;
    let b = -6;

    let addition = add_two_numbers(a, b);
    let subtraction = subtract_two_numbers(a, b);
    let multiplication = multiply_two_numbers(a, b);
    let division = divide_two_numbers(a, b);

    println!("Addition Result: {}", addition);
    println!("Subtraction Result: {}", subtraction);
    println!("Multiplication Result: {}", multiplication);
    println!("Division Result: {}", division);
}