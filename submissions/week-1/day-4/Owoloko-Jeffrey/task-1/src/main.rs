#[derive(Debug)]
enum ItemType{
    Book,
    Magazine,
    Fiction
}

#[derive(Debug)]
struct LibraryItem{
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

fn display_quantity(item: &LibraryItem){
    println!("Quantity: {:?}", item.quantity);
}

fn display_id(item: &LibraryItem){
    println!("ID: {:?}", item.id);
}

fn display_item_type(item: &LibraryItem){
    println!("Item Type: {:?}", item.item_type);
}
fn main() {
    let item = LibraryItem{
        quantity: 10,
        id: 1,
        item_type: ItemType::Book,
    };
    display_quantity(&item);
    display_id(&item);
    display_item_type(&item);
}
