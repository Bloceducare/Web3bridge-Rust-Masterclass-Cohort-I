use std::vec::Vec;

enum Book {
    Fiction(String, f32),  // author, price
    Magazine(String, f32), // author, price
    SciFi(f32),            // price
}


fn main() {
    let mut books: Vec<Book> = Vec::new();

    let village_people= Book::Fiction(String::from("Tosin Ade"), 15.99);
    books.push(village_people);

    let pastry_world = Book::Magazine(String::from("Ayo Segun"), 5.99);
    books.push(pastry_world);

    let scifi = Book::SciFi(20.00);
    books.push(scifi);

    for book in &books {
        match book {
            Book::Fiction(author, price) => {
                println!("Fiction by {}: ${:.2}", author, price);
            }
            Book::Magazine(author, price) => {
                println!("Magazine by {}: ${:.2}", author, price);
            }
            Book::SciFi(price) => {
                println!("Sci-Fi book: ${:.2}", price);
            }
        }
    }
}
