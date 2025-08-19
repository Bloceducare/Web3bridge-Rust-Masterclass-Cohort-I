// Define the ItemType enum with four variants
#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
    Newspaper,
    DVD,
}

// Define the LibraryItem struct
struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

// Function to display quantity
fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

// Function to display ID
fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

// Function to display item type
fn display_item_type(item: &LibraryItem) {
    match &item.item_type {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Newspaper => println!("Item Type: Newspaper"),
        ItemType::DVD => println!("Item Type: DVD"),
    }
}

fn main() {
    // Create example library items
    let item1 = LibraryItem {
        quantity: 10,
        id: 1001,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        quantity: 5,
        id: 1002,
        item_type: ItemType::DVD,
    };

    // Display details of item1
    display_quantity(&item1);
    display_id(&item1);
    display_item_type(&item1);

    println!("-----------------");

    // Display details of item2
    display_quantity(&item2);
    display_id(&item2);
    display_item_type(&item2);
}
