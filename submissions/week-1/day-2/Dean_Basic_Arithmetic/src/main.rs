fn add_two_numbers(a: i8, b: i8) -> i8 {
    a + b
}

fn sub_two_numbers(a: i8, b: i8) -> i8 {
    if a > b { a - b } else { b - a }
}

fn multiply_two_numbers(a: i8, b: i8) -> i8 {
    a * b
}

fn divide_two_numbers(a: i8, b: i8) -> i8 {
    if b < 1 { 0 } else { a / b }
}

fn main() {

    let a: i8  = 6;
    let b: i8 = 2;

    println!("Addition: {}", add_two_numbers(a, b));
    println!("Subtraction: {}", sub_two_numbers(a, b));
    println!("Multiply: {}", multiply_two_numbers(a, b));
    println!("Division: {}", divide_two_numbers(a, b));
}
