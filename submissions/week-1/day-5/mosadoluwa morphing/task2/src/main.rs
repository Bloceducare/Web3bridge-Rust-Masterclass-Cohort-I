enum Books {
    Fiction {title: String, author: String, price: f32},
    SciFi {title: String, author: String, price: f32},
    Magazine {title: String, author: String, price: f32},
}

fn main() {
    let bookList = vec![
        Books::Fiction {
            title: String::from("The Great Gatsby"),
            author: String::from("F. Scott Fitzgerald"),
            price: 15.12,
        },
        Books::Fiction {
            title: String::from("To Kill a Mockingbird"),
            author: String::from("Harper Lee"),
            price: 18.50,
        },
        Books::SciFi {
            title: String::from("Dune"),
            author: String::from("Frank Herbert"),
            price: 20.99,
        },
        Books::SciFi {
            title: String::from("Neuromancer"),
            author: String::from("William Gibson"),
            price: 22.50,
        },
        Books::Magazine {
            title: String::from("National Geographic"),         
            author: String::from("Various Contributors"),
            price: 5.99,
        },
        Books::Magazine {
            title: String::from("Time"),
            author: String::from("Various Contributors"),
            price: 6.50,
        },
    ];


    println!("Book List");
    for book in &bookList {
        match book {
            Books::Fiction {title, author, price} => {
                println!("Fiction: {} by {}, Price: ${}", title, author, price);
            }
            Books::SciFi {title, author, price} => {
                println!("Sci-Fi: {} by {}, Price: ${}", title, author, price);
            }
            Books::Magazine {title, author, price} => {
                println!("Magazine: {} by {}, Price: ${}", title, author, price);
            }
        }
    }
}