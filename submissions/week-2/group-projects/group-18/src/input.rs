use std::io::{self, Write};
use crate::models::FeedbackError;

pub fn get_user_input(prompt: &str) -> Result<String, FeedbackError> {
    print!("{}", prompt);
    io::stdout().flush().map_err(|_| FeedbackError::InvalidInput)?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| FeedbackError::InvalidInput)?;
    Ok(input)
}
