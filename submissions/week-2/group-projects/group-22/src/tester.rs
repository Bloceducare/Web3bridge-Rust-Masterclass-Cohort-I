use crate::models::{Space, SpaceManager};

pub fn add_space_test(
    space_manager: &mut SpaceManager,
    space_name: &str,
    occupant: &str,
    purpose: &str,
) {
    if space_manager.spaces.contains_key(space_name) {
        return println!("Space existed already");
    }
    space_manager.spaces.insert(
        space_name.to_string(),
        Space {
            name: space_name.to_string(),
            occupant: occupant.to_string(),
            purpose: purpose.to_string(),
        },
    );
}

pub fn view_spaces_test(space_manager: &SpaceManager) {
    if space_manager.spaces.is_empty() {
        return println!("No spaces available");
    }
}

pub fn remove_space_test(space_manager: &mut SpaceManager, space_name: &str) {
    if space_manager.spaces.remove(space_name).is_none() {
        return println!("Space does not exist");
    }
}

pub fn edit_space_test(
    space_manager: &mut SpaceManager,
    space_name: &str,
    new_occupant: &str,
    new_purpose: &str,
) {
    if let Some(space) = space_manager.spaces.get_mut(space_name) {
        space.occupant = new_occupant.to_string();
        space.purpose = new_purpose.to_string();
    } else {
        println!("Space does not exist");
    }
}
