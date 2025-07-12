use crate::library::{ItemType, LibraryItem, display_id, display_quantity, display_type};
use crate::shipping::{Color, ShippingBox};

mod library;
mod shipping;

fn main() {
    let book = LibraryItem {
        quantity: 5,
        id: 1001,
        item_type: ItemType::Book,
    };

    dbg!("Library Item Information:");
    dbg!("------------------------");
    display_quantity(&book);
    display_id(&book);
    display_type(&book);

    let magazine = LibraryItem {
        quantity: 10,
        id: 1002,
        item_type: ItemType::Magazine,
    };

    dbg!("Library Item Information:");
    display_quantity(&magazine);
    display_id(&magazine);
    display_type(&magazine);

    let fiction = LibraryItem {
        quantity: 20,
        id: 1003,
        item_type: crate::library::ItemType::Fiction,
    };

    dbg!("Library Item Information:");
    display_quantity(&fiction);
    display_id(&fiction);
    display_type(&fiction);

    //=============================================

    let box1 = ShippingBox::new(10.0, 5.0, 3.0, 2.5, Color::Red);
    box1.print_characteristics();
}
