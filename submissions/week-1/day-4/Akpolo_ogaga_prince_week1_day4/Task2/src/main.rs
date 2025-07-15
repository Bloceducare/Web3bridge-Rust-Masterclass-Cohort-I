// Task 2: Implementing Functionality with the impl Keyword
// Requirements
// Print the characteristics of a shipping box
// Must include dimensions, weight, and color
// Notes
// Use a struct to encapsulate the box characteristics
// Use an enum for the box color
// Implement functionality on the box struct to create a new box
// Implement functionality on the box struct to print the characteristics




#[derive(Debug)] 
enum BoxColor {
    Brown,
    White,
    Blue,
    Red,
    Green,
}
// so i notice that you have put derive debug so we can be able to print the enum values directly

#[derive(Debug)]
struct ShippingBox {
    length: f32,
    width: f32,
    height: f32,
    weight: f32,
    color: BoxColor, 
}

// so here in my implementation of the ShippingBox struct, i have added the color field of type BoxColor
// this allows us to use the enum BoxColor to specify the color of the box
// and i named the function apapa_new_box to create a new box
impl ShippingBox {
    
    fn apapa_new_box(length: f32, width: f32, height: f32, weight: f32, color: BoxColor) -> Self {
        ShippingBox {
            length,
            width,
            height,
            weight,
            color,
        }
    }
    
    fn print_characteristics(&self) {
        println!("This is Apapa Shipping Box!");
        println!("the dimension is: {} × {} × {} cm", self.length, self.width, self.height);
        println!("The weight is: {} kg", self.weight);
        println!("The color is: {:?}", self.color);
        println!("The volume is: {} cubic cm", self.calculate_volume());

    }
 
    fn calculate_volume(&self) -> f32 {
        self.length * self.width * self.height
    }
}

fn main() {
    
    let my_box = ShippingBox::apapa_new_box(
        21.5,
        10.0,
        12.0,
        2.3,
        BoxColor::Brown,
    );
    
  
    my_box.print_characteristics();

    //i Creating another shipping box using the apapa_new_box function
    // and printing its characteristics
    let another_box = ShippingBox::apapa_new_box(
        30.0,
        20.0,
        12.5,
        4.7,
        BoxColor::Blue,
    );
    
    another_box.print_characteristics();
    
  

    println!("The first box is {:?} colored", my_box.color);
    println!("The second box weighs {} kg", another_box.weight);
}