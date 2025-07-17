/**
📝 Task Description
You are building a basic library system to manage different types of items in a collection.

Your task:
Print out the quantity and ID number of a library item (e.g., a book or magazine).
The library items should have different types represented using an enum.
💡Requirements
Create a struct named LibraryItem.

It should contain three fields:

quantity: i32
id: i32
Add a third field called item_type, which should be of an enum type named ItemType.

The ItemType enum should have at least two variants (e.g., Book, Magazine,Fiction).

Implement two functions:

display_quantity(item: &LibraryItem) — prints the quantity
display_id(item: &LibraryItem) — prints the ID
another fn to display the type of the LibraryItem.

*/

#[allow(dead_code)]

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

fn main() {
    let item = LibraryItem {
        quantity: 10,
        id: 123456,
        item_type: ItemType::Book,
    };
    display_quantity(&item);
    display_id(&item);
    display_item_type(&item);
}
