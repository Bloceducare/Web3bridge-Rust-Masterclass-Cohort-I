// enum for different library items
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

// struct for a library item
struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

// Function to display the quantity of a library item
fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

// Function to display the ID of a library item
fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

// Function to display the type of library item
fn display_item_type(item: &LibraryItem) {
    match &item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

// Entry point of the program
fn main() {
    // Create a library item (example)
    let item = LibraryItem {
        quantity: 5,
        id: 501,
        item_type: ItemType::Fiction,
    };

    // Call display functions
    display_quantity(&item);
    display_id(&item);
    display_item_type(&item);
}
