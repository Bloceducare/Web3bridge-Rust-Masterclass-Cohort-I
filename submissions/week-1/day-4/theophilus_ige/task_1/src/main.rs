struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

enum ItemType {
    Book,
    Magazine,
    Fiction,
}


fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}


fn display_library_item(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    let my_book = LibraryItem {
        quantity: 4,
        id: 123,
        item_type: ItemType::Book,
    };

    // display item details
    display_quantity(&my_book);
    display_id(&my_book);
    display_library_item(&my_book);
}
