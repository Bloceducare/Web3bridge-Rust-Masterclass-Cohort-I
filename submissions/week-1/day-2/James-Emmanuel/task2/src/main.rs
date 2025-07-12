// main.rs

fn main() {
    println!("The sum of 3 and 4 is: {}", add(3, 4));
    println!("The difference of 9 and 3 is: {}", sub(9, 3));
    println!("The multiplication of 3 and 4 is: {}", mul(3, 4));
    println!("The division of 26 and 3 is: {}", div(26, 3));
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: u32, b: u32) -> u32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> f32 {
    a as f32 / b as f32
}
