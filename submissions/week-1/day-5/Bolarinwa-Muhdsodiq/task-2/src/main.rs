#[derive(Debug)]
enum Book {
    Magazine { author: String, price: f64 },
    Fiction { author: String, price: f64 },
    SciFi { price: f64 },
}

fn main() {
    // Create a vector of books
    let books = vec![
        Book::Magazine {
            author: String::from("John Smith"),
            price: 12.99,
        },
        Book::Fiction {
            author: String::from("Jane Doe"),
            price: 24.99,
        },
        Book::SciFi { price: 19.99 },
    ];

    println!("=== Book Inventory ===");

    // Iterate through the vector and use match to print book info
    for (index, book) in books.iter().enumerate() {
        print!("Book {}: ", index + 1);

        match book {
            Book::Magazine { author, price } => {
                println!("Magazine by {} - ${:.2}", author, price);
            }
            Book::Fiction { author, price } => {
                println!("Fiction by {} - ${:.2}", author, price);
            }
            Book::SciFi { price } => {
                println!("SciFi - ${:.2}", price);
            }
        }
    }
}
// 'cargo run 
// Compiling task-2 v0.1.0 (/Users/amityclev/Documents/dev/rustweek/week1/Web3bridge-Rust-Masterclass-Cohort-I/submissions/week-1/day-5/Bolarinwa-Muhdsodiq/task-2)
//  Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.43s
//   Running `target/debug/task-2`
// === Book Inventory ===
// Book 1: Magazine by John Smith - $12.99
// Book 2: Fiction by Jane Doe - $24.99
// Book 3: SciFi - $19.99