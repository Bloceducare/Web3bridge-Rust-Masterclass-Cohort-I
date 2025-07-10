pub struct LibraryItem {
    pub quantity: i32,
    pub id: i32,
    pub item_type: ItemType,
}

pub enum temType {
    Book,
    Magazine,
    Fiction,
}


    pub fn display_quantiyy(&item: &LibraryItem) {
        println!("Quantity: {}", item.quantity);
    }

    pub fn display_id(&item: &LibraryItem) {
        println!("ID: {}", item.id);
    }

    fn display_item_type(&type: &LibraryItem) {
        println!("Type: {:?}", type.item_type);
    }
