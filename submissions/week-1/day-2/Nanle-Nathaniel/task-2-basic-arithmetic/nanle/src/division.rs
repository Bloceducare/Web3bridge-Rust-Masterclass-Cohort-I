pub fn division(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}