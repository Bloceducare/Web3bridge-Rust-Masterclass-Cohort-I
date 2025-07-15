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

// Functions
fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}


fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}


fn display_item_type(item: &LibraryItem) {
    println!("Item Type: {:?}", item.item_type);
}



fn main() {
   
    let rust_book = LibraryItem {
        quantity: 5,
        id: 1001,
        item_type: ItemType::Book,
    };
    
    let ovation = LibraryItem {
        quantity: 12,
        id: 2001,
        item_type: ItemType::Magazine,
    };
    
    let harry_potter = LibraryItem {
        quantity: 8,
        id: 3001,
        item_type: ItemType::Fiction,
    };
    
   
    println!("=== Library Item Information ===");
    
    println!("\n For Rust Book (ID: 1001):");
    display_id(&rust_book);
    display_quantity(&rust_book);
    display_item_type(&rust_book);

    println!("\n For Ovation Magazine (ID: 2001):");
    display_id(&ovation);
    display_quantity(&ovation);
    display_item_type(&ovation);

    println!("\n For Harry Potter (ID: 3001):");
    display_id(&harry_potter);
    display_quantity(&harry_potter);
    display_item_type(&harry_potter);
}