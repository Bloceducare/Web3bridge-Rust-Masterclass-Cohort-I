#[derive(Debug)]
enum BoxColor {
    Brown,
    Black,
    Red
}

struct Box {
    dimensions: (u32, u32),
    weight: u32,
    color: BoxColor
}

impl Box {
    fn new(dimensions:(u32, u32), weight:u32, color:BoxColor) -> Box {
        Box {
            dimensions,
            weight,
            color
        }
    }

    fn print_box(&self) {
        let (width, height) = self.dimensions;
        println!("This box has a dimension of {}cm x {}cm, it's color is {:?} and it weighs {}kg", width, height, &self.color, self.weight)
    }
}

fn main() {
    let my_box = Box::new((10, 24), 50, BoxColor::Black);
    my_box.print_box();
}
