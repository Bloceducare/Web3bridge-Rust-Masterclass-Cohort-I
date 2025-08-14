mod mods;


// ... HASHMAP IMPLEMENTATIONS ...
mod hashmapfn;
mod vecfn;
use vecfn::*;
use std::collections::HashMap;
use std::io;
use mods::Leads;
use hashmapfn::*;

fn main() {
    let mut leads: HashMap<u32, Leads> = HashMap::new();
    let mut backup_lead: Option<Leads> = None;
    let mut input = String::new();

    loop {
        println!("... Lead Management System (HashMap) ...");
        println!("1. Add Lead");
        println!("2. Display Leads");
        println!("3. Remove Lead");
        println!("4. Edit Lead");
        println!("5. Cancel Edit");
        println!("6. Exit");
        println!("....................................................");
        println!("Please select an option:");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim() {
            "1" => {
                match get_lead_input() {
                    Ok((name, contact, value, status)) => {
                        add_lead(&mut leads, name, contact, value, status);
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            },
            "2" => display_leads(&leads),
            "3" => remove_lead(&mut leads),
            "4" => edit_lead(&mut leads, &mut backup_lead),
            "5" => cancel_edit(&mut leads, &mut backup_lead),
            "6" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid option selected. Please try again."),
        }
    }
}

// ... VEC IMPLEMENTATION ...

// mod vecfn;
// use std::io;
// use mods::Leads;
// use vecfn::*;

// fn main() {
//     let mut leads: Vec<Leads> = Vec::new();
//     let mut backup_lead: Option<Leads> = None;
//     let mut input = String::new();

//     loop {
//         println!("... Lead Management System (Vec) ...");
//         println!("1. Add Lead");
//         println!("2. Display Leads");
//         println!("3. Edit Lead");
//         println!("4. Cancel Edit");
//         println!("5. Exit");
//         println!("....................................................");
//         println!("Please select an option:");

//         input.clear();
//         io::stdin().read_line(&mut input).expect("Failed to read line");
        
//         match input.trim() {
//             "1" => {
//                 match get_lead_input() {
//                     Ok((name, contact, value, status)) => {
//                         add_lead(&mut leads, name, contact, value, status);
//                     },
//                     Err(err) => {
//                         println!("{}", err);
//                     }
//                 }
//             },
//             "2" => display_leads(&leads),
//             "3" => edit_lead(&mut leads, &mut backup_lead),
//             "4" => cancel_edit(&mut leads, &mut backup_lead),
//             "5" => {
//                 println!("Goodbye!");
//                 break;
//             },
//             _ => println!("Invalid option selected. Please try again."),
//         }
//     }
// }