              # Task 1: Structs - Library Item Management

## 📝 Task Description

You are building a basic library system to manage different types of items in a collection.

### Your task:

- Print out the **quantity** and **ID number** of a library item (e.g., a book or magazine).
- The library items should have different types represented using an `enum`.

---

## 💡Requirements

- Create a `struct` named `LibraryItem`.
- It should contain three fields:
  - `quantity: i32`
  - `id: i32`
- Add a third field called `item_type`, which should be of an `enum` type named `ItemType`.
- The `ItemType` enum should have at least two variants (e.g., `Book`, `Magazine`,`Fiction`).

- Implement two functions:
  - `display_quantity(item: &LibraryItem)` — prints the quantity
  - `display_id(item: &LibraryItem)` — prints the ID
  - another fn to display the type of the LibraryItem.

---

# Task 2: Implementing Functionality with the `impl` Keyword

## Requirements

- Print the characteristics of a shipping box
- Must include dimensions, weight, and color

## Notes

- Use a struct to encapsulate the box characteristics
- Use an enum for the box color
- Implement functionality on the box struct to create a new box
- Implement functionality on the box struct to print the characteristics

---
