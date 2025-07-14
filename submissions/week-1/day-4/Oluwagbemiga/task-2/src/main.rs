struct Box {
    width: i32,
    height: i32,
    color: Box_Color,
}

#[derive(Debug)]
enum Box_Color {
    Red,
    Green,
    Blue,
}

impl Box {
    fn new(width: i32, height: i32, color: Box_Color) -> Box {
        Box {
            width,
            height,
            color,
        }
    }

    fn display_dimensions(&self) {
        println!(
            "Box dimensions are: width of {:#?}, height of {:#?}, color of {:#?}",
            self.width, self.height, self.color
        );
    }
}

fn main() {
    let my_box = Box::new(10, 20, Box_Color::Red);
    let our_box = Box::new(100, 350, Box_Color::Green);
    my_box.display_dimensions();
    our_box.display_dimensions();
}
