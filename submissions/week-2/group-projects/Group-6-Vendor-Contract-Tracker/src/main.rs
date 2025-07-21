use chrono::Local;
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

 
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ContractStatus {
    Active,
    Expired,
    Canceled,
}

#[derive(Debug, Clone)]
pub struct VendorContract {
    id: u8,
    vendor_name: String,
    contract_value: u64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    status: ContractStatus,
}

impl VendorContract {
    fn check_status(
        id: u8,
        vendor_name: String,
        contract_value: u64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        let today = Local::now().naive_local().date();

        let status = match end_date < today {
            true => ContractStatus::Expired,
            false => ContractStatus::Active,
        };

        // let status  = if end_date < today {
        //     ContractStatus::Expired

        // } else {
        //     ContractStatus::Active
        // };

        VendorContract {
            id,
            vendor_name,
            contract_value,
            start_date,
            end_date,
            status,
        }
    }

    fn add_contract(
        contract_data: &mut Vec<VendorContract>,
        id: u8,
        vendor_name: String,
        contract_value: u64,
        start_date: NaiveDate,
        end_date: NaiveDate,
        status: ContractStatus,
    ) {
        let new_contract = VendorContract {
            id,
            vendor_name: vendor_name.to_string(),
            contract_value,
            start_date,
            end_date,
            status,
        };

        contract_data.push(new_contract);
        println!("Contract with ID {} has been added.", id);
    }

    fn add_contract2(contract_data: &mut Vec<VendorContract>, new_contract: VendorContract) {
        println!("Contract with ID {} has been added.", new_contract.id);
        contract_data.push(new_contract);
    }

    fn view_allcontract(contract_data: &Vec<VendorContract>) {
        for contract in contract_data {
            print!(
                "\n {}, {}, {}, {}, {}, {:?} \n",
                contract.id,
                contract.vendor_name,
                contract.contract_value,
                contract.start_date,
                contract.end_date,
                contract.status
            );
        }
    }

    fn delete_expired_or_canceled(contract_data: &mut HashMap<u32, VendorContract>) {
        // let today = Local::now().naive_local().date();

        let to_remove: Vec<u32> = contract_data
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

        // Remove them
        for id in to_remove {
            contract_data.remove(&id);
        }
    }

    fn update_contract(
        contract_data: &mut HashMap<u32, VendorContract>,
        id: u32,
        new_vendor_name: Option<String>,
        new_contract_value: Option<u64>,
        new_start_date: Option<NaiveDate>,
        new_end_date: Option<NaiveDate>,
    ) {
        if let Some(contract) = contract_data.get_mut(&id) {
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

                // Re-check contract status after changing end date
                let today = Local::now().naive_local().date();
                contract.status = if end < today {
                    ContractStatus::Expired
                } else {
                    ContractStatus::Active
                };
            }

            println!("Contract with ID {} has been updated.", id);
        } else {
            println!("No contract found with ID {}.", id);
        }
    }
}

fn main() {
    let mut contract_db: Vec<VendorContract> = Vec::new();

    let contract1 = VendorContract::check_status(
        1,
        String::from("John"),
        1_000_000,
        NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
        NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
    );
    let contract2 = VendorContract::check_status(
        2,
        String::from("Faith"),
        4_000_000,
        NaiveDate::from_ymd_opt(2025, 1, 31).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 31).unwrap(),
    );

    VendorContract::add_contract(
        &mut contract_db,
        contract1.id,
        contract1.vendor_name,
        contract1.contract_value,
        contract1.start_date,
        contract1.end_date,
        contract1.status,
    );

    print!("\n the new db is {:?} \n", contract_db);

    VendorContract::add_contract2(&mut contract_db, contract2);

    // println!("Hello, world!");
    print!("\n the new db is {:?} \n", contract_db);

    VendorContract::view_allcontract(&contract_db);

    let mut contract_map: HashMap<u32, VendorContract> = contract_db
        .clone()
        .into_iter()
        .map(|contract| (contract.id as u32, contract))
        .collect();

    VendorContract::delete_expired_or_canceled(&mut contract_map);

    print!("\n the hashmap db is {:?} \n", contract_map);

    let contract_vec: Vec<VendorContract> = contract_map.values().cloned().collect();
    VendorContract::view_allcontract(&contract_vec);

    // Stage 3: Update contract by ID
    VendorContract::update_contract(
        &mut contract_map,
        2,
        Some("Faithy Enterprise Ltd.".to_string()),
        Some(5_500_000),
        None,
        Some(NaiveDate::from_ymd_opt(2026, 12, 1).unwrap()),
    );

    // Convert HashMap back to Vec to view
    let updated_vec: Vec<VendorContract> = contract_map.values().cloned().collect();

    println!("\n--- Updated Contracts After Delete + Update (Vec) ---");
    VendorContract::view_allcontract(&updated_vec);
}
