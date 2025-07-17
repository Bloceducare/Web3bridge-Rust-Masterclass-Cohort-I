fn main() {
    
    let lib_item = &LibraryItem {
        quantity: 10,
        id: 1,
        item_type: ItemType::Book,
    };

    display_quantity(lib_item);

    display_id(lib_item);

    type_of_library_Item(lib_item);
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
    Fiction
}

#[derive(Debug)]
struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

fn display_quantity(item: &LibraryItem){

    println!("The quantity of the created library item is {}", item.quantity);

}

fn display_id(item: &LibraryItem){

    println!("The id of the created library item is {}", item.id);

}

fn type_of_library_Item(item: &LibraryItem) {

    println!("This is the type of library {:?}", item);
}
