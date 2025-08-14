fn add(num1: f32, num2: f32) -> f32 {
    num1 + num2
}

fn sub(num1: f32, num2: f32) -> f32 {
    num1 - num2
}

fn mul(num1: f32, num2: f32) -> f32 {
    num1 * num2
}

fn div(num1: f32, num2: f32) -> f32 {
    num1 / num2
}

fn main() {
    println!("Hello, world!");

    let num1 = 7.0;
    let num2 = 5.0;

    println!("num 1 + num2 is {}", add(num1, num2));
    println!("num 1 - num2 is {}", sub(num1, num2));
    println!("num 1 * num2 is {}", mul(num1, num2));
    println!("num 1 / num2 is {}", div(num1, num2));

}
