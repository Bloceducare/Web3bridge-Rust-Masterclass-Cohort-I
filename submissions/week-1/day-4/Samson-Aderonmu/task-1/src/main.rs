#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
    Fiction,
    NonFiction,
    Journal,
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
    match &item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
        ItemType::NonFiction => println!("Type: Non-Fiction"),
        ItemType::Journal => println!("Type: Journal"),
    }
}

fn main() {
    let book = LibraryItem {
        quantity: 5,
        id: 1001,
        item_type: ItemType::Book,
    };

    let magazine = LibraryItem {
        quantity: 3,
        id: 1002,
        item_type: ItemType::Magazine,
    };

    let fiction_book = LibraryItem {
        quantity: 8,
        id: 1003,
        item_type: ItemType::Fiction,
    };

    let non_fiction_book = LibraryItem {
        quantity: 2,
        id: 1004,
        item_type: ItemType::NonFiction,
    };

    let journal = LibraryItem {
        quantity: 1,
        id: 1005,
        item_type: ItemType::Journal,
    };

    println!("=== Library Item 1 ===");
    display_quantity(&book);
    display_id(&book);
    display_type(&book);

    println!("\n=== Library Item 2 ===");
    display_quantity(&magazine);
    display_id(&magazine);
    display_type(&magazine);

    println!("\n=== Library Item 3 ===");
    display_quantity(&fiction_book);
    display_id(&fiction_book);
    display_type(&fiction_book);

    println!("\n=== Library Item 4 ===");
    display_quantity(&non_fiction_book);
    display_id(&non_fiction_book);
    display_type(&non_fiction_book);

    println!("\n=== Library Item 5 ===");
    display_quantity(&journal);
    display_id(&journal);
    display_type(&journal);
}
