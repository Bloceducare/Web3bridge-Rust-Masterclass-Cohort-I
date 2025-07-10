#[derive(Debug)]
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

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_item_type(item: &LibraryItem) {
    println!("Type: {:?}", item.item_type);
}

fn main() {
    {
        let book = LibraryItem {
            quantity: 10,
            id: 101,
            item_type: ItemType::Book,
        };

        display_quantity(&book);
        display_id(&book);
        display_item_type(&book);
        println!()
    }


    {
        let book = LibraryItem {
            quantity: 40,
            id: 106,
            item_type: ItemType::Magazine,
        };

        display_quantity(&book);
        display_id(&book);
        display_item_type(&book);
        println!()
    }

    {
        let book = LibraryItem {
            quantity: 18,
            id: 102,
            item_type: ItemType::Fiction,
        };

        display_quantity(&book);
        display_id(&book);
        display_item_type(&book);
        println!()
    }
}
