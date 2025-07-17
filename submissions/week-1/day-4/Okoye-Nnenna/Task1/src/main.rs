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

fn display_quantity(item: &LibraryItem){
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem){
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
    let item = LibraryItem {
        quantity: 3,
        id: 1,
        item_type: ItemType::Book,  
    };

    display_quantity(&item);
    display_id(&item);
    display_item_type(&item);
}