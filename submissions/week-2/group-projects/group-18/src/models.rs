use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub id: u32,
    pub customer_name: String,
    pub comment: String,
    pub rating: u8,
    pub date_created: DateTime<Utc>,
}

#[derive(Debug)]
pub enum FeedbackError {
    InvalidRating,
    FeedbackNotFound,
    InvalidInput,
    PersistenceError(String),
}
