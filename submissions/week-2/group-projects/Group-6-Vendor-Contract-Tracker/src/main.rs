/**
 *
 *
# 24 Management-Related Rust Projects for Students

These are the projects for the 24 Groups we have, if you don't have a group send a message on discord and I'll add you an existing group,read instructions carefully, these projects builds on what we've learnt in the past few weeks. Each project should and must be designed as a command-line interactive application. The projects progress through three stages: Stage 1 (add and view), Stage 2 (remove), and Stage 3 (edit and cancel). To complete this project you must use features like the Vec, hashmap, enums and Option think about some of these things and use separate functions for each menu options.

### General Implementation Notes

- Structure: Use a `loop` for an interactive menu with options like `1. Add`, `2. View`, `3. Remove` (Stage 2), and `4. Edit` (Stage 3). Implement each option as a separate function for modularity.
- Data Storage: Start with a `Vec<Struct>` in Stage 1 to make your lives easier. Then in Stages 2 and 3, transition to a `HashMap` for efficient lookups, using a unique identifier as the key.
- Rust Libraries: Use `std::io` for input/output, `serde` for potential serialization, and `chrono` for date handling where relevant (e.g., contracts, bookings).
- Error Handling: Use Rust’s `Result` and `Option` types to handle invalid inputs and missing data.
- Cancel Functionality: In Stage 3, implement a confirmation step for edits (e.g., “Save changes? (y/n)”) to allow canceling.

You are not limited to these instructions, do as occasion serves you in your respective projects, This is just to somehow aid your ideation process and help you gain speed.

Enjoy!

---

### Group 6: Vendor Contract Tracker

- Description: Manage vendor contracts.
- Stage 1:
  - Add contracts (vendor name, contract value, end date).
  - View all contracts.
- Stage 2:
  - Remove expired or canceled contracts.
- Stage 3:
  - Edit contract details.
  - Cancel edits.
- Implementation Tips: Use a `Vec` in Stage 1, switch to a `HashMap` with vendor name or contract ID as the key.
 */

 
use chrono::{Local, NaiveDate};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContractStatus {
    Active,
    Expired,
    Canceled,
}

#[derive(Debug, Clone)]
pub struct VendorContract {
    id: u32,
    vendor_name: String,
    contract_value: u64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    status: ContractStatus,
}

impl VendorContract {
    fn check_status(
        id: u32,
        vendor_name: String,
        contract_value: u64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        let today = Local::now().naive_local().date();
        let status = if end_date < today {
            ContractStatus::Expired
        } else {
            ContractStatus::Active
        };

        VendorContract {
            id,
            vendor_name,
            contract_value,
            start_date,
            end_date,
            status,
        }
    }

    fn view_all(contracts: &HashMap<u32, VendorContract>) {
        for contract in contracts.values() {
            println!(
                "\nID: {}\nVendor: {}\nValue: {}\nStart: {}\nEnd: {}\nStatus: {:?}\n",
                contract.id,
                contract.vendor_name,
                contract.contract_value,
                contract.start_date,
                contract.end_date,
                contract.status
            );
        }
    }

    fn delete_expired_or_canceled(contracts: &mut HashMap<u32, VendorContract>) {
        let to_remove: Vec<u32> = contracts
            .iter()
            .filter_map(|(&id, contract)| {
                if contract.status == ContractStatus::Expired
                    || contract.status == ContractStatus::Canceled
                {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();

        for id in to_remove {
            contracts.remove(&id);
        }

        println!("✅ Expired or canceled contracts have been deleted.");
    }

    fn update_contract(
        contracts: &mut HashMap<u32, VendorContract>,
        id: u32,
        new_vendor_name: Option<String>,
        new_contract_value: Option<u64>,
        new_start_date: Option<NaiveDate>,
        new_end_date: Option<NaiveDate>,
    ) {
        if let Some(contract) = contracts.get_mut(&id) {
            if let Some(name) = new_vendor_name {
                contract.vendor_name = name;
            }

            if let Some(value) = new_contract_value {
                contract.contract_value = value;
            }

            if let Some(start) = new_start_date {
                contract.start_date = start;
            }

            if let Some(end) = new_end_date {
                contract.end_date = end;

                let today = Local::now().naive_local().date();
                contract.status = if end < today {
                    ContractStatus::Expired
                } else {
                    ContractStatus::Active
                };
            }

            println!("✅ Contract updated successfully.");
        } else {
            println!("❌ No contract found with ID {}", id);
        }
    }

    fn cancel_contract_by_id(contracts: &mut HashMap<u32, VendorContract>, id: u32) {
        if let Some(contract) = contracts.get_mut(&id) {
            contract.status = ContractStatus::Canceled;
            println!("✅ Contract with ID {} has been canceled.", id);
        } else {
            println!("❌ No contract found with ID {}", id);
        }
    }
}

// ====== Input Helpers ======
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_u32(prompt: &str) -> u32 {
    loop {
        let input = get_input(prompt);
        if let Ok(num) = input.parse::<u32>() {
            return num;
        }
        println!("Please enter a valid number.");
    }
}

fn parse_u64(prompt: &str) -> u64 {
    loop {
        let input = get_input(prompt);
        if let Ok(num) = input.parse::<u64>() {
            return num;
        }
        println!("Please enter a valid number.");
    }
}

fn parse_date(prompt: &str) -> NaiveDate {
    loop {
        let input = get_input(prompt);
        if let Ok(date) = NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            return date;
        }
        println!("Enter date in format YYYY-MM-DD.");
    }
}

fn main() {
    let mut contract_map: HashMap<u32, VendorContract> = HashMap::new();

    loop {
        println!("\n===== Vendor Contract Tracker =====");
        println!("1. Add Contract");
        println!("2. View All Contracts");
        println!("3. Delete Expired or Canceled Contracts");
        println!("4. Update Contract");
        println!("5. Cancel a Contract by ID");
        println!("6. Exit");
        println!("===================================");

        let choice = get_input("Enter your choice: ");

        match choice.as_str() {
            "1" => {
                let id = parse_u32("Enter contract ID: ");
                let name = get_input("Enter vendor name: ");
                let value = parse_u64("Enter contract value: ");
                let start = parse_date("Enter start date (YYYY-MM-DD): ");
                let end = parse_date("Enter end date (YYYY-MM-DD): ");

                let new_contract = VendorContract::check_status(id, name, value, start, end);
                contract_map.insert(id, new_contract);

                println!("✅ Contract added.");
            }

            "2" => {
                if contract_map.is_empty() {
                    println!("No contracts found.");
                } else {
                    VendorContract::view_all(&contract_map);
                }
            }

            "3" => {
                VendorContract::delete_expired_or_canceled(&mut contract_map);
            }

            "4" => {
                let id = parse_u32("Enter ID of contract to update: ");

                println!("Leave input empty to keep current value.");
                let name_input = get_input("New vendor name: ");
                let name = if name_input.is_empty() {
                    None
                } else {
                    Some(name_input)
                };

                let value_input = get_input("New contract value: ");
                let value = if value_input.is_empty() {
                    None
                } else {
                    value_input.parse::<u64>().ok()
                };

                let start_input = get_input("New start date (YYYY-MM-DD): ");
                let start = if start_input.is_empty() {
                    None
                } else {
                    NaiveDate::parse_from_str(&start_input, "%Y-%m-%d").ok()
                };

                let end_input = get_input("New end date (YYYY-MM-DD): ");
                let end = if end_input.is_empty() {
                    None
                } else {
                    NaiveDate::parse_from_str(&end_input, "%Y-%m-%d").ok()
                };

                VendorContract::update_contract(&mut contract_map, id, name, value, start, end);
            }

            "5" => {
                let id = parse_u32("Enter ID of contract to cancel: ");
                VendorContract::cancel_contract_by_id(&mut contract_map, id);
            }

            "6" => {
                println!("👋 Exiting.");
                break;
            }

            _ => println!("Invalid choice. Try again."),
        }
    }
}

