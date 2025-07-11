enum ItemType {
    Book,
    Magazine,
    Newspaper,
    Novel,
    Journal,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

struct LibraryItem {
    quantity: i32,
    id: i32,
    item_type: ItemType,
}

struct ShippingBox {
    weight: f64,
    dimensions: (f64, f64, f64), // (length, width, height)
    color: Color,
}

impl ShippingBox {
    fn new(weight: f64, dimensions: (f64, f64, f64), color: Color) -> Self {
        ShippingBox {
            weight,
            dimensions,
            color,
        }
    }

    fn display(&self) {
        println!(
            "Shipping Box - Weight: {}, Dimensions: ({}, {}, {}), Color: {:?}",
            self.weight, self.dimensions.0, self.dimensions.1, self.dimensions.2, self.color
        );
    }
}

fn main() {
    let item1 = LibraryItem {
        quantity: 5,
        id: 101,
        item_type: ItemType::Book,
    };

    let item2 = LibraryItem {
        quantity: 3,
        id: 202,
        item_type: ItemType::Magazine,
    };

    display_quantity(&item1);
    display_id(&item2);
    display_item_type(&item1);
    display_item_type(&item2);

    let box1 = ShippingBox::new(10.5, (12.0, 8.0, 6.0), Color::Red);
    box1.display();
}

fn display_quantity(item: &LibraryItem) {
    println!("The quantity of the item is: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("The ID of the item is: {}", item.id);
}

fn display_item_type(item: &LibraryItem) {
    match item.item_type {
        ItemType::Book => println!("The item is a Book."),
        ItemType::Magazine => println!("The item is a Magazine."),
        ItemType::Newspaper => println!("The item is a Newspaper."),
        ItemType::Novel => println!("The item is a Novel."),
        ItemType::Journal => println!("The item is a Journal."),
    }
}
