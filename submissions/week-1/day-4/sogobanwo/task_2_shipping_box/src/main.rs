fn main() {

    let shipping_box = Characteristics::create_box(10, 10, color_type::red);

    // shipping_box::print_char();

}


#[derive(Debug)]
enum color_type{
    red, blue, yellow
}


#[derive(Debug)]
struct Characteristics {
    dimemsion: i32,
    weight: i32,
    color: color_type,
}

impl Characteristics {

    fn create_box(x:i32, y:i32, z:color_type) {

       let new_box = Characteristics{
            dimemsion: x,
            weight:y,
            color: z
        };

        println!("The new box is: {:?}", &new_box);
        
    }

    fn print_char(&self){

        println!("This is charteristics {:?}", &self);

    }
}