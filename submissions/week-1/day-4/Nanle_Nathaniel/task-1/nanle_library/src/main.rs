
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

impl LibraryItem {
    // Constructor for LibraryItem
    // Takes quantity, id, and item_type as parameters
    // Returns a new instance of LibraryItem
    fn new(quantity: i32, id: i32, item_type: ItemType) -> Self {
        LibraryItem {
            quantity,
            id,
            item_type,
        }
    }

    fn display_quantity(&self, item: &LibraryItem) -> i32 {
        item.quantity
    }

    fn display_id(&self, item: &LibraryItem) -> i32 {
        item.id
    }

    fn get_item_type(&self) -> &ItemType {
        &self.item_type
    }
}

fn main() {
    // Create a new LibraryItem instance
    let book = LibraryItem::new(5, 1, ItemType::Book);
    
    // Display the quantity and id of the item
    println!("Quantity: {}", book.display_quantity(&book));
    println!("ID: {}", book.display_id(&book));
    
    // Display the item type
    match book.get_item_type() {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Fiction => println!("Item Type: Fiction"),
    }
}