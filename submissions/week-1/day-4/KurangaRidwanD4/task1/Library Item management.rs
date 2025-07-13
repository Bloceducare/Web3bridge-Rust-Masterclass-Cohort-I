fn main(){
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
        ItemType::Book => println!("Type: Iwe"),
        ItemType::Magazine => println!("Type: Magazini"),
        ItemType::Fiction => println!("Type: Fictionu"),
    }
}
}