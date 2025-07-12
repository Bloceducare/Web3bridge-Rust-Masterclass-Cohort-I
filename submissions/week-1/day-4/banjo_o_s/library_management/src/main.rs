fn main() {
    // println!("Hello, world!");

    let book = LibraryItem {
        quantity: 10,
        id: 1,
        item_type: ItemType::Book,
    };

    let magazine = LibraryItem {
        quantity: 5,
        id: 2,
        item_type: ItemType::Magazine,
    };

    let fiction = LibraryItem {
        quantity: 3,
        id: 3,
        item_type: ItemType::Fiction,
    };

    println!("Book ID: {}, Quantity: {}, Type: {:?}", 
             book.display_id(), 
             book.display_quantity(), 
             book.display_library_type());
}


impl LibraryItem {
    fn display_quantity(&self) -> i32 {
        self.quantity
    }

    fn display_id(&self) -> i32 {
        self.id
    }

    fn display_library_type(&self) -> ItemType {
        self.item_type
    }
}


#[derive(Debug)]
struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

#[derive(Debug, Copy, Clone)]
enum ItemType {
    Book,
    Magazine, 
    Fiction
}