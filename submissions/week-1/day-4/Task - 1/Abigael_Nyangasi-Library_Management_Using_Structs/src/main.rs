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

fn display_item_type(item: &LibraryItem) {
    let type_str = match item.item_type {
        ItemType::Book => "Book",
        ItemType::Magazine => "Magazine",
        ItemType::Fiction => "Fiction",
    };
    println!("Item Type: {}", type_str);
}

fn main() {
    let item = LibraryItem {
        quantity: 5,
        id: 101,
        item_type: ItemType::Book,
    };

    display_quantity(&item);
    display_id(&item);
    display_item_type(&item);
}