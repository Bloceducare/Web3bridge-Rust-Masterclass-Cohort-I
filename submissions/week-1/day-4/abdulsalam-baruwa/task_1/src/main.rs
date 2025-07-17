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
    match item.item_type {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Fiction => println!("Item Type: Fiction"),
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 3,
        id: 101,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        quantity: 7,
        id: 102,
        item_type: ItemType::Fiction,
    };

    let item3 = LibraryItem {
        quantity: 2,
        id: 103,
        item_type: ItemType::Magazine,
    };

    display_id(&item1);
    display_item_type(&item1);
    display_quantity(&item1);

    println!("---------------------------------");

    display_id(&item2);
    display_item_type(&item2);
    display_quantity(&item2);

    println!("---------------------------------");

    display_id(&item3);
    display_item_type(&item3);
    display_quantity(&item3);
}
