use std::u32;

pub fn mul(a: u32, b: u32) -> u32 {
    return a * b;
}

pub fn div(a: u32, b: u32) -> u32 {
    return a / b;
}

pub fn add(a: u32, b: u32) -> u32 {
    return a + b;
}

pub fn sub(a: u32, b: u32) -> u32 {
    return a - b;
}

fn main() {
    let a = 6;
    let b = 2;
    let c = mul(a, b);
    let d = div(a, b);
    let e = add(a, b);
    let f = sub(a, b);

    print!("Mul Answer is {a}, {b} = {c}\n");
    print!("Div answer is{a}, {b} = {d}\n");
    print!("Add answer is{a}, {b} = {e}\n");
    print!("Sub answer is{a}, {b} = {f}\n");
}
