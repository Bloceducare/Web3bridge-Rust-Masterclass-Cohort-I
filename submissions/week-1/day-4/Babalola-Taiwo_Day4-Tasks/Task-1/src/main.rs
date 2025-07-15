enum ItemType {
    Book,
    Magazine,
    Fiction,
}

struct LibraryItem {
    quantity: u32,
    id: u32,
    item_type: ItemType,
}

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID Number: {}", item.id);
}

fn display_item_type(item: &LibraryItem) {
    let item_type_str = match item.item_type {
        ItemType::Book => "Book",
        ItemType::Magazine => "Magazine",
        ItemType::Fiction => "Fiction",
    };
    println!("Item Type: {}", item_type_str);
}

fn main() {
    let my_book = LibraryItem {
        quantity: 10,
        id: 101,
        item_type: ItemType::Book,
    };

    display_quantity(&my_book);
    display_id(&my_book);
    display_item_type(&my_book);
}
