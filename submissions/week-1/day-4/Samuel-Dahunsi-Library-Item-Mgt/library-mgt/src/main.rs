#[derive(Debug)]
#[allow(dead_code)]
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

fn display_quantity(item: &LibraryItem) {
    println!("Quantity: {}", item.quantity);
}

fn display_id(item: &LibraryItem) {
    println!("ID: {}", item.id);
}

fn display_item_type(item: &LibraryItem) {
    println!("Item Type: {:?}", item.item_type);
}

#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green
}

struct ShippingBox {
    dimensions: (f32, f32, f32),
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: (f32, f32, f32), weight: f32, color: Color) -> Self {
        Self { dimensions, weight, color }
    }

    fn get_dimensions(&self) -> (f32, f32, f32) {
        self.dimensions
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_color(self) -> Color {
        self.color
    }
}

fn main() {
    let item = LibraryItem {
        quantity: 10,
        id: 1,
        item_type: ItemType::Book
    };

    display_quantity(&item);
    display_id(&item);
    display_item_type(&item);


    let dimensions = (1.0, 2.0, 3.0);
    let weight = 10.0;
    let color = Color::Red;

    let shipping_box = ShippingBox::new(dimensions, weight, color);
    let box_dimensions = shipping_box.get_dimensions();
    let box_weight = shipping_box.get_weight();
    let box_color = shipping_box.get_color();
    println!("dimension is {:?}", box_dimensions);
    println!("weight is: {}", box_weight);
    println!("color is: {:?}", box_color);

}
