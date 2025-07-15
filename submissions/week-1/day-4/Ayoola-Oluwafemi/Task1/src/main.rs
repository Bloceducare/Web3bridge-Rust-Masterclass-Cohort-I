// Define the ItemType enum
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

// Define the LibraryItem struct
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
fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 3,
        id: 101,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        quantity: 5,
        id: 202,
        item_type: ItemType::Magazine,
    };

    display_quantity(&item1);
    display_id(&item1);
    display_type(&item1);

    println!("---");

    display_quantity(&item2);
    display_id(&item2);
    display_type(&item2);
}
