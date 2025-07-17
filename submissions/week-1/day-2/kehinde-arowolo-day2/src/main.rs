fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn divide(x: i32, y: i32) -> i32 {
    x / y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn add_float(x: f32, y: f32) -> f32 {
    x + y
}

fn main() {
    let first_num = 5;
    let second_num = 7;

    let float_one: f32 = 5.0;
    let float_two: f32= 5.0;

    println!("The sum of the two numbers are {}", add(first_num, second_num));
    println!("The subtraction of the two numbers are {}", subtract(first_num, second_num));
    println!("The division of the two numbers are {}", divide(first_num, second_num));
    println!("The multiplication of the two numbers are {}", multiply(first_num, second_num));
    println!("The addition of floats  are {}", add_float(float_one, float_two));

    
}