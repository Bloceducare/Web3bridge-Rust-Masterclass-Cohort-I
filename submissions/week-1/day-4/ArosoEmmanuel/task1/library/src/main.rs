fn main() {
    let item1 = LibraryItem {
        id: 1,
        quantity: 5,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        id: 2,
        quantity: 3,
        item_type: ItemType::Magazine,
    };

    // Displaying the quantity of item1
    println!("Quantity: {}", item1.display_quantity(&item1));

    // Displaying the ID of item2
    println!("ID: {}", item2.display_id(&item2));
   

    // display the type of the LibraryItem
    item1.display_type(&item1);
    item2.display_type(&item2);

}



struct LibraryItem {
    id: i32,
    quantity: i32,
    item_type: ItemType,
}

enum ItemType {
    Book,
    Magazine,
    Fiction,
    NonFiction,
    Journal,
    Newspaper,
    Comics,
}

// implement two functions:

impl LibraryItem {
    fn display_quantity(&self, item: &LibraryItem) -> i32{
        // let quantity = item.quantity;
        // println!("Quantity: {}", self.quantity);
        item.quantity
    }

    fn display_id(&self, item: &LibraryItem) -> i32 {
        // println!("ID: {}", self.id);
        item.id
    }

    fn display_type(&self, item: &LibraryItem) {
        match item.item_type {
            ItemType::Book => println!("Type: Book"),
            ItemType::Magazine => println!("Type: Magazine"),
            ItemType::Fiction => println!("Type: Fiction"),
            ItemType::NonFiction => println!("Type: Non-Fiction"),
            ItemType::Journal => println!("Type: Journal"),
            ItemType::Newspaper => println!("Type: Newspaper"),
            ItemType::Comics => println!("Type: Comics"),
        }
    }
}

