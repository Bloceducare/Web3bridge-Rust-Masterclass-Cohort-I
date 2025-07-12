            
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
    println!("Item Type: {:?}", item.item_type);
}