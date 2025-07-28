use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Feedback {
    id: u32,
    customer_name: String,
    comment: String,
    rating: u8,
}

enum MenuOption {
    Add,
    View,
    Remove,
    Edit,
    Exit,
}

impl MenuOption {
    fn from_input(input: &str) -> Option<Self> {
        match input.trim() {
            "1" => Some(MenuOption::Add),
            "2" => Some(MenuOption::View),
            "3" => Some(MenuOption::Remove),
            "4" => Some(MenuOption::Edit),
            "5" => Some(MenuOption::Exit),
            _ => None,
        }
    }
}

struct FeedbackLogger {
    feedback_map: HashMap<u32, Feedback>,
    next_id: u32,
}

impl FeedbackLogger {
    fn new() -> Self {
        FeedbackLogger {
            feedback_map: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_feedback(&mut self) -> io::Result<()> {
        println!("Enter customer name:");
        let customer_name = read_input()?.trim().to_string();

        println!("Enter comment:");
        let comment = read_input()?.trim().to_string();

        println!("Enter rating (1-5):");
        let rating_str = read_input()?.trim().to_string();
        let rating = rating_str.parse::<u8>().map_err(|_e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "Rating must be a number between 1 and 5",
            )
        })?;

        if !(1..=5).contains(&rating) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Rating must be between 1 and 5",
            ));
        }

        let feedback = Feedback {
            id: self.next_id,
            customer_name,
            comment,
            rating,
        };

        self.feedback_map.insert(self.next_id, feedback);
        self.next_id += 1;
        println!("Feedback added successfully!");
        Ok(())
    }

    fn view_feedback(&self) -> io::Result<()> {
        if self.feedback_map.is_empty() {
            println!("No feedback entries found.");
            return Ok(());
        }

        for feedback in self.feedback_map.values() {
            println!(
                "ID: {}\nName: {}\nComment: {}\nRating: {}\n",
                feedback.id, feedback.customer_name, feedback.comment, feedback.rating
            );
        }
        Ok(())
    }

    fn remove_feedback(&mut self) -> io::Result<()> {
        println!("Enter feedback ID to remove:");
        let id_str = read_input()?.trim().to_string();
        let id = id_str
            .parse::<u32>()
            .map_err(|_e| io::Error::new(io::ErrorKind::InvalidInput, "ID must be a number"))?;

        match self.feedback_map.remove(&id) {
            Some(_) => println!("Feedback with ID {} removed successfully!", id),
            None => println!("No feedback found with ID {}", id),
        }
        Ok(())
    }

    fn edit_feedback(&mut self) -> io::Result<()> {
        println!("Enter feedback ID to edit:");
        let id_str = read_input()?.trim().to_string();
        let id = id_str
            .parse::<u32>()
            .map_err(|_e| io::Error::new(io::ErrorKind::InvalidInput, "ID must be a number"))?;

        if let Some(feedback) = self.feedback_map.get_mut(&id) {
            println!("Current feedback details:");
            println!(
                "Name: {}\nComment: {}\nRating: {}",
                feedback.customer_name, feedback.comment, feedback.rating
            );

            println!("Enter new customer name (press Enter to keep current):");
            let new_name = read_input()?.trim().to_string();
            let customer_name = if new_name.is_empty() {
                feedback.customer_name.clone()
            } else {
                new_name
            };

            println!("Enter new comment (press Enter to keep current):");
            let new_comment = read_input()?.trim().to_string();
            let comment = if new_comment.is_empty() {
                feedback.comment.clone()
            } else {
                new_comment
            };

            println!("Enter new rating (1-5, press Enter to keep current):");
            let new_rating = read_input()?.trim().to_string();
            let rating = if new_rating.is_empty() {
                feedback.rating
            } else {
                let rating = new_rating.parse::<u8>().map_err(|_e| {
                    io::Error::new(io::ErrorKind::InvalidInput, "Rating must be a number")
                })?;
                if !(1..=5).contains(&rating) {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Rating must be between 1 and 5",
                    ));
                }
                rating
            };

            println!("Save changes? (y/n)");
            let confirm = read_input()?.trim().to_lowercase();
            if confirm == "y" {
                feedback.customer_name = customer_name;
                feedback.comment = comment;
                feedback.rating = rating;
                println!("Feedback updated successfully!");
            } else {
                println!("Changes discarded.");
            }
        } else {
            println!("No feedback found with ID {}", id);
        }
        Ok(())
    }
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn display_menu() {
    println!("\nCustomer Feedback Logger");
    println!("1. Add Feedback");
    println!("2. View All Feedback");
    println!("3. Remove Feedback");
    println!("4. Edit Feedback");
    println!("5. Exit");
    print!("Select an option: ");
    io::stdout().flush().unwrap();
}

fn main() -> io::Result<()> {
    let mut logger = FeedbackLogger::new();

    loop {
        display_menu();
        let input = read_input()?;

        match MenuOption::from_input(&input) {
            Some(MenuOption::Add) => {
                if let Err(e) = logger.add_feedback() {
                    println!("Error: {}", e);
                }
            }
            Some(MenuOption::View) => {
                if let Err(e) = logger.view_feedback() {
                    println!("Error: {}", e);
                }
            }
            Some(MenuOption::Remove) => {
                if let Err(e) = logger.remove_feedback() {
                    println!("Error: {}", e);
                }
            }
            Some(MenuOption::Edit) => {
                if let Err(e) = logger.edit_feedback() {
                    println!("Error: {}", e);
                }
            }
            Some(MenuOption::Exit) => {
                println!("Exiting...");
                break;
            }
            None => println!("Invalid option. Please try again."),
        }
    }
    Ok(())
}
