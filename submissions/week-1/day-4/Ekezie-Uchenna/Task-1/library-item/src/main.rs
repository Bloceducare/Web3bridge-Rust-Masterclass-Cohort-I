// # Task 1: Structs - Library Item Management

// ## 📝 Task Description

// You are building a basic library system to manage different types of items in a collection.

// ### Your task:

// - Print out the **quantity** and **ID number** of a library item (e.g., a book or magazine).
// - The library items should have different types represented using an `enum`.

// ---

// ## 💡Requirements

// - Create a `struct` named `LibraryItem`.
// - It should contain three fields:
//   - `quantity: i32`
//   - `id: i32`
// - Add a third field called `item_type`, which should be of an `enum` type named `ItemType`.
// - The `ItemType` enum should have at least two variants (e.g., `Book`, `Magazine`,`Fiction`).

// - Implement two functions:
//   - `display_quantity(item: &LibraryItem)` — prints the quantity
//   - `display_id(item: &LibraryItem)` — prints the ID
//   - another fn to display the type of the LibraryItem.

// - In the `main` function, create an instance of `LibraryItem` and call the functions to display its properties.
// # Task 1: Structs - Library Item Management
struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}
enum ItemType {
    Book,
    Magazine,
    Fiction,
}

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}
fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}
fn displat_lib_type(item: &LibraryItem) {
    match &item.item_type {
        ItemType::Book => println!("Item Type: Book"),
        ItemType::Magazine => println!("Item Type: Magazine"),
        ItemType::Fiction => println!("Item Type: Fiction"),
    }
}
fn main() {
    let book = LibraryItem {
        quantity: 3,
        id: 101,
        item_type: ItemType::Book,
    };

    display_quantity(&book);
    display_id(&book);
    display_lib_type(&book);
}


// enum ItemType {
//     Book,
//     Magazine,
//     Fiction,
// }
// struct LibraryItem{
//     quantity: i32,
//     id: i32,
//     item_type: ItemType,

// }

// fn display_quantity(item: &LibraryItem) {
//     println!("Quantity: {}", item.quantity);
// }
// fn display_id(item: &LibraryItem) {
//     println!("ID: {}", item.id);
// }

// fn display_item_type(item: &LibraryItem) {
//     match &item.item_type {
//         ItemType::Book => println!("Item Type: Book"),
//         ItemType::Magazine => println!("Item Type: Magazine"),
//         ItemType::Fiction => println!("Item Type: Fiction"),
//     }
// }

// fn main() {
    
//     let book = LibraryItem {
//         quantity: 3,
//         id: 101,
//         item_type: ItemType::Book,
//     };

    
//     display_quantity(&book);
//     display_id(&book);
//     display_item_type(&book);
// }


