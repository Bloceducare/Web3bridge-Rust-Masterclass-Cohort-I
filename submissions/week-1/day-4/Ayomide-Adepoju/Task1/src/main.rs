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
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_item_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Fiction => println!("Item Type: Fiction"),
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 5,
        id: 101,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        quantity: 2,
        id: 202,
        item_type: ItemType::Magazine,
    };

    display_quantity(&item1);
    display_id(&item1);
    display_item_type(&item1);

    println!("Book-------Magazine");

    display_quantity(&item2);
    display_id(&item2);
    display_item_type(&item2);
}
