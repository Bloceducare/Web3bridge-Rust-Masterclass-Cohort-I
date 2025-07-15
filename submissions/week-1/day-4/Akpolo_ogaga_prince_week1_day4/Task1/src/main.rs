



#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}


struct LibraryItem {
    quantity: u8,
    id: u64,
    item_type: ItemType,
}
fn main() {


    let book = LibraryItem {
        quantity: 5,
        id: 101,
        item_type: ItemType::Book,
    };

    display_quantity(&book);
    display_id(&book);
    display_item_type(&book);
}

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_item_type(item: &LibraryItem) {
    println!("Item Type: {:?}", item.item_type);
}