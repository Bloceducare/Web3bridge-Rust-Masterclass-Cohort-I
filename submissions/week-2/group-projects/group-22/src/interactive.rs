use crate::models::{Space, SpaceManager};
use std::io;

pub fn input(text: &str) -> String {
    println!("[prompt] -> {}", text);
    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .expect("something went wrong");

    prompt.trim().to_string()
}

pub fn add_space(space_manager: &mut SpaceManager) {
    let space_name = input("Enter space name: ");

    if space_manager.spaces.get(&space_name).is_none() {
        let occupant = input("Enter occupant name: ");
        let purpose = input("Enter purpose of the space: ");
        space_manager.spaces.insert(
            space_name.trim().to_string(),
            Space {
                name: space_name,
                occupant: occupant,
                purpose: purpose,
            },
        );
        println!("[response] -> Space Created \n");
    } else {
        println!("[response] -> Space existed already\n")
    }
}

pub fn view_spaces(space_manager: &SpaceManager) {
    if space_manager.spaces.is_empty() {
        println!("[response] -> No spaces available.\n");
        return;
    }

    for (_, space) in &space_manager.spaces {
        println!(
            "Space -> {}: Occupant: {}, Purpose: {}",
            space.name, space.occupant, space.purpose
        );
    }
    println!("");
}

pub fn remove_space(space_manager: &mut SpaceManager) {
    let space_name = input("Enter space name: ");

    if space_manager.spaces.get(&space_name).is_none() {
        println!("[response] -> Space does not exist.\n");
        return;
    }
    space_manager.spaces.remove(&space_name);
    println!("[response] -> Space deleted\n");
}

pub fn edit_space(space_manager: &mut SpaceManager) {
    let space_name = input("Enter space name: ");

    if let Some(space) = space_manager.spaces.get_mut(&space_name) {
        let confirmation = input(
            format!(
                "Are you sure you want to edit this space {}? (yes/no): ",
                space.name,
            )
            .as_str(),
        );
        if confirmation.to_lowercase() != "yes" {
            println!("[response] -> Edit cancelled.\n");
            return;
        }

        space.occupant = input("Enter new occupant name: ");
        space.purpose = input("Enter new purpose: ");
        return;
    }
    println!("[response] -> Space does not exist.\n");
}
