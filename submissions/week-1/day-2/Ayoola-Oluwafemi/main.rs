pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divivde by zero")
    }
    a / b
}

fn main() {
    let addition = add(5, 3);
    let subtraction = subtract(10, 4);
    let multiplication = multiply(2, 7);
    let division = divide(8.4, 2.0);

    println!("The sum of 5 and 3 is {}", addition);
    println!("The difference between 10 and 4 is {}", subtraction);
    println!("The product of 2 and 7 is {}", multiplication);
    println!("The division of 8.4 by 2.0 is {}", division);
}
