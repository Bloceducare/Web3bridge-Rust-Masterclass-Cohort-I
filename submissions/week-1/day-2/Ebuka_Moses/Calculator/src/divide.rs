pub fn divide(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("Divide with a Number, Not zero");
    }
    a / b
}