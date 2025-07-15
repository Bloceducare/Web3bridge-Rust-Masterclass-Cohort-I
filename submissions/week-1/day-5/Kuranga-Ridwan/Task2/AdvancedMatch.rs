enum Book {
    Fiction { author: String, price: f64 },
    Magazine { author: String, price: f64 },
    SciFi { price: f64 },
}

fn main() {
    
    let books: Vec<Book> = vec![
        Book::Fiction {
            author: String::from("Kuranga Ridwaan"),
            price: 15.99,
        },
        Book::Magazine {
            author: String::from("Ikogosi_Warm_Spring"),
            price: 7.50,
        },
        Book::SciFi { price: 22.00 },
        Book::Fiction {
            author: String::from("Omo Gidi"),
            price: 12.75,
        },
        Book::Magazine {
            author: String::from("Alaroye"),
            price: 6.99,
        },
        Book::SciFi { price: 18.25 },
    ];

    println!("--- List of Books and Their Information ---");
    println!("-----------------------------------------");

    
    for book in books {
        match book {
            // Match the Fiction variant, binding the author and price.
            Book::Fiction { author, price } => {
                println!("Type: Fiction");
                println!("  Author: {}", author);
                println!("  Price: #{:.2}", price);
            }
            // Match the Magazine variant, binding the author and price.
            Book::Magazine { author, price } => {
                println!("Type: Magazine");
                println!("  Publisher/Author: {}", author); // Magazine might have a publisher or primary author
                println!("  Price: #{:.2}", price);
            }
            // Match the SciFi variant, binding only the price.
            Book::SciFi { price } => {
                println!("Type: Sci-Fi");
                println!("  Price: #{:.2}", price);
            }
        }
        println!("-----------------------------------------");
    }
}
