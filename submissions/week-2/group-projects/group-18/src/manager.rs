use crate::input::get_user_input;
use crate::models::{Feedback, FeedbackError};
use crate::persistence;
use chrono::Utc;
use std::collections::HashMap;

pub struct FeedbackManager {
    feedbacks: HashMap<u32, Feedback>,
    next_id: u32,
}

impl FeedbackManager {
    pub fn new() -> Self {
        // Try to load existing data from file
        match persistence::load_from_file() {
            Ok((feedbacks, next_id)) => {
                println!("Loaded {} existing feedback entries.", feedbacks.len());
                Self { feedbacks, next_id }
            }
            Err(e) => {
                eprintln!("Warning: Could not load existing data: {:?}", e);
                eprintln!("Starting with empty feedback database.");
                Self {
                    feedbacks: HashMap::new(),
                    next_id: 1,
                }
            }
        }
    }

    pub fn add_feedback(
        &mut self,
        customer_name: String,
        comment: String,
        rating: u8,
    ) -> Result<(), FeedbackError> {
        if rating < 1 || rating > 5 {
            return Err(FeedbackError::InvalidRating);
        }

        let feedback = Feedback {
            id: self.next_id,
            customer_name,
            comment,
            rating,
            date_created: Utc::now(),
        };

        self.feedbacks.insert(self.next_id, feedback);
        self.next_id += 1;

        // Auto-save after adding
        persistence::auto_save(&self.feedbacks, self.next_id);
        Ok(())
    }

    pub fn view_all_feedback(&self) {
        if self.feedbacks.is_empty() {
            println!("No feedback entries found.");
            return;
        }

        println!("\n=== ALL FEEDBACK ENTRIES ===");
        for feedback in self.feedbacks.values() {
            println!("ID: {}", feedback.id);
            println!("Customer: {}", feedback.customer_name);
            println!("Rating: {}/5 stars", feedback.rating);
            println!("Comment: {}", feedback.comment);
            println!(
                "Date: {}",
                feedback.date_created.format("%Y-%m-%d %H:%M:%S UTC")
            );
            println!("{}", "-".repeat(40));
        }
    }

    pub fn remove_feedback(&mut self, id: u32) -> Result<(), FeedbackError> {
        match self.feedbacks.remove(&id) {
            Some(feedback) => {
                println!("Removed feedback from customer: {}", feedback.customer_name);
                // Auto-save after removing
                persistence::auto_save(&self.feedbacks, self.next_id);
                Ok(())
            }
            None => Err(FeedbackError::FeedbackNotFound),
        }
    }

    pub fn edit_feedback(&mut self, id: u32) -> Result<(), FeedbackError> {
        let feedback = self
            .feedbacks
            .get(&id)
            .ok_or(FeedbackError::FeedbackNotFound)?;

        println!("\nCurrent feedback details:");
        println!("Customer: {}", feedback.customer_name);
        println!("Rating: {}/5", feedback.rating);
        println!("Comment: {}", feedback.comment);

        println!("\nWhat would you like to edit?");
        println!("1. Customer name");
        println!("2. Rating");
        println!("3. Comment");
        println!("4. Cancel");

        let choice = get_user_input("Enter your choice (1-4): ")?;
        let choice: u8 = choice
            .trim()
            .parse()
            .map_err(|_| FeedbackError::InvalidInput)?;

        let mut updated_feedback = self.feedbacks[&id].clone();

        match choice {
            1 => {
                let prompt = format!(
                    "Enter new customer name [{}]: ",
                    updated_feedback.customer_name
                );
                let new_name = get_user_input(&prompt)?;
                if !new_name.trim().is_empty() {
                    updated_feedback.customer_name = new_name.trim().to_string();
                }
            }
            2 => {
                let prompt = format!("Enter new rating (1-5) [{}]: ", updated_feedback.rating);
                let new_rating = get_user_input(&prompt)?;
                if !new_rating.trim().is_empty() {
                    let rating: u8 = new_rating
                        .trim()
                        .parse()
                        .map_err(|_| FeedbackError::InvalidInput)?;
                    if rating < 1 || rating > 5 {
                        return Err(FeedbackError::InvalidRating);
                    }
                    updated_feedback.rating = rating;
                }
            }
            3 => {
                let prompt = format!("Enter new comment [{}]: ", updated_feedback.comment);
                let new_comment = get_user_input(&prompt)?;
                if !new_comment.trim().is_empty() {
                    updated_feedback.comment = new_comment.trim().to_string();
                }
            }
            4 => {
                println!("Edit cancelled.");
                return Ok(());
            }
            _ => return Err(FeedbackError::InvalidInput),
        }

        // Confirmation step
        let confirm = get_user_input("Save changes? (y/n): ")?;
        if confirm.trim().to_lowercase() == "y" || confirm.trim().to_lowercase() == "yes" {
            self.feedbacks.insert(id, updated_feedback);
            println!("Feedback updated successfully!");
            persistence::auto_save(&self.feedbacks, self.next_id);
        } else {
            println!("Changes cancelled.");
        }

        Ok(())
    }

    pub fn list_feedback_ids(&self) {
        if self.feedbacks.is_empty() {
            println!("No feedback entries available.");
            return;
        }

        println!("\nAvailable feedback IDs:");
        for (id, feedback) in &self.feedbacks {
            println!("ID: {} - Customer: {}", id, feedback.customer_name);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.feedbacks.is_empty()
    }

    pub fn save_data(&self) -> Result<(), FeedbackError> {
        persistence::save_to_file(&self.feedbacks, self.next_id)
    }
}
