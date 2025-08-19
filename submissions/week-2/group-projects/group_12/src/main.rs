use group_12::{AllCampaigns, CampaignStatus};
use std::io;

fn main() {
    println!("---------------------------");
    println!("Campaign Management System");
    println!("---------------------------");

    let mut all_campaigns = AllCampaigns::new();

    loop {
        println!("Enter a command: \n   1. add-campaign \n   2. update-campaign-status \n   3. get-campaign \n   4. get-all-campaign \n   5. remove-completed-campaign \n   6. update-campaign-budget \n   7. cancel-edit-campaign \n   8. edit-campaign \n   9. exit");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let command = user_input.trim();

        match command {
            "add-campaign" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim().to_string();

                println!("Enter campaign budget:");
                let mut budget = String::new();
                io::stdin()
                    .read_line(&mut budget)
                    .expect("Failed to read line");
                let budget: f64 = budget.trim().parse().expect("Invalid budget");

                println!("Enter start date (YYYY-MM-DD):");
                let mut start_date = String::new();
                io::stdin()
                    .read_line(&mut start_date)
                    .expect("Failed to read line");
                let start_date = start_date.trim().to_string();

                println!("Enter end date (YYYY-MM-DD):");
                let mut end_date = String::new();
                io::stdin()
                    .read_line(&mut end_date)
                    .expect("Failed to read line");
                let end_date = end_date.trim().to_string();

                all_campaigns.add_campaign(name, budget, start_date, end_date);
                println!("----- Campaign added successfully -----");
            }
            "update-campaign-status" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                println!("Enter new status (Active, Inactive, or Completed):");
                let mut status_input = String::new();
                io::stdin()
                    .read_line(&mut status_input)
                    .expect("Failed to read line");

                let status = match status_input.trim().to_lowercase().as_str() {
                    "active" => CampaignStatus::Active,
                    "inactive" => CampaignStatus::Inactive,
                    "completed" => CampaignStatus::Completed,
                    _ => {
                        println!("Invalid status");
                        continue;
                    }
                };

                match all_campaigns.update_campaign_status(name, status) {
                    Err(e) => println!("Error: {}", e),
                    Ok(_) => println!("----- Campaign status updated successfully -----"),
                }
            }
            "get-campaign" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                match all_campaigns.get_campaign(name) {
                    Some(campaign) => {
                        println!("Campaign found: {:#?}", campaign);
                    }
                    None => {
                        println!("No campaign found with the name: {}", name);
                    }
                }
            }
            "get-all-campaign" => {
                let campaigns = all_campaigns.get_all_campaign();
                if campaigns.is_empty() {
                    println!("No campaigns available.");
                } else {
                    println!("All campaigns:");
                    for campaign in campaigns {
                        println!("{:#?}", campaign);
                    }
                }
            }
            "remove-completed-campaign" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                match all_campaigns.remove_completed_campaign(name) {
                    Ok(_) => println!("----- Campaign removed successfully -----"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "update-campaign-budget" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                println!("Enter campaign budget:");
                let mut budget = String::new();
                io::stdin()
                    .read_line(&mut budget)
                    .expect("Failed to read line");
                let budget: f64 = budget.trim().parse().expect("Invalid budget");

                match all_campaigns.update_campaign_budget(name, budget) {
                    Ok(_) => println!("----- Campaign budget updated successfully -----"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "cancel-edit-campaign" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                match all_campaigns.cancel_edit_campaign(name) {
                    Ok(_) => println!("----- Campaign edit cancelled successfully -----"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "edit-campaign" => {
                println!("Enter campaign name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");
                let name = name.trim();

                println!("Enter new campaign name (or press Enter to skip):");
                let mut new_name = String::new();
                io::stdin()
                    .read_line(&mut new_name)
                    .expect("Failed to read line");
                let new_name = if new_name.trim().is_empty() {
                    None
                } else {
                    Some(new_name.trim().to_string())
                };

                println!("Enter new budget (or press Enter to skip):");
                let mut new_budget = String::new();
                io::stdin()
                    .read_line(&mut new_budget)
                    .expect("Failed to read line");
                let new_budget = if new_budget.trim().is_empty() {
                    None
                } else {
                    Some(new_budget.trim().parse().expect("Invalid budget"))
                };

                println!("Enter new start date (YYYY-MM-DD or press Enter to skip):");
                let mut new_start_date = String::new();
                io::stdin()
                    .read_line(&mut new_start_date)
                    .expect("Failed to read line");
                let new_start_date = if new_start_date.trim().is_empty() {
                    None
                } else {
                    Some(new_start_date.trim().to_string())
                };

                println!("Enter new end date (YYYY-MM-DD or press Enter to skip):");
                let mut new_end_date = String::new();
                io::stdin()
                    .read_line(&mut new_end_date)
                    .expect("Failed to read line");
                let new_end_date = if new_end_date.trim().is_empty() {
                    None
                } else {
                    Some(new_end_date.trim().to_string())
                };

                match all_campaigns.edit_campaign(
                    name,
                    new_name,
                    new_budget,
                    new_start_date,
                    new_end_date,
                ) {
                    Ok(_) => println!("----- Campaign edited successfully -----"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "exit" => break,
            _ => println!("Invalid command"),
        }
    }
}
