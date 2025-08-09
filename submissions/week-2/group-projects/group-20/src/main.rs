// use std::collections::HashMap;
use std::io::{self, Write};
use group_20::{CandidatesInfo, CandidateStatus};


fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input.trim().to_string()
}

fn main() {
    let mut candidate_system = CandidatesInfo::new();

    loop {
        println!("\n Welcome to Group 20 Hiring Candidate Tracking System");
        println!("These are the lists of options to choose from");
        println!("1. Add new Candidates to the system");
        println!("2. View all  Candidates in the System");
        println!("3. Remove  Candidates from the Hiring the system");
        println!("4. Edit Candidates in the System");
        println!("5. Cancel the edit of Candidates in the system");
        println!("6. Exit the Program Goodbye!");


        let choice = input("\n Enter your choice: ");


        match choice.as_str() {
            "1" => {
                let name = input("Enter candidate name: ");
                let position = input("Enter the candidate position: ");
                let contact = input("Enter the candidate contact: ");
                let id = candidate_system.add_candidate(name, contact, position, CandidateStatus::ACCEPTED);
                println!("Candidate added with the ID: {} has been added ", id);              
            }

            "2" => {
                println!("\n View all the Candidates:");
                for candidate in candidate_system.get_all_candidates() {
                    println!("[{}], {}, {}, {}, {:?}", candidate.id, candidate.name, candidate.position, candidate.contact, candidate.status);
                }
            }

            "3" => {
                let id_input = input("Enter the ID to edit");
                if let Ok(id) = id_input.parse::<u32>() {
                    let name = input("Enter new name: ");
                    let position = input("Enter new position: ");
                    let contact = input("Enter new contact: ");
                    if candidate_system.edit_candidates(id, name, position, contact) {
                        println!("Candidate updated");
                    } else {
                        println!("candidate with the ID {} not found.", id);
                    }
                } else {
                    println!("Invalid ID");
                }
            }

            "4"  => {
                let id_input = input("Enter ID to remove");
                if let Ok(id) = id_input.parse::<u32>() {
                    candidate_system.remove_candidates(id);
                        println!("candidate removed if existed");
                } else {
                        println!("Invalid ID");
                    }
                }

            "5"  => {
                let id_input = input("Enter ID to cancel edit");
                if let Ok(id) = id_input.parse::<u32>() {
                    candidate_system.cancel_edit(id); 
                        println!("candidate edited");
                } else {
                        println!("Invalid ID");
                    }
            }
            "6"   =>  {
                println!("Exiting the Program Goodbye!");
                break;
            }
            _ =>  {
                println!("Invalid Option. Please enter 1 -6.");
            }
        }
    }

}

