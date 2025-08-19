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

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}
fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_type(item: &LibraryItem) {   
    println!("Item Type: {:?}", item.item_type);
}
fn display_all_info(item: &LibraryItem) {
    println!("--- Library Item Details ---");
    display_id(item);
    display_quantity(item);
    display_type(item);
    println!();
}

fn main() {
    let book1 = LibraryItem {
        id: 101,
        quantity: 5,
        item_type: ItemType::Book,
    };

    let magazine1 = LibraryItem {
        id: 201,
        quantity: 12,
        item_type: ItemType::Magazine,
    };

    let fiction_book = LibraryItem {
        id: 301,
        quantity: 3,
        item_type: ItemType::Fiction,
    };

    println!("=== Library Management System ===\n");

    println!("Book 1:");
    display_quantity(&book1);
    display_id(&book1);
    display_type(&book1);
    println!();

    println!("Magazine 1:");
    display_all_info(&magazine1);

    println!("Fiction Book:");
    display_all_info(&fiction_book);

    println!("Direct access example:");
    println!("Book 1 has {} copies with ID {}", book1.quantity, book1.id);
}