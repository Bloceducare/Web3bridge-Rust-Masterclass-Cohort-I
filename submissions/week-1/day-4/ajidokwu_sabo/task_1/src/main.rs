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

fn display_item_type(item: &LibraryItem) {
    println!("Displaying the type of the LibraryItem: {:?}", item.item_type);
}

fn main() {
    let book = LibraryItem {
        quantity: 5,
        id: 1,
        item_type: ItemType::Book,
    };

    let magazine = LibraryItem {
        quantity: 5,
        id: 2,
        item_type: ItemType::Magazine,
    };

    let fiction = LibraryItem {
        quantity: 5,
        id: 3,
        item_type: ItemType::Fiction,
    };

    println!("Library Item");
    display_id(&book);
    display_quantity(&book);
    display_item_type(&book);

    println!("Library Item 2");
    display_id(&magazine);
    display_quantity(&magazine);
    display_item_type(&magazine);

    println!("Library Item 3");
    display_id(&fiction);
    display_quantity(&fiction);
    display_item_type(&fiction);
}
