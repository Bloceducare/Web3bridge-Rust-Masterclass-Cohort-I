#[derive(Debug)]
enum LibraryType {
    Book(String),
    Magazine(String),
    Fiction(String),
}

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: LibraryType,
}

fn main() {
    let item_1 = LibraryItem {
        quantity: 5,
        id: 1,
        item_type: LibraryType::Book(String::from("The Rust Programming Language")),
    };
    display_quantity(&item_1);
    display_id(&item_1);
    display_type(&item_1);
}

fn display_quantity(item: &LibraryItem) {
    println!("{}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("{}", item.id);
}

fn display_type(item: &LibraryItem) {
    println!("{:?}", item.item_type)
}
