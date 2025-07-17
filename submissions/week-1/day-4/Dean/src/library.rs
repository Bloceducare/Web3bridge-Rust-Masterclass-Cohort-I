pub enum ItemType {
    Book,
    Magazine,
    Fiction,
}

pub struct LibraryItem {
    pub(crate) quantity: i32,
    pub(crate) id: i32,
    pub(crate) item_type: ItemType,
}


pub fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

pub fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

pub fn display_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("Type: Book"),
        ItemType::Magazine => println!("Type: Magazine"),
        ItemType::Fiction => println!("Type: Fiction"),
    }
}
