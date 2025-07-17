enum ItemType {
    Book,
    Magazine,
    Fiction,
}

#[derive(Debug)]
struct LibraryItem {
    quantity: u32,
    id: i32,
    item_type: ItemType,
}

fn display_quantity(item: &LibraryItem) {
    println!("Item Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("Item ID: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Fiction => println!("Item Type: Fiction"),
    }
}

fn main() {
    let book = LibraryItem {
        quantity: 10,
        id: 1,
        item_type: ItemType::Book,
    };
    display_id(&book);
    display_quantity(&book);
    display_type(&book);
}
