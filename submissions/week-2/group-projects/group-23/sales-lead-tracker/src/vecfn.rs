// use std::io;
// use crate::mods::{Leads, LeadStatus};

// pub fn add_lead(leads: &mut Vec<Leads>, name: String, contact: String, value: f64, status: LeadStatus) {
//     if name.is_empty() || contact.is_empty() || value <= 0.0 {
//         println!("Invalid lead data provided.");
//         return;
//     }
    
//     let id = leads.len() as u32 + 1;
//     let new_lead = Leads { id, name, contact, value, status };
    
//     leads.push(new_lead);
//     println!("Lead added successfully.");
// }

// pub fn display_leads(leads: &Vec<Leads>) {
//     if leads.is_empty() {
//         println!("No leads available.");
//         return;
//     }
    
//     println!("{:<5} {:<20} {:<15} {:<10} {:<10}", "ID", "Name", "Contact", "Value", "Status");
//     println!("{:-<65}", "");
    
//     for lead in leads {
//         println!("{:<5} {:<20} {:<15} ${:<9.2} {:?}", 
//                  lead.id, lead.name, lead.contact, lead.value, lead.status);
//     }
// }

// pub fn edit_lead(leads: &mut Vec<Leads>, backup: &mut Option<Leads>) {
//     if leads.is_empty() {
//         println!("No leads available to edit.");
//         return;
//     }
    
//     display_leads(leads);
//     println!("\nEnter lead ID to edit:");
//     let mut id_input = String::new();
//     io::stdin().read_line(&mut id_input).expect("Failed to read line");
    
//     if let Ok(id) = id_input.trim().parse::<u32>() {
//         if let Some(lead) = leads.iter_mut().find(|l| l.id == id) {
//             *backup = Some(lead.clone());
            
//             println!("Editing lead: {}", lead.name);
            
//             loop {
//                 println!("\n1. Edit Name  2. Edit Contact  3. Edit Value  4. Save");
//                 println!("Choose option:");
                
//                 let mut choice = String::new();
//                 io::stdin().read_line(&mut choice).expect("Failed to read line");
                
//                 match choice.trim() {
//                     "1" => {
//                         println!("Enter new name:");
//                         let mut new_name = String::new();
//                         io::stdin().read_line(&mut new_name).expect("Failed to read line");
//                         lead.name = new_name.trim().to_string();
//                         println!("Name updated.");
//                     },
//                     "2" => {
//                         println!("Enter new contact:");
//                         let mut new_contact = String::new();
//                         io::stdin().read_line(&mut new_contact).expect("Failed to read line");
//                         lead.contact = new_contact.trim().to_string();
//                         println!("Contact updated.");
//                     },
//                     "3" => {
//                         println!("Enter new value:");
//                         let mut new_value = String::new();
//                         io::stdin().read_line(&mut new_value).expect("Failed to read line");
//                         if let Ok(value) = new_value.trim().parse::<f64>() {
//                             lead.value = value;
//                             println!("Value updated.");
//                         } else {
//                             println!("Invalid value format.");
//                         }
//                     },
//                     "4" => {
//                         *backup = None;
//                         println!("Changes saved.");
//                         break;
//                     },
//                     _ => println!("Invalid choice. Please enter 1, 2, 3, or 4."),
//                 }
//             }
//         } else {
//             println!("Lead not found.");
//         }
//     } else {
//         println!("Invalid ID.");
//     }
// }

// pub fn cancel_edit(leads: &mut Vec<Leads>, backup: &mut Option<Leads>) {
//     if let Some(original) = backup.take() {
//         if let Some(lead) = leads.iter_mut().find(|l| l.id == original.id) {
//             *lead = original;
//             println!("Changes cancelled.");
//         }
//     } else {
//         println!("No changes to cancel.");
//     }
// }

// pub fn get_lead_input() -> Result<(String, String, f64, LeadStatus), String> {
//     let mut name = String::new();
//     let mut contact = String::new();
//     let mut value_str = String::new();
//     let mut status_str = String::new();

//     println!("Enter lead name:");
//     io::stdin().read_line(&mut name).expect("Failed to read line");
    
//     println!("Enter contact information:");
//     io::stdin().read_line(&mut contact).expect("Failed to read line");
    
//     println!("Enter lead value:");
//     io::stdin().read_line(&mut value_str).expect("Failed to read line");
    
//     println!("Enter lead status (New, Contacted, Qualified, Lost, Converted):");
//     io::stdin().read_line(&mut status_str).expect("Failed to read line");

//     let value: f64 = value_str.trim().parse()
//         .map_err(|_| "Invalid value entered.")?;

//     let status = match status_str.trim().to_lowercase().as_str() {
//         "new" => LeadStatus::New,
//         "contacted" => LeadStatus::Contacted,
//         "qualified" => LeadStatus::Qualified,
//         "lost" => LeadStatus::Lost,
//         "converted" => LeadStatus::Converted,
//         _ => return Err("Invalid status entered.".to_string()),
//     };

//     Ok((name.trim().to_string(), contact.trim().to_string(), value, status))
// }