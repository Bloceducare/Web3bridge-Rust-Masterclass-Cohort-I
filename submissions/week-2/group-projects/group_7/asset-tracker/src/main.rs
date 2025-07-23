use asset_tracker::{
    Asset, MenuOption, Storage, add_asset, convert_to_hashmap, edit_asset, remove_asset,
    view_assets,
};
use std::io::{self, Write};

fn main() {
    let mut storage = Storage::Vec(Vec::new());
    loop {
        match show_menu() {
            Ok(MenuOption::Add) => {
                let (name, serial_number, value) = get_asset_input();
                match value {
                    Ok(v) => add_asset(&mut storage, name, serial_number, v),
                    Err(e) => println!("Error: {}", e),
                }
            }
            Ok(MenuOption::View) => view_assets(&storage),
            Ok(MenuOption::Remove) => {
                storage = convert_to_hashmap(storage);
                let serial_number = get_serial_number_input();
                remove_asset(&mut storage, serial_number);
            }
            Ok(MenuOption::Edit) => {
                storage = convert_to_hashmap(storage);
                let serial_number = get_serial_number_input();
                let (new_name, new_value) = get_edit_input(&storage, &serial_number);
                if new_name.is_some() || new_value.is_some() {
                    let confirm = read_input("Save changes? (y/n): ");
                    if confirm.to_lowercase() == "y" {
                        edit_asset(&mut storage, serial_number, new_name, new_value);
                    } else {
                        println!("Changes canceled.");
                    }
                }
            }
            Ok(MenuOption::Exit) => break,
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn show_menu() -> Result<MenuOption, String> {
    println!("\nAsset Tracker");
    println!("1. Add Asset");
    println!("2. View Assets");
    println!("3. Remove Asset");
    println!("4. Edit Asset");
    println!("5. Exit");
    let input = read_input("Enter choice: ");
    match input.as_str() {
        "1" => Ok(MenuOption::Add),
        "2" => Ok(MenuOption::View),
        "3" => Ok(MenuOption::Remove),
        "4" => Ok(MenuOption::Edit),
        "5" => Ok(MenuOption::Exit),
        _ => Err("Invalid choice".to_string()),
    }
}

fn get_asset_input() -> (String, String, Result<f64, String>) {
    let name = read_input("Enter asset name: ");
    let serial_number = read_input("Enter serial number: ");
    let value_input = read_input("Enter value: ");
    let value = value_input
        .parse::<f64>()
        .map_err(|_| "Invalid value!".to_string());

    (name, serial_number, value)
}

fn get_serial_number_input() -> String {
    read_input("Enter serial number: ")
}

fn get_edit_input(storage: &Storage, serial_number: &str) -> (Option<String>, Option<f64>) {
    let asset = if let Storage::HashMap(hm) = storage {
        hm.get(serial_number).cloned()
    } else {
        None
    };

    let asset = match asset {
        Some(a) => a,
        None => {
            println!("Error: Asset not found!");
            return (None, None);
        }
    };

    let prompt_name = format!("Enter new name (or press Enter to keep '{}'): ", asset.name);
    let name = read_input(&prompt_name);
    let new_name = if name.trim().is_empty() {
        None
    } else {
        Some(name)
    };

    let prompt_value = format!(
        "Enter new value (or press Enter to keep {:.2}): ",
        asset.value
    );
    let value_input = read_input(&prompt_value);
    let new_value = if value_input.trim().is_empty() {
        None
    } else {
        match value_input.trim().parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => {
                println!("Error: Invalid value!");
                None
            }
        }
    };

    (new_name, new_value)
}

// Function to read input from the user
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
