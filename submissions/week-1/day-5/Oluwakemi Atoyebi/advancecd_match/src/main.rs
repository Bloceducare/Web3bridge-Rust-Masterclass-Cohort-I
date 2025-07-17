enum Book {
    SciFi {
        price: i32,
        title: String,
    },
    Magazine {
        price: i32,
        author: String,
        title: String,
    },
    Fiction {
        price: i32,
        author: String,
        title: String,
    },
}

fn main() {
    let books = vec![
        Book::SciFi {
            price: 1,
            title: String::from("Alien Agenda"),
        },
        Book::Magazine {
            price: 12,
            author: String::from("Forbes Editorial"),
            title: String::from("Forbes under 30"),
        },
        Book::Fiction {
            price: 5,
            author: String::from("J.K Rowlings"),
            title: String::from("Harry Potter"),
        },
    ];

    for book in books {
        match book {
            Book::Fiction { title, author, price } => {
                println!("Fiction: {} by {} - ${}", title, author, price);
            }
            Book::Magazine { title, author, price } => {
                println!("Magazine: {} by {} - ${}", title, author, price);
            }
            Book::SciFi { title, price } => {
                println!("SciFi: {} - ${}", title, price);
            }
        }
    }
}
