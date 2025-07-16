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
    println!("Book Store Inventory");
    println!("=======================");


    let fiction_book = Book::Fiction {
        title: "The Great Adventure".to_string(),
        author: "John Smith".to_string(),
        price: 19.99,
    };
    
    let magazine_book = Book::Magazine {
        title: "Tech Weekly".to_string(),
        author: "Sarah Johnson".to_string(),
        price: 5.99,
    };
    
    let scifi_book = Book::SciFi {
        title: "Space Odyssey".to_string(),
        price: 24.99,
    };
    
 
    let books = vec![fiction_book, magazine_book, scifi_book];
    
    
    println!("\n Available Books:");
    println!("-------------------");
    
    for (index, book) in books.iter().enumerate() {
        print!("{}. ", index + 1);
        
     
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction: {} by {}", title, author);
                println!("Price: ${}", price);
            },
            Book::Magazine { title, author, price } => {
                println!("Magazine: {} by {}", title, author);
                println!("Price: ${}", price);
            },
            Book::SciFi { title, price } => {
                println!("Sci-Fi: {}", title);
                println!("Price: ${}", price);
            },
        }
        println!("");
    }
    
    println!("All books displayed successfully!");
}
