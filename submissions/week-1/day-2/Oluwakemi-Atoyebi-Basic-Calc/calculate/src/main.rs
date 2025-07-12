fn main() {
    let s = 5;
    let t  = 10;

    add_num(s, t);
    sub_num(s, t);
    div_num(s , t);
    mul_num(s, t);
}

fn add_num(x: u8, y: u8) {
    let sum = x + y;
    println!("sum is {sum}");
}

fn sub_num(a: u8, b: u8) {
    let deduct = a as i8  - b as i8;
    println!("Subtraction is {deduct}");
}

fn mul_num(x: u8, y: u8) {
    let product = x * y;
    println!("Product is {product}");
}

fn div_num(x: u8, y: u8) {
    let total = x as f32 / y as f32;
    println!("Division is {total}");
}