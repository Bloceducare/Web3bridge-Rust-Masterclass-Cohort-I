#[derive(Copy, Clone, Debug)]
enum ItemType {
    Book,
    Magazine,
    Fiction
}

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType
}

fn display_quantity(item: &LibraryItem) -> i32 {
    item.quantity
}

fn display_id(item: &LibraryItem) -> i32 {
    item.id
}

fn display_item_type(item: &LibraryItem) -> ItemType {
    item.item_type
}


fn main() {
    let lib = LibraryItem {
        quantity: 500,
        id: 1,
        item_type: ItemType::Magazine
    };

    println!(
        "The quantity of {:?} the library is {}, id {}.",
        lib.item_type,
        lib.quantity,
        lib.id,
    );
}

