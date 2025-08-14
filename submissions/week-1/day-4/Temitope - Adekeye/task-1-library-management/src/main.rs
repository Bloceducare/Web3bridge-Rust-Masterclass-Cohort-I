

enum ItemType {
    Book,
    Magazine,
    Fiction,
}


pub struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}


impl LibraryItem {
    pub fn display_quantity(&self) {
        println!("Quantity: {}", self.quantity);
    }

    pub fn display_id(&self) {
        println!("ID: {}", self.id);
    }

    pub fn display_type(&self) {
        match self.item_type {
            ItemType::Book => println!("Item Type: Book"),
            ItemType::Magazine => println!("Item Type: Magazine"),
            ItemType::Fiction => println!("Item Type: Fiction"),
        }
    }
}

fn main() {
    println!("\nTemitope Adekeye!\n");

    let item = LibraryItem{
        quantity: 5,
        id: 1,
        item_type: ItemType::Book,
    };

    item.display_quantity();
    item.display_id();
    item.display_type();
}
