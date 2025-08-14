/**
Task 2: Implementing Functionality with the impl Keyword
Requirements
Print the characteristics of a shipping box
Must include dimensions, weight, and color
Notes
Use a struct to encapsulate the box characteristics
Use an enum for the box color
Implement functionality on the box struct to create a new box
Implement functionality on the box struct to print the characteristics
*/

#[allow(dead_code)]

#[derive(Debug)]
enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Pink,
    Brown,
    Black,
}

struct Box {
    dimensions: (f32, f32, f32),
    weight: f32,
    color: BoxColor,
}

impl Box {
    fn new(dimensions: (f32, f32, f32), weight: f32, color: BoxColor) -> Self {
        Self { dimensions, weight, color }
    }
    fn print_box_details(&self) {
        let (length, width, height) = self.dimensions;
        println!("Box Length {:?}", length);
        println!("Box Width {:?}", width);
        println!("Box Height {:?}", height);

        println!("Box Dimensions: {:?}", self.dimensions);
        println!("Box Weight: {}", self.weight);
        println!("Box Color: {:?}", self.color);
    }
}

fn main() {
    let shipping_box = Box::new((10.0, 10.0, 10.0), 10.0, BoxColor::Red);
    shipping_box.print_box_details();
}   