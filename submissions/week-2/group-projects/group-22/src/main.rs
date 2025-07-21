use std::io;
mod interactive;
mod models;
mod tester;

use crate::interactive::{add_space, edit_space, remove_space, view_spaces};
use crate::models::{Space, SpaceManager};
use crate::tester::{add_space_test, edit_space_test, remove_space_test, view_spaces_test};

fn main() {
    let mut space_manager = SpaceManager::new();

    println!(
        "
    ===============================================
    =                  Welcome                    =
    =           Facility Space Manager            =
    =                                             =
    =   Authors:                                  =
    =           Adekeye Temitope;                 =
    =           Gbangbola Oluwagbemiga;           =
    =           James Emmanuel;                   =
    =           Ofuzor Chukwuemeke;               =
    =                                             =
    =                               Group 22;     =
    ===============================================
    "
    );

    loop {
        println!("Prompt: To create a Space, press 1");
        // println!("\tTo create a Space, press 1");
        println!("\tTo view all Spaces, press 2");
        println!("\tTo edit a Space, press 3");
        println!("\tTo delete a Space, press 4");
        println!("\tTo abort, press 5");

        let mut option = String::new();

        let _ = io::stdin().read_line(&mut option);

        match option.trim() {
            "1" => {
                add_space(&mut space_manager);
            }
            "2" => {
                view_spaces(&space_manager);
            }
            "3" => {
                edit_space(&mut space_manager);
            }
            "4" => {
                remove_space(&mut space_manager);
            }
            "5" => {
                println!("[Thank you for staying with us . . . do come again]");
                break;
            }
            _ => println!("Unrecognized command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn initialize() -> SpaceManager {
        let mut space_manager = SpaceManager::new();
        add_space_test(
            &mut space_manager,
            "Web3Bridge Garage",
            "Gbemiga",
            "Meetings",
        );
        add_space_test(&mut space_manager, "Ethereum house", "Temi", "Sleeping");
        space_manager
    }

    #[test]
    fn test_add_space() {
        let space_manager = initialize();
        assert_eq!(space_manager.spaces.len(), 2);
    }

    #[test]
    fn test_view_spaces() {
        let space_manager = initialize();
        assert!(!space_manager.spaces.is_empty());
    }

    #[test]
    fn test_remove_space() {
        let mut space_manager = initialize();
        remove_space_test(&mut space_manager, "Ethereum house");
        assert_eq!(space_manager.spaces.len(), 1);
    }

    #[test]
    fn test_edit_space() {
        let mut space_manager = initialize();
        edit_space_test(
            &mut space_manager,
            "Web3Bridge Garage",
            "Gbemiga",
            "Meetings and Coding",
        );
        assert_eq!(
            space_manager
                .spaces
                .get("Web3Bridge Garage")
                .unwrap()
                .purpose,
            "Meetings and Coding"
        );
    }
}
