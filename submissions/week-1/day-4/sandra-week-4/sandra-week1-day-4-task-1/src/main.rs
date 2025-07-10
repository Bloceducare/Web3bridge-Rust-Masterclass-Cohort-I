#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

#[derive(Debug)]
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
    println!("Item Type: {:?}", item.item_type);
}

fn main() {
    let book = LibraryItem {
        quantity: 15,
        id: 101,
        item_type: ItemType::Book,
    };

    let magazine = LibraryItem {
        quantity: 8,
        id: 102,
        item_type: ItemType::Magazine,
    };

    display_quantity(&book);
    display_id(&book);
    display_item_type(&book);

    println!();

    display_quantity(&magazine);
    display_id(&magazine);
    display_item_type(&magazine);
}
