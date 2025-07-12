// Library Item Management Example in Rust

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

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 3,
        id: 101,
        item_type: ItemType::Book,
    };
    let item2 = LibraryItem {
        quantity: 5,
        id: 202,
        item_type: ItemType::Magazine,
    };
    let item3 = LibraryItem {
        quantity: 2,
        id: 303,
        item_type: ItemType::Fiction,
    };

    display_quantity(&item1);
    display_id(&item1);
    display_type(&item1);
    println!("---");
    display_quantity(&item2);
    display_id(&item2);
    display_type(&item2);
    println!("---");
    display_quantity(&item3);
    display_id(&item3);
    display_type(&item3);
}
