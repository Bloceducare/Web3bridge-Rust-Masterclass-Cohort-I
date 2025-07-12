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
    println!("The quantity is: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("The ID is: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
    }
}

fn main() {
    let my_book = LibraryItem {
        quantity: 5,
        id: 101,
        item_type: ItemType::Book,
    };

    display_quantity(&my_book);
    display_id(&my_book);
    display_type(&my_book);
}
