use rand::Rng;

fn main() {
    let a: f32 = rand::thread_rng().gen_range(1.0..100.0);
    let b: f32 = rand::thread_rng().gen_range(1.0..100.0);  
    println!("This is the value for a: {a}, and b: {b}");
    add(a, b);
    subtract(a, b);
    multiply(a, b);
    divide(a, b);
}

fn add(a:f32, b:f32) {
    println!("Adding a and b = {}", a + b);
}

fn subtract(a:f32, b:f32) {
    println!("Subtracting b from a = {}", a - b);
}

fn multiply(a:f32, b:f32) {
    println!("Multiplying a and b = {}", a * b);
}

fn divide(a:f32, b:f32) {
    println!("Dividing a by b = {}", a / b);
}
