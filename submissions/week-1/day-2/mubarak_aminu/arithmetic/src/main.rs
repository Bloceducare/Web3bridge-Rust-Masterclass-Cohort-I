
fn add(a: u32, b: u32) -> u32 {
        a + b
}

fn subtract(a:u32, b: u32) -> u32 {
    a - b
}

fn multiple(x: u32, y: u32) -> u32 {
    x * y
}

fn divide(a: u32, b: u32) -> u32 {
    a / b
}



fn main() {
    
    let addition = add(60, 40);
    let subtract = subtract(200, 120);
    let multiple = multiple(12, 6);
    let divide = divide(24, 6);

    println!("addition result is: {}", addition);
    println!("subtraction result is: {}", subtract);
    println!("multiplction result is: {}", multiple);
    println!("division result is: {}", divide);
}
