use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub quantity: u32,
    pub price: f64,
}

impl Item {
    pub fn new(name: String, quantity: u32, price: f64) -> Self {
        Item {
            name,
            quantity,
            price
        }
    }

    pub fn display(&self) -> String {
        format!(
            "Item: {}, Quantity: {}, Price: ${:.2}",
            self.name, self.quantity, self.price
        )
    }

    pub fn total_value(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}

#[derive(Debug)]
pub enum InventoryError {
    ItemNotFound,
    ItemAlreadyExists,
    EmptyInventory,
    InvalidInput(String)
}

impl std::fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryError::ItemNotFound => write!(f, "Item not found in inventory."),
            InventoryError::ItemAlreadyExists => write!(f, "Item already exists. Use Edit option to modify existing items"),
            InventoryError::EmptyInventory => write!(f, "Inventory is empty."),
            InventoryError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for InventoryError {}

pub struct InventoryManager {
    items:  HashMap<String, Item>,
}

impl InventoryManager {
    pub fn new() -> Self {
        InventoryManager {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, name: String, quantity: u32, price: f64) -> Result<(), InventoryError> {
        if name.trim().is_empty() {
            return Err(InventoryError::InvalidInput("Item name cannot be empty".to_string()));
        }

        if price < 0.0 {
            return Err(InventoryError::InvalidInput("Price cannot be negative".to_string()));
        }

        let key = name.to_lowercase();
        if self.items.contains_key(&key) {
            return Err(InventoryError::ItemAlreadyExists);
        }

        let item = Item::new(name, quantity, price);
        self.items.insert(key, item);
        Ok(())
    }

    // Stage 1: Get all items for viewing
    pub fn get_all_items(&self) -> Vec<&Item> {
        let mut items_vec: Vec<_> = self.items.values().collect();
        items_vec.sort_by(|a, b| a.name.cmp(&b.name));
        items_vec
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn total_inventory_value(&self) -> f64 {
        self.items.values().map(|item| item.total_value()).sum()
    }

    // Stage 2: Remove items
    pub fn find_item(&self, name: &str) -> Option<&Item> {
        let key = name.to_lowercase();
        self.items.get(&key)
    }

    pub fn remove_item(&mut self, name: &str) -> Result<Item, InventoryError> {
        if self.items.is_empty() {
            return Err(InventoryError::EmptyInventory);
        }

        let key = name.to_lowercase();
        match self.items.remove(&key) {
            Some(item) => Ok(item),
            None => Err(InventoryError::ItemNotFound),
        }
    }

    // Stage 3: Edit items
    pub fn edit_item(&mut self, original_name: &str, new_name: String, new_quantity: u32, new_price: f64) -> Result<Item, InventoryError> {
        if self.items.is_empty() {
            return Err(InventoryError::EmptyInventory);
        }

        if new_name.trim().is_empty() {
            return Err(InventoryError::InvalidInput("Item name cannot be empty".to_string()));
        }

        if new_price < 0.0 {
            return Err(InventoryError::InvalidInput("Price cannot be negative".to_string()));
        }

        let old_key = original_name.to_lowercase();
        let new_key = new_name.to_lowercase();

        // Check if original item exists
        if !self.items.contains_key(&old_key) {
            return Err(InventoryError::ItemNotFound);
        }

        // Check if new name conflicts with existing item (unless it's the same item)
        if old_key != new_key && self.items.contains_key(&new_key) {
            return Err(InventoryError::ItemAlreadyExists);
        }

        // Remove old entry
        let _old_item = self.items.remove(&old_key).unwrap();

        // Create and insert updated item
        let updated_item = Item::new(new_name, new_quantity, new_price);
        self.items.insert(new_key, updated_item.clone());

        Ok(updated_item)
    }

    pub fn get_item_names(&self) -> Vec<String> {
        let mut names: Vec<_> = self.items.values().map(|item| item.name.clone()).collect();
        names.sort();
        names
    }
}

impl Default for InventoryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new("Test Item".to_string(), 10, 15.99);
        assert_eq!(item.name, "Test Item");
        assert_eq!(item.quantity, 10);
        assert_eq!(item.price, 15.99);
    }

    #[test]
    fn test_item_display() {
        let item = Item::new("Test Item".to_string(), 5, 9.99);
        let display = item.display();
        assert!(display.contains("Test Item"));
        assert!(display.contains("5"));
        assert!(display.contains("9.99"));
    }

    #[test]
    fn test_item_total_value() {
        let item = Item::new("Test Item".to_string(), 5, 10.0);
        assert_eq!(item.total_value(), 50.0);
    }

    #[test]
    fn test_inventory_manager_creation() {
        let manager = InventoryManager::new();
        assert_eq!(manager.item_count(), 0);
        assert!(manager.is_empty());
    }

    #[test]
    fn test_add_item_success() {
        let mut manager = InventoryManager::new();
        let result = manager.add_item("Test Item".to_string(), 10, 15.99);
        assert!(result.is_ok());
        assert_eq!(manager.item_count(), 1);
    }

    #[test]
    fn test_add_duplicate_item() {
        let mut manager = InventoryManager::new();
        manager.add_item("Test Item".to_string(), 10, 15.99).unwrap();
        let result = manager.add_item("test item".to_string(), 5, 10.0);
        assert!(matches!(result, Err(InventoryError::ItemAlreadyExists)));
    }

    #[test]
    fn test_remove_item() {
        let mut manager = InventoryManager::new();
        manager.add_item("Test Item".to_string(), 10, 15.99).unwrap();
        
        let removed_item = manager.remove_item("Test Item").unwrap();
        assert_eq!(removed_item.name, "Test Item");
        assert!(manager.is_empty());
    }

    #[test]
    fn test_edit_item() {
        let mut manager = InventoryManager::new();
        manager.add_item("Old Name".to_string(), 10, 15.99).unwrap();
        
        let result = manager.edit_item("Old Name", "New Name".to_string(), 20, 25.99);
        assert!(result.is_ok());
        
        let updated_item = result.unwrap();
        assert_eq!(updated_item.name, "New Name");
        assert_eq!(updated_item.quantity, 20);
        assert_eq!(updated_item.price, 25.99);
    }
}