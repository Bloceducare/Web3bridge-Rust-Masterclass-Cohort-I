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
    fn echo(&self) {
        println!("quantity: {}", self.quantity);
        println!("id: {}", self.id);
    }
}

fn main() {
    let book1: LibraryItem = LibraryItem {
        id: 001,
        quantity: 50,
        item_type: ItemType::Book,
    };

    let book2: LibraryItem = LibraryItem {
        id: 2,
        quantity: 23,
        item_type: ItemType::Fiction,
    };

    let book3: LibraryItem = LibraryItem {
        id: 3,
        quantity: 10,
        item_type: ItemType::Magazine,
    };

    book1.echo();
    book2.echo();
    book3.echo();
}
