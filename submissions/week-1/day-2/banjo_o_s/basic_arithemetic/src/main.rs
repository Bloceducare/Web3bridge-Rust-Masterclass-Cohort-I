fn main() {
    println!("Hello, world!");

    let first_number = 10;
    let second_number = 5;

    let addition_result = addition(first_number, second_number);
    let subtraction_result = subtraction(first_number, second_number);
    let multiplication_result = multiplication(first_number, second_number);
    let division_result = division(first_number, second_number);

    println!("addition reslt of {} and {} is: {}", first_number, second_number, addition_result);
    println!("subtraction reslt of {} and {} is: {}", first_number, second_number, subtraction_result);
    println!("multiplication reslt of {} and {} is: {}", first_number, second_number, multiplication_result);
    println!("division reslt of {} and {} is: {}", first_number, second_number, division_result);
}

fn addition(first: i32, second: i32) -> i32 {
    first + second
}

fn subtraction(first: i32, second: i32) -> i32 {
    first - second
}

fn multiplication(first: i32, second: i32) -> i32 {
    first * second
}

fn division(first: i32, second: i32) -> f32 {
    first as f32 / second as f32
}