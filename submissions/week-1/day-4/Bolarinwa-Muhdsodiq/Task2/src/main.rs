
#[derive(Debug)]
enum Color {
    RED,
    GREEN,
    BLUE
}
struct ShoppingBox {
    dimensions: i32,
    weight: i32,
    color: Color,
}

impl ShoppingBox {
    fn create(dimensions: i32, weight: i32, color: Color) -> ShoppingBox {
        ShoppingBox {
            dimensions,
            weight,
            color,
        }
    }

    fn cal_volume(&self) -> i32 {
        self.dimensions * self.weight
    }

    fn display_item(&self) {
        println!("Shopping Box Details:");
        println!("dimensions: {}", self.dimensions);
        println!("Weight: {}", self.weight);
        println!("Color: {:?}", self.color)
    }
}


fn main() {
    println!("Hello, world!");

    let box1 = ShoppingBox::create(10, 5, Color::RED);
    box1.display_item();

    let volume = box1.cal_volume();

    println!("Volume of the box is: {}", volume);
    


}
