#[derive(Debug)]
enum Book {
    Fiction {
        title: String,
        author: String,
        price: f64,
    },
    Magazine {
        title: String,
        author: String,
        price: f64,
    },
    SciFi {
        title: String,
        price: f64,
    },
}

fn main() {
    // Create one of each book type
    let books = vec![
        Book::Fiction {
            title: String::from("To Kill a Mockingbird"),
            author: String::from("Harper Lee"),
            price: 12.99,
        },
        Book::Magazine {
            title: String::from("National Geographic"),
            author: String::from("Various Contributors"),
            price: 5.99,
        },
        Book::SciFi {
            title: String::from("Dune"),
            price: 15.99,
        },
    ];

    println!("Book Collection:");

    for (index, book) in books.iter().enumerate() {
        println!("\n{}. ", index + 1);

        match book {
            Book::Fiction {
                title,
                author,
                price,
            } => {
                println!("Fiction Book");
                println!("   Title: {}", title);
                println!("   Author: {}", author);
                println!("   Price: ${:.2}", price);
            }
            Book::Magazine {
                title,
                author,
                price,
            } => {
                println!("Magazine");
                println!("   Title: {}", title);
                println!("   Author: {}", author);
                println!("   Price: ${:.2}", price);
            }
            Book::SciFi { title, price } => {
                println!("Science Fiction Book");
                println!("   Title: {}", title);
                println!("   Price: ${:.2}", price);
            }
        }
    }

    // Calculate and display total value
    let total_value: f64 = books
        .iter()
        .map(|book| match book {
            Book::Fiction { price, .. } => *price,
            Book::Magazine { price, .. } => *price,
            Book::SciFi { price, .. } => *price,
        })
        .sum();

    println!("\nTotal Collection Value: ${:.2}", total_value);
}
