#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
    Fiction,
}
struct LibraryItem {
    id: i32,
    quantity: i32,
    item_type: ItemType,
}

//  Implementing two functions
fn display_quantity(item: &LibraryItem) {
    println!("Item quantity is {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("Item Id is {} ", item.id);
}

fn display_item_in_library(item: &LibraryItem) {
    println!("Item Type is {:?}", item.item_type);
}

fn main() {
    println!("Hello, world!");

    let item = LibraryItem {
        id: 1,
        quantity: 10,
        item_type: ItemType::Book,
    };
    display_quantity(&item);
    display_id(&item);
    display_item_in_library(&item);
}
