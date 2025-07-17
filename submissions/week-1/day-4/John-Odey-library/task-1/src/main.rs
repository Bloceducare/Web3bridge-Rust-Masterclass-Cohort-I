#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

/// Prints the quantity of a given library item.
fn display_quantity(item: &LibraryItem) {
    println!("Item Quantity: {}", item.quantity);
}

/// Prints the ID of a given library item.
fn display_id(item: &LibraryItem) {
    println!("Item ID: {}", item.id);
}

/// Prints the type of the library item.
fn display_item_type(item: &LibraryItem) {
    println!("Item Type: {:?}", item.item_type);
}

fn main() {
    let book = LibraryItem {
        quantity: 5,
        id: 1,
        item_type: ItemType::Book,
    };

    let magazine = LibraryItem {
        quantity: 20,
        id: 2,
        item_type: ItemType::Magazine,
    };

    let fiction = LibraryItem {
        quantity: 15,
        id: 3,
        item_type: ItemType::Fiction,
    };

    println!("--- Item 1 ---");
    display_quantity(&book);
    display_id(&book);
    display_item_type(&book);

    println!("\n--- Item 2 ---");
    display_quantity(&magazine);
    display_id(&magazine);
    display_item_type(&magazine);
    
    println!("\n--- Item 3 ---");
    display_quantity(&fiction);
    display_id(&fiction);
    display_item_type(&fiction);
}
