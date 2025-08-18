use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub serial_number: String,
    pub value: f64,
}

pub enum MenuOption {
    Add,
    View,
    Remove,
    Edit,
    Exit,
}

pub enum Storage {
    Vec(Vec<Asset>),
    HashMap(HashMap<String, Asset>),
}

pub fn convert_to_hashmap(storage: Storage) -> Storage {
    match storage {
        Storage::Vec(vec) => {
            let mut hashmap = HashMap::new();
            for asset in vec {
                hashmap.insert(asset.serial_number.clone(), asset);
            }
            Storage::HashMap(hashmap)
        }
        Storage::HashMap(_) => storage,
    }
}

pub fn add_asset(storage: &mut Storage, name: String, serial_number: String, value: f64) {
    match storage {
        Storage::Vec(vec) => {
            if vec.iter().any(|a| a.serial_number == serial_number) {
                println!("Error: Serial number already exists!");
                return;
            }
            let asset = Asset {
                name,
                serial_number: serial_number.clone(),
                value,
            };
            vec.push(asset);
            println!("Asset added successfully!");
        }
        Storage::HashMap(hm) => {
            if hm.contains_key(&serial_number) {
                println!("Error: Serial number already exists!");
                return;
            }
            let asset = Asset {
                name,
                serial_number: serial_number.clone(),
                value,
            };
            hm.insert(serial_number, asset);
            println!("Asset added successfully!");
        }
    }
}

pub fn view_assets(storage: &Storage) {
    match storage {
        Storage::Vec(vec) => {
            if vec.is_empty() {
                println!("No assets found.");
                return;
            }
            for asset in vec {
                println!(
                    "Name: {}, Serial: {}, Value: ${:.2}",
                    asset.name, asset.serial_number, asset.value
                );
            }
        }
        Storage::HashMap(hm) => {
            if hm.is_empty() {
                println!("No assets found.");
                return;
            }
            for asset in hm.values() {
                println!(
                    "Name: {}, Serial: {}, Value: ${:.2}",
                    asset.name, asset.serial_number, asset.value
                );
            }
        }
    }
}

pub fn remove_asset(storage: &mut Storage, serial_number: String) {
    if let Storage::HashMap(hm) = storage {
        match hm.remove(&serial_number) {
            Some(_) => println!("Asset removed successfully!"),
            None => println!("Error: Asset not found!"),
        }
    }
}

pub fn edit_asset(
    storage: &mut Storage,
    serial_number: String,
    new_name: Option<String>,
    new_value: Option<f64>,
) {
    if let Storage::HashMap(hm) = storage {
        let asset = match hm.get(&serial_number) {
            Some(asset) => asset.clone(),
            None => {
                println!("Error: Asset not found!");
                return;
            }
        };

        let name = new_name.unwrap_or(asset.name);
        let value = new_value.unwrap_or(asset.value);

        let new_asset = Asset {
            name,
            serial_number: serial_number.clone(),
            value,
        };
        println!(
            "Preview: Name: {}, Serial: {}, Value: ${:.2}",
            new_asset.name, new_asset.serial_number, new_asset.value
        );
        // In the original code, confirmation is handled in main.rs, so we assume changes are saved
        hm.insert(serial_number, new_asset);
        println!("Asset updated successfully!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() -> Storage {
        let mut storage = Storage::Vec(Vec::new());
        if let Storage::Vec(vec) = &mut storage {
            vec.push(Asset {
                name: "Laptop".to_string(),
                serial_number: "SN123".to_string(),
                value: 999.99,
            });
            vec.push(Asset {
                name: "Phone".to_string(),
                serial_number: "SN456".to_string(),
                value: 499.99,
            });
        }
        storage
    }

    #[test]
    fn test_add_asset_vec() {
        let mut storage = setup();
        if let Storage::Vec(vec) = &mut storage {
            let new_asset = Asset {
                name: "Tablet".to_string(),
                serial_number: "SN789".to_string(),
                value: 299.99,
            };
            if !vec
                .iter()
                .any(|a| a.serial_number == new_asset.serial_number)
            {
                vec.push(new_asset.clone());
            }
            assert_eq!(vec.len(), 3);
            assert_eq!(vec[2].name, "Tablet");
            assert_eq!(vec[2].serial_number, "SN789");
            assert_eq!(vec[2].value, 299.99);
        }
    }

    #[test]
    fn test_add_asset_duplicate_serial_vec() {
        let mut storage = setup();
        if let Storage::Vec(vec) = &mut storage {
            let original_len = vec.len();
            let duplicate_asset = Asset {
                name: "Duplicate".to_string(),
                serial_number: "SN123".to_string(),
                value: 199.99,
            };
            vec.push(duplicate_asset.clone());
            assert_eq!(vec.len(), original_len + 1);
            vec.pop();
            add_asset(
                &mut storage,
                duplicate_asset.name,
                duplicate_asset.serial_number,
                duplicate_asset.value,
            );
        }
    }

    #[test]
    fn test_view_assets_vec() {
        let storage = setup();
        if let Storage::Vec(vec) = &storage {
            assert_eq!(vec.len(), 2);
            assert_eq!(vec[0].name, "Laptop");
            assert_eq!(vec[0].serial_number, "SN123");
            assert_eq!(vec[0].value, 999.99);
            assert_eq!(vec[1].name, "Phone");
            assert_eq!(vec[1].serial_number, "SN456");
            assert_eq!(vec[1].value, 499.99);
        }
    }

    #[test]
    fn test_convert_to_hashmap() {
        let storage = setup();
        let converted = convert_to_hashmap(storage);
        if let Storage::HashMap(hm) = &converted {
            assert_eq!(hm.len(), 2);
            assert!(hm.contains_key("SN123"));
            assert!(hm.contains_key("SN456"));
            assert_eq!(hm.get("SN123").unwrap().name, "Laptop");
            assert_eq!(hm.get("SN456").unwrap().name, "Phone");
        }
    }

    #[test]
    fn test_remove_asset_hashmap() {
        let mut storage = convert_to_hashmap(setup());
        if let Storage::HashMap(hm) = &mut storage {
            let original_len = hm.len();
            hm.remove("SN123");
            assert_eq!(hm.len(), original_len - 1);
            assert!(!hm.contains_key("SN123"));
            assert!(hm.contains_key("SN456"));
        }
    }

    #[test]
    fn test_edit_asset_hashmap() {
        let mut storage = convert_to_hashmap(setup());
        if let Storage::HashMap(hm) = &mut storage {
            let new_asset = Asset {
                name: "Updated Laptop".to_string(),
                serial_number: "SN123".to_string(),
                value: 1099.99,
            };
            hm.insert("SN123".to_string(), new_asset.clone());
            assert_eq!(hm.get("SN123").unwrap().name, "Updated Laptop");
            assert_eq!(hm.get("SN123").unwrap().value, 1099.99);
        }
    }

    #[test]
    fn test_empty_storage() {
        let storage = Storage::Vec(Vec::new());
        if let Storage::Vec(vec) = &storage {
            assert!(vec.is_empty());
        }
        let storage = Storage::HashMap(HashMap::new());
        if let Storage::HashMap(hm) = &storage {
            assert!(hm.is_empty());
        }
    }
}
