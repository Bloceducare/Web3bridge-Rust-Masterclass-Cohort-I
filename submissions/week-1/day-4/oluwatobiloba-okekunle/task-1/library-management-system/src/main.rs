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


fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    // Create a new library item
    let book = LibraryItem {
        quantity: 5,
        id: 1001,
        item_type: ItemType::Book,
    };

    // Display information about the book
    println!("Library Item Information:");
    println!("------------------------");
    display_quantity(&book);
    display_id(&book);
    display_type(&book);

    let magazine = LibraryItem {
        quantity: 10,
        id: 1002,
        item_type: ItemType::Magazine,
    };

    println!("Library Item Information:");
    display_quantity(&magazine);
    display_id(&magazine);
    display_type(&magazine);


    let fiction = LibraryItem {
        quantity: 20,
        id: 1003,
        item_type: ItemType::Fiction,
    };

    println!("Library Item Information:");
    display_quantity(&fiction);
    display_id(&fiction);
    display_type(&fiction);
}