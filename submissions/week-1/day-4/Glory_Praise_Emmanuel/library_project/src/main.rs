#[derive(Debug, Clone)]
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

struct LibraryItem{
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

fn display_quantity(item: &LibraryItem) -> i32 {
   item.quantity
}

fn display_id(item: &LibraryItem) -> i32 {
    item.id
}

fn display_type(item: &LibraryItem) -> ItemType {
    item.item_type.clone()
}

fn main() {
    println!("\nLibrary Inventory");

    let library_item1 = LibraryItem {
        quantity: 10,
        id: 001010,
        item_type: ItemType::Book
    };

    let library_item2 = LibraryItem {
        quantity: 20,
        id: 001011,
        item_type: ItemType::Magazine
    };

    let library_item3 = LibraryItem {
        quantity: 2,
        id: 001012,
        item_type: ItemType::Fiction
    };

    println!("\nThere are {} copies of {:?}s with ID number {} available in the library.", display_quantity(&library_item1), display_type(&library_item1), display_id(&library_item1));

    println!("\nThere are {} copies of {:?}s with ID number {} available in the library.", display_quantity(&library_item2), display_type(&library_item2), display_id(&library_item2));
    
    println!("\nThere are {} copies of {:?}s with ID number {} available in the library.", display_quantity(&library_item3), display_type(&library_item3), display_id(&library_item3));
}
