#[derive(Debug)]
enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn main() {
    // Create one of each book type
    let books = vec![
        Book::Fiction {
            title: String::from("fictional book"),
            author: String::from("John Doe"),
            price: 12.99,
        },
        Book::Magazine {
            title: String::from("Tech Today"),
            author: String::from("contributors"),
            price: 5.99,
        },
        Book::SciFi {
            title: String::from("Dune"),
            price: 15.99,
        },
    ];

    println!("Book Library Collection");
    println!("{}", "=".repeat(50));
    // Iterate through the vector and use match to print book info
    for (index, book) in books.iter().enumerate() {
        println!("\n{}. Book Information:", index + 1);
        
        match book {
            Book::Fiction { title, author, price } => {
                println!("   Type: Fiction");
                println!("   Title: {}", title); 
                println!("   Author: {}", author);
                println!("   Price: ${:.2}", price);
            },
            Book::Magazine { title, author, price } => {
                println!("   Type: Magazine");
                println!("   Title: {}", title);
                println!("   Author: {}", author);
                println!("   Price: ${:.2}", price);
            },
            Book::SciFi { title, price } => {
                println!("   Type: Science Fiction");
                println!("   Title: {}", title);
                println!("   Price: ${:.2}", price);
            },
        }
    }
    println!("\n{}", "=".repeat(50));
    println!("Total books in collection: {}", books.len());
}