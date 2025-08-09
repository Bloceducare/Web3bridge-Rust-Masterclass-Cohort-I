use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use crate::models::{Feedback, FeedbackError};
use serde_json;

const DATA_FILE: &str = "feedback_data.json";

#[derive(serde::Serialize, serde::Deserialize)]
struct FeedbackData {
    feedbacks: HashMap<u32, Feedback>,
    next_id: u32,
}

pub fn save_to_file(feedbacks: &HashMap<u32, Feedback>, next_id: u32) -> Result<(), FeedbackError> {
    let data = FeedbackData {
        feedbacks: feedbacks.clone(),
        next_id,
    };

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DATA_FILE)
        .map_err(|e| FeedbackError::PersistenceError(format!("Failed to create file: {}", e)))?;

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data)
        .map_err(|e| FeedbackError::PersistenceError(format!("Failed to write data: {}", e)))?;

    println!("Data saved successfully to {}", DATA_FILE);
    Ok(())
}

pub fn load_from_file() -> Result<(HashMap<u32, Feedback>, u32), FeedbackError> {
    if !Path::new(DATA_FILE).exists() {
        // If file doesn't exist, return empty data
        return Ok((HashMap::new(), 1));
    }

    let file = File::open(DATA_FILE)
        .map_err(|e| FeedbackError::PersistenceError(format!("Failed to open file: {}", e)))?;

    let reader = BufReader::new(file);
    let data: FeedbackData = serde_json::from_reader(reader)
        .map_err(|e| FeedbackError::PersistenceError(format!("Failed to parse data: {}", e)))?;

    println!("Data loaded successfully from {}", DATA_FILE);
    Ok((data.feedbacks, data.next_id))
}

pub fn auto_save(feedbacks: &HashMap<u32, Feedback>, next_id: u32) {
    if let Err(e) = save_to_file(feedbacks, next_id) {
        eprintln!("Warning: Failed to auto-save data: {:?}", e);
    }
}
