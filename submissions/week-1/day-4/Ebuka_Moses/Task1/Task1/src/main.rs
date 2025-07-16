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
    match &item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    let book = LibraryItem {
        quantity: 3,
        id: 101,
        item_type: ItemType::Book,
    };
    let magazine = LibraryItem {
        quantity: 5,
        id: 202,
        item_type: ItemType::Magazine,
    };
    let fiction = LibraryItem {
        quantity: 2,
        id: 303,
        item_type: ItemType::Fiction,
    };

    display_quantity(&book);
    display_id(&book);
    display_type(&book);
    println!("---");
    display_quantity(&magazine);
    display_id(&magazine);
    display_type(&magazine);
    println!("---");
    display_quantity(&fiction);
    display_id(&fiction);
    display_type(&fiction);
}
