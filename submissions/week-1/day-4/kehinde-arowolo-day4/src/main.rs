            
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

#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
    Brown,
    White,
}

struct ShippingBox {
    length: f64,
    width: f64,
    height: f64,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    fn new(length: f64, width: f64, height: f64, weight: f64, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }

    fn print_characteristics(&self) {
        println!("Box Characteristics:");
        println!("  Dimensions: {}L x {}W x {}H", self.length, self.width, self.height);
        println!("  Weight: {} kg", self.weight);
        println!("  Color: {:?}", self.color);
    }
}

fn main() {
    let my_box = ShippingBox::new(10.5, 8.0, 6.0, 2.3, BoxColor::Brown);
    my_box.print_characteristics();
}