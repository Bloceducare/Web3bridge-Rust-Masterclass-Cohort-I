#[derive(Debug)]

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

#[derive(Debug)]

enum ItemType {
    Book, 
    Magazine,
    Fiction
}

fn main() {
    let book_item = LibraryItem {
        quantity: 30,
        id: 1,
        item_type: ItemType::Book,
    };

    display_quantity(&book_item);
    display_id(&book_item);
    display_type(&book_item);
}

fn display_quantity(item: &LibraryItem) {
    let q = item.quantity;
    println!("q is {:?}", q);
}

fn display_id(item: &LibraryItem) {
    let x = item.id;
    println!("x is {:?}", x);
}

fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Book"),
        ItemType::Magazine => println!("Magazine"),
        ItemType::Fiction => println!("Fiction"),
    }
}