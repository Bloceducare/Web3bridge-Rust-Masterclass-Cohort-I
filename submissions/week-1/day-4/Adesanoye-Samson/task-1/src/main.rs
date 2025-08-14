enum ItemType {
    Book,
    Magazine,
    Fiction
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
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 4,
        id: 23,
        item_type: ItemType::Magazine
    };

    let item2 = LibraryItem {
        quantity: 7,
        id: 23,
        item_type: ItemType::Book,
    };

    let item3 = LibraryItem {
        quantity: 78,
        id: 45,
        item_type: ItemType::Fiction,
    };

    display_quantity(&item1);
    display_id(&item1);
    display_item_type(&item1);

    display_quantity(&item2);
    display_id(&item2);
    display_item_type(&item2);

    display_quantity(&item3);
    display_id(&item3);
    display_item_type(&item3);
}
