fn add(num1: u64, num2: u64) -> u64 {
    num1 + num2
}

fn sub(num1: u64, num2: u64) -> u64 {
    num1 - num2
}

fn mul(num1: u64, num2: u64) -> u64 {
    num1 * num2
}

fn div(num1: f32, num2: f32) -> f32 {
    num1 / num2
}

fn main() {
    let num1: u64 = 10;
    let num2: u64 = 5;

    println!("num 1 + num2 is {}", add(num1, num2));
    println!("num 1 - num2 is {}", sub(num1, num2));
    println!("num 1 * num2 is {}", mul(num1, num2));
    println!("num 1 / num2 is {}", div(num1 as f32, num2 as f32));

}