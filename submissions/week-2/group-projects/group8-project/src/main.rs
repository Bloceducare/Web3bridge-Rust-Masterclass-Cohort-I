    // - using VEC -
// use std::io;
// use group8_project::{add_volunteer, view_volunteers, Volunteer};

// fn main() {
//     let mut volunteers: Vec<Volunteer> = Vec::new();

//     loop {
//         println!("\n--- Event Volunteer Coordinator ---");
//         println!("1. Add Volunteer");
//         println!("2. View Volunteers");
//         println!("3. Exit");
//         println!("Enter your choice:");

//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).expect("Failed to read input");
//         let choice = choice.trim();

//         match choice {
//             "1" => {
//                 let (name, role, contact) = get_volunteer_details();
//                 add_volunteer(&mut volunteers, name, role, contact);
//             }
//             "2" => view_volunteers(&volunteers),
//             "3" => {
//                 println!("Goodbye!");
//                 break;
//             }
//             _ => println!("Invalid choice. Try again."),
//         }
//     }
// }

// fn get_volunteer_details() -> (String, String, String) {
//     let name = input("Enter volunteer name:");
//     let role = input("Enter volunteer role:");
//     let contact = input("Enter volunteer contact:");
//     (name, role, contact)
// }

// fn input(prompt: &str) -> String {
//     println!("{}", prompt);
//     let mut value = String::new();
//     io::stdin().read_line(&mut value).expect("Failed to read input");
//     value.trim().to_string()
// }

    // -- HashMaps --
use std::collections::HashMap;
use std::io;

use group8_project::{add_volunteer, view_volunteers, remove_volunteer, edit_volunteer, Volunteer};

fn main() {
    let mut volunteers: HashMap<u32, Volunteer> = HashMap::new();
    let mut id_counter: u32 = 0;

    loop {
        println!("\n-- Event Volunteer Coordinator --");
        println!("1. Add Volunteer");
        println!("2. View Volunteers");
        println!("3. Remove Volunteer");
        println!("4. Edit Volunteer");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let (name, role, contact) = get_volunteer_details();
                add_volunteer(&mut volunteers, &mut id_counter, name, role, contact);
            }
            "2" => view_volunteers(&volunteers),
            "3" => {
                println!("Enter Volunteer ID to remove:");
                let id = read_id();
                remove_volunteer(&mut volunteers, id);
            }
            "4" => {
                println!("Enter Volunteer ID to edit:");
                let id = read_id();

                let (new_name, new_role, new_contact) = get_volunteer_details();

                println!("Confirm edit? (y/n):");
                let mut confirm_input = String::new();
                io::stdin().read_line(&mut confirm_input).expect("Failed to read");

                let confirm = confirm_input.trim().eq_ignore_ascii_case("y");
                edit_volunteer(&mut volunteers, id, new_name, new_role, new_contact, confirm);
            }
            "5" => {
                println!("Exiting program...");
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Try again!"),
        }
    }
}

fn get_volunteer_details() -> (String, String, String) {
    let mut name = String::new();
    let mut role = String::new();
    let mut contact = String::new();

    println!("Enter volunteer name:");
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter volunteer role:");
    io::stdin().read_line(&mut role).expect("Failed to read input");

    println!("Enter volunteer contact:");
    io::stdin().read_line(&mut contact).expect("Failed to read input");

    (
        name.trim().to_string(),
        role.trim().to_string(),
        contact.trim().to_string(),
    )
}

fn read_id() -> u32 {
    let mut id_input = String::new();
    io::stdin().read_line(&mut id_input).expect("Failed to read input");
    id_input.trim().parse::<u32>().unwrap_or(0)
}
