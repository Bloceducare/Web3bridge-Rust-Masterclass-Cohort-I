fn main() {
    let book = LibraryItem {
        quantity: 5,
        id: 101,
        item_type: ItemType::Book,
    };
    
    display_quantity(&book);
    display_id(&book);
    display_type(&book);
    

}

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

fn display_quantity(item: &LibraryItem) {
    println!("quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("id: {}", item.id);
}

fn display_type(item: &LibraryItem) {
    let type_name = match item.item_type {
        ItemType::Book => "Book",
        ItemType::Magazine => "Magazine",
        ItemType::Fiction => "Fiction",
    };
    println!("Type: {}", type_name);
}