

fn main() {

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
    fn divide(a: i32, b: i32) -> f32 {
        a as f32 / b as f32
    }

    let a = 20;
    let b = 77;

    println!("{} + {} = {}", a, b, add(a, b));
    println!("{} - {} = {}", a, b, subtract(a, b));
    println!("{} * {} = {}", a, b, multiply(a, b));
    println!("{} / {} = {}", a, b, divide(a, b));

}
