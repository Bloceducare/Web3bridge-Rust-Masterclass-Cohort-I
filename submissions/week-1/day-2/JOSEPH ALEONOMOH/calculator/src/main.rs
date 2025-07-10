fn main() {
    let add = add(5,5);
    println!("ADDITION: {add}");

    let sub = sub(7,5);
    println!("SUBTRACTION: {sub}");

    let mul = mul(5,5);
    println!("MULTIPLY: {mul}");

    let div = div(10,5);
    println!("DIVISION: {div}");
}

fn add(a:u32, b:u32) -> u32 {
    a + b
}

fn sub(a:u32, b:u32) -> u32 {
    a - b
}

fn mul(a:u32, b:u32) -> u32 {
    a * b
}

fn div(a:u32, b:u32) -> u32 {
    a / b
}
