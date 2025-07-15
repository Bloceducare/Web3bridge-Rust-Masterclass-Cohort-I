fn main() {
    // println!("Hello, world!");
    let new_box: ShippingBox = ShippingBox::new_box(10, Dimension { length: 5, width: 5, height: 5 }, BoxColor::Red);
    let boxed: ShippingBox = new_box.display_box();
    println!("Box 1: {:?}", boxed);
}


impl ShippingBox{

    fn new_box(weight: i32, dimension: Dimension, color: BoxColor) -> Self {
        Self {
            weight,
            dimension,
            color
        }
    }

    fn display_box(&self) -> Self {
        // ShippingBox {
        //     weight: self.weight,
        //     dimension: self.dimension,
        //     color: self.color
        // }

        *self
    }
}



#[derive(Debug, Copy, Clone)]
struct ShippingBox {
    weight: i32,
    dimension: Dimension,
    color: BoxColor,
}


#[derive(Debug, Copy, Clone)]
struct Dimension {
    length: i32,
    width: i32,
    height: i32,
}


#[derive(Debug, Copy, Clone)]
enum BoxColor {
    Red,
    Blue,
    Green,
}