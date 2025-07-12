// enum definition
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

// struct definition
struct LibraryItem {
    id: i32,
    quantity: i32,
    item_type: ItemType,
}

// implementation of methods for LibraryItem
impl LibraryItem {
    fn display_quantity(&self) -> i32 {
        self.quantity
    }

    fn display_id(&self) -> i32 {
        self.id
    }

    fn display_type(&self) -> &str {
        match self.item_type {
            ItemType::Book => "Book",
            ItemType::Magazine => "Magazine",
            ItemType::Fiction => "Fiction",
        }
    }
}

fn main() {
    // struct instantiations
    let book_1 = LibraryItem {
        id: 1,
        quantity: 50,
        item_type: ItemType::Book,
    };

    let magazine_1 = LibraryItem {
        id: 2,
        quantity: 15,
        item_type: ItemType::Magazine,
    };

    let fiction_1 = LibraryItem {
        id: 3,
        quantity: 25,
        item_type: ItemType::Fiction,
    };

    // displays

    println!(
        "Book Item 1 - Id: {}, Quantity: {}, Type: {}",
        book_1.display_id(),
        book_1.display_quantity(),
        book_1.display_type()
    );

    println!(
        "Magazine Item 1 - Id: {}, Quantity: {}, Type: {}",
        magazine_1.display_id(),
        magazine_1.display_quantity(),
        magazine_1.display_type()
    );

    println!(
        "Fiction Item 1 - Id: {},  Quantity: {}, Type: {}",
        fiction_1.display_id(),
        fiction_1.display_quantity(),
        fiction_1.display_type()
    );
}
