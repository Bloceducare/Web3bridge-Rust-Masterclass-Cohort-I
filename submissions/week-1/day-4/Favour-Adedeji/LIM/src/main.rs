// - Create a `struct` named `LibraryItem`.
// - It should contain three fields:
//   - `quantity: i32`
//   - `id: i32`
// - Add a third field called `item_type`, which should be of an `enum` type named `ItemType`.
// - The `ItemType` enum should have at least two variants (e.g., `Book`, `Magazine`,`Fiction`).

// - Implement two functions:
//   - `display_quantity(item: &LibraryItem)` — prints the quantity
//   - `display_id(item: &LibraryItem)` — prints the ID
//   - another fn to display the type of the LibraryItem.

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

impl LibraryItem {
    fn display_quantity(&self) {
        println!("Quantity: {}", self.quantity);
    }

    fn display_id(&self) {
        println!("ID: {}", self.id);
    }

    fn display_item_type(&self) {
        match self.item_type {
            ItemType::Book => println!("Item Type: Book"),
            ItemType::Magazine => println!("Item Type: Magazine"),
            ItemType::Fiction => println!("Item Type: Fiction"),
        }
    }
}

fn main() {
    let item = LibraryItem {
        quantity: 10,
        id: 1,
        item_type: ItemType::Book,
    };

    item.display_quantity();
    item.display_id() ;
    item.display_item_type();  
}
