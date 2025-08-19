
// Task 1: Library Item Management System

/// Represents different types of items available in the library
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Book,
    Magazine,
    Fiction,
    NonFiction,
    Reference,
}

/// Represents a library item with quantity, ID, and type
#[derive(Debug, Clone)]
pub struct LibraryItem {
    /// The quantity of this item available in the library
    pub quantity: i32,
    /// Unique identifier for this item
    pub id: i32,
    /// The type/category of this library item
    pub item_type: ItemType,
}

/// Displays the quantity of a library item
/// 
/// # Arguments
/// * `item` - A reference to the LibraryItem to display quantity for
pub fn display_quantity(item: &LibraryItem) {
    println!("Item Quantity: {}", item.quantity);
}

/// Displays the ID of a library item
/// 
/// # Arguments
/// * `item` - A reference to the LibraryItem to display ID for
pub fn display_id(item: &LibraryItem) {
    println!("Item ID: {}", item.id);
}

/// Displays the type of a library item
/// 
/// # Arguments
/// * `item` - A reference to the LibraryItem to display type for
pub fn display_item_type(item: &LibraryItem) {
    println!("Item Type: {:?}", item.item_type);
}

fn main() {
    println!("TASK 1: LIBRARY ITEM MANAGEMENT SYSTEM");
    println!("==========================================");
    
    // Create some library items
    let book = LibraryItem {
        quantity: 15,
        id: 1001,
        item_type: ItemType::Book,
    };
    
    let magazine = LibraryItem {
        quantity: 25,
        id: 2001,
        item_type: ItemType::Magazine,
    };
    
    let fiction_book = LibraryItem {
        quantity: 8,
        id: 3001,
        item_type: ItemType::Fiction,
    };
    
    // Display information for each library item
    println!("
Processing Library Item #1:");
    display_quantity(&book);
    display_id(&book);
    display_item_type(&book);
    
    println!("
Processing Library Item #2:");
    display_quantity(&magazine);
    display_id(&magazine);
    display_item_type(&magazine);
    
    println!("
Processing Library Item #3:");
    display_quantity(&fiction_book);
    display_id(&fiction_book);
    display_item_type(&fiction_book);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Task 1: Library Item Management System
    #[test]
    fn test_library_item_creation() {
        let book = LibraryItem {
            quantity: 10,
            id: 101,
            item_type: ItemType::Book,
        };
        assert_eq!(book.quantity, 10);
        assert_eq!(book.id, 101);
        assert_eq!(book.item_type, ItemType::Book);
    }
}
