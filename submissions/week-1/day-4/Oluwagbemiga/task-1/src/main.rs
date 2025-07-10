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
    println!("The quantity of the item is: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("The ID of the item is: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("This is a book."),
        ItemType::Magazine => println!("This is a magazine."),
        ItemType::Fiction => println!("This is a fiction."),
    }
}

fn main() {
    let book = LibraryItem {
        quantity: 5,
        id: 1,
        item_type: ItemType::Book,
    };

    let magazine = LibraryItem {
        quantity: 10,
        id: 2,
        item_type: ItemType::Magazine,
    };

    let fiction = LibraryItem {
        quantity: 15,
        id: 3,
        item_type: ItemType::Fiction,
    };

    display_quantity(&book);
    display_type(&book);

    display_quantity(&magazine);
    display_type(&magazine);

    display_quantity(&fiction);
    display_type(&fiction);
}
