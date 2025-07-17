enum ItemType {
    Book,
    Magazine,
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
    }
}

fn main() {
    let book = LibraryItem {
        quantity: 9,
        id: 22,
        item_type: ItemType::Book,
    };

    display_quantity(&book);
    display_id(&book);
    display_type(&book);
}