use chrono::NaiveDate;
use std::collections::HashMap;
use std::io;

#[derive(Clone)]
struct Donation {
    donor_name: String,
    amount: f64,
    date: NaiveDate,
}

struct DonationTracker {
    donations: HashMap<String, Vec<Donation>>,
}

impl DonationTracker {
    fn new() -> Self {
        Self { donations: HashMap::new() }
    }

    fn add_donation(&mut self, donor_name: &str, amount: f64, date: NaiveDate) {
        let donation = Donation {
            donor_name: donor_name.to_string(),
            amount,
            date,
        };

        self.donations.entry(donor_name.to_string()).or_default().push(donation);
        //     self.donations.entry(donor_name.to_string()).or_insert_with(Vec::new).push(donation);
    }

    fn view_all_donations(&self) {
        for donation in self.donations.values() {
            for d in donation {
                println!("Donor: {}, Amount: {}, Date: {}", d.donor_name, d.amount, d.date);
            }
        }
    }

    fn remove_donation(&mut self, donor_name: &str) {
        if self.donations.remove(donor_name).is_some() {
            println!("Donor: {} removed successfully!", donor_name);
        } else {
            println!("Donor: {} not Found!", donor_name)
        }
    }

    fn edit_donation(
        &mut self,
        donor_name: &str,
        new_donor_name: &str,
        new_amount: f64,
        date: NaiveDate
    ) {
        if let Some(mut donates) = self.donations.remove(donor_name) {
            // Update the Struct with new values
            if let Some(donation) = donates.first_mut() {
                donation.donor_name = new_donor_name.to_owned();
                donation.amount = new_amount;
                donation.date = date;

                // If the name has changed, move to a new key
                if new_donor_name != donor_name {
                    // Remove the donation from the current list
                    let updated_donation = donation.clone();
                    donates.remove(0);

                    // If the original list is now empty, remove the key
                    if donates.is_empty() {
                        self.donations.remove(donor_name);
                    }

                    // Insert the updated donation under the new name
                    self.donations
                        .entry(new_donor_name.to_owned())
                        .or_insert_with(Vec::new)
                        .push(updated_donation);
                }

                println!("Donation '{}' updated successfully.", new_donor_name);
            } else {
                println!("No donates found in the list for  '{}'.", donor_name);
                // Put it back to avoid data loss
                self.donations.insert(donor_name.to_owned(), donates);
            }
        } else {
            println!("No donation found for '{}'.", donor_name);
        }
    }

    fn exit_program(&self) {
        println!("Exiting the program. Thank you for using the Donation Tracker!");
    }
}
fn input_field() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn input_date() -> NaiveDate {
    println!("Enter the date (YYYY-MM-DD):");
    let date_str = input_field();

    match NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid date format. Please use YYYY-MM-DD.");
            // Return a default date or handle the error as needed
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap() // Default date
        }
    }
}

fn main() {
    println!("Welcome to the Donation Tracker!");
    println!("You can add, view, remove, and edit Donation.");

    println!("Menu");
    println!("---------------");
    println!("1. Add donations");
    println!("2. View all donations");
    println!("3. Remove donations");
    println!("4. Edit donations");
    println!("5. Exits the program");
    println!("Please enter your choice (1-5):");

    let mut tracker = DonationTracker::new();

    // let mut choice = String::new();
    // std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    // let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    loop {
        let choice = input_field();

        match choice.as_str() {
            "1" => {
                println!("Enter donor name:");
                let donor_name = input_field();
                println!("Enter donation amount:");
                let amount: f64 = input_field().parse().expect("Please enter a valid number");
                let date = input_date();
                if date == NaiveDate::from_ymd_opt(1970, 1, 1).unwrap() {
                    println!("Invalid date provided. Please try again.");
                    continue;
                }

                tracker.add_donation(&donor_name, amount, date);
                println!("Donation added successfully!");
            }
            "2" => {
                println!("View all donations");
                if tracker.donations.is_empty() {
                    println!("No donations Available.");
                } else {
                    tracker.view_all_donations();
                }
            }
            "3" => {
                println!("Remove donations using the donor name");
                let donor_name = input_field();
                tracker.remove_donation(&donor_name);
            }
            "4" => {
                println!("Enter the donor name you want to edit:");
                let donor_name = input_field();
                println!("Enter the new donor name:");
                let new_donor_name = input_field();
                println!("Enter the new donation amount:");
                let new_amount: f64 = input_field().parse().expect("Please enter a valid number");
                let date = input_date();
                if date == NaiveDate::from_ymd_opt(1970, 1, 1).unwrap() {
                    println!("Invalid date provided. Please try again.");
                    continue;
                }
                tracker.edit_donation(&donor_name, &new_donor_name, new_amount, date);
            }
            "5" => {
                tracker.exit_program();
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 5.");
            }
        }
    }
}
