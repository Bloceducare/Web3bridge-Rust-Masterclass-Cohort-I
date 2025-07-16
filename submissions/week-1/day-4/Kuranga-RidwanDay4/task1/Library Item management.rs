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
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Alaroye"),
        ItemType::Fiction => println!("Type: Alawiye"),
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 30,
        id: 1,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        quantity: 10,
        id: 2,
        item_type: ItemType::Magazine,
    };

    let item3 = LibraryItem {
        quantity: 40,
        id: 3,
        item_type: ItemType::Fiction,
    };

    display_quantity(&item1);
    display_id(&item1);
    display_type(&item1);

    display_quantity(&item2);
    display_id(&item2);
    display_type(&item2);

    display_quantity(&item3);
    display_id(&item3);
    display_type(&item3);
}