#[derive(Debug)]
enum BoxColor {
    Green,
    Yellow,
    Orange,
    Black,
}

struct StorageCrate {
    height_cm: f64,
    width_cm: f64,
    depth_cm: f64,
    mass_kg: f64,
    color: BoxColor,
}

impl StorageCrate {
    fn build(height: f64, width: f64, depth: f64, mass: f64, color: BoxColor) -> Self {
        Self {
            height_cm: height,
            width_cm: width,
            depth_cm: depth,
            mass_kg: mass,
            color,
        }
    }

    fn display_info(&self) {
        println!("--- Crate Details ---");
        println!("Size: {} x {} x {} cm", self.height_cm, self.width_cm, self.depth_cm);
        println!("Mass: {} kg", self.mass_kg);
        println!("Crate Color: {:?}", self.color);
        println!("Internal Volume: {:.2} cubic cm", self.volume());
    }

    fn volume(&self) -> f64 {
        self.height_cm * self.width_cm * self.depth_cm
    }
}

fn main() {
    let create_small_box = StorageCrate::build(18.0, 12.0, 9.0, 1.2, BoxColor::Black);
    let create_medium_box = StorageCrate::build(35.0, 25.0, 20.0, 2.8, BoxColor::Green);
    let create_biggest_box = StorageCrate::build(55.0, 40.0, 35.0, 7.5, BoxColor::Orange);

    create_small_box.display_info();
    println!();

    create_medium_box.display_info();
    println!();

    create_biggest_box.display_info();
}
