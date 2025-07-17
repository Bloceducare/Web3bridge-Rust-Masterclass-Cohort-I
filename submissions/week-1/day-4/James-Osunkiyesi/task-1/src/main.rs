#[derive(Debug)]
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

fn main() {
    let item_one = LibraryItem {
        quantity: 5,
        id: 1,
        item_type: ItemType::Fiction
    };
    display_quantity(&item_one);
    display_id(&item_one);
    display_item_type(&item_one);

}

fn display_quantity(item: &LibraryItem)  {
    println!("quantity of item: {}", {item.quantity});
}
fn display_id(item: &LibraryItem)  {
    println!("id of item: {}", {item.id});
}
fn display_item_type(item: &LibraryItem)  {
    println!("type of item: {:?}", {&item.item_type});
}
