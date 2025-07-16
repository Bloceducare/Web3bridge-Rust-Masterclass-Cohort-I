// Define the enum for item types
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

// Define the struct for a library item
struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

// Function to display the quantity
fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

// Function to display the ID
fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

// Function to display the item type
fn display_item_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    let book = LibraryItem {
        quantity: 3,
        id: 101,
        item_type: ItemType::Book,
    };

    display_quantity(&book);
    display_id(&book);
    display_item_type(&book);
}