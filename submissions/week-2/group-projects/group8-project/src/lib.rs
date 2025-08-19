// using VEC

// #[derive(Debug)]
// pub struct Volunteer {
//     pub name: String,
//     pub role: String,
//     pub contact: String,
// }

// pub fn add_volunteer(volunteers: &mut Vec<Volunteer>, name: String, role: String, contact: String) {
//     let new_volunteer = Volunteer { name, role, contact };
//     volunteers.push(new_volunteer);
//     println!("Volunteer added successfully!");
// }

// pub fn view_volunteers(volunteers: &Vec<Volunteer>) {
//     if volunteers.is_empty() {
//         println!("No volunteers registered yet.");
//     } else {
//         println!("\n--- Volunteers List ---");
//         for (i, v) in volunteers.iter().enumerate() {
//             println!(
//                 "{}. Name: {} | Role: {} | Contact: {}",
//                 i + 1,
//                 v.name,
//                 v.role,
//                 v.contact
//             );
//         }
//     }
// }

// -- using HashMaps --
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Volunteer {
    pub name: String,
    pub role: String,
    pub contact: String,
}

	// -- Add and view volunteers --
pub fn add_volunteer(
    volunteers: &mut HashMap<u32, Volunteer>,
    id_counter: &mut u32,
    name: String,
    role: String,
    contact: String,
) {
    *id_counter += 1;
    let new_volunteer = Volunteer { name, role, contact };
    volunteers.insert(*id_counter, new_volunteer);
    println!("Volunteer added successfully with ID: {}", id_counter);
}

pub fn view_volunteers(volunteers: &HashMap<u32, Volunteer>) {
    if volunteers.is_empty() {
        println!("No volunteers registered yet.");
    } else {
        println!("\n-- Volunteers List --");
        for (id, v) in volunteers.iter() {
            println!(
                "ID: {} | Name: {} | Role: {} | Contact: {}", id, v.name, v.role, v.contact);
        }
    }
}

	// -- Remove a volunteer by ID --
pub fn remove_volunteer(volunteers: &mut HashMap<u32, Volunteer>, id: u32) {
    match volunteers.remove(&id) {
        Some(_) => println!("Volunteer with ID {} removed successfully", id),
        None => println!("No volunteer found with ID {}", id),
    }
}

	// -- Edit volunteer details with cancel option --
pub fn edit_volunteer(volunteers: &mut HashMap<u32, Volunteer>, id: u32, new_name: String, new_role: String, new_contact: String, confirm: bool) {
    if let Some(volunteer) = volunteers.get_mut(&id) {
        let old_details: Option<Volunteer> = Some(volunteer.clone());

        if confirm {
            volunteer.name = new_name;
            volunteer.role = new_role;
            volunteer.contact = new_contact;
            println!("Volunteer with ID {} updated successfully!", id);
        } else {
            if let Some(old) = old_details {
                *volunteer = old;
                println!("Edit cancelled. Volunteer details restored.");
            }
        }
    } else {
        println!("No volunteer found with ID {}", id);
    }
}
