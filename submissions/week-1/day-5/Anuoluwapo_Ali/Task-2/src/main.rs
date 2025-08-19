#[derive(Debug)]
enum Book {
    Fiction { title: String, author: String, price: f64 },
    Magazine { title: String, author: String, price: f64 },
    SciFi { title: String, price: f64 },
}

fn main() {
    let books = vec![
        Book::Fiction {
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
            price: 12.99,
        },
        Book::Magazine {
            title: "National Geographic".to_string(),
            author: "Various Contributors".to_string(),
            price: 5.99,
        },
        Book::SciFi {
            title: "Dune".to_string(),
            price: 15.99,
        },
    ];

    println!(" Book Library Inventory:");

    for (index, book) in books.iter().enumerate() {
        println!("\n{}. Book Information:", index + 1);
        
        match book {
            Book::Fiction { title, author, price } => {
                println!("   Type: Fiction");
                println!("   Title: {}", title);
                println!("   Author: {}", author);
                println!("   Price: ${:.2}", price);
            }
            Book::Magazine { title, author, price } => {
                println!("   Type: Magazine");
                println!("   Title: {}", title);
                println!("   Author: {}", author);
                println!("   Price: ${:.2}", price);
            }
            Book::SciFi { title, price } => {
                println!("   Type: Science Fiction");
                println!("   Title: {}", title);
                println!("   Price: ${:.2}", price);
            }
        }
    }

    let total_value: f64 = books.iter().map(|book| {
        match book {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        }
    }).sum();

    println!("Total Inventory Value: ${:.2}", total_value);
}