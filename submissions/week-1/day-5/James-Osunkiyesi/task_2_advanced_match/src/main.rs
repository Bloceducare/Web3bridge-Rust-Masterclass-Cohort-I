enum BookType {
    Fiction {
        title: String,
        author: String,
        price: u32,
    },
    Magazine {
        title: String,
        author: String,
        price: u32,
    },
    SciFi {
        title: String,
        price: u32,
    },
}

fn print_books (list: &Vec<BookType>) {
    for book in list {
        match book {
            BookType::SciFi {title, price} => println!("| Name: {}; Type: Sci-fi; Price: ${} |", title, price ),
            BookType::Fiction {title, price,author} => println!("| Name: {}; Type: Fiction; Author: {}; Price: ${} |", title, author, price ),
            BookType::Magazine {title, price, author} => println!("| Name: {}; Type: Magazine; Author: {}; Price: ${} |", title, author, price ),
        }
    }
}

fn main() {
    let book_list: Vec<BookType> = vec![
        BookType::Fiction {
            title: String::from("Nineteen Eighty-four"),
            author: String::from("George Orwell"),
            price: 90,
        },
        BookType::Fiction {
            title: String::from("The Hunger Games"),
            author: String::from("Suzanne Collins"),
            price: 150,
        },
        BookType::Magazine {
            title: String::from("The Lagos Review"),
            author: String::from("Tomi Kan"),
            price: 45,
        },
        BookType::Magazine {
            title: String::from("Isele Magazine"),
            author: String::from("Ukamaka Olisakwe"),
            price: 62,
        },
        BookType::SciFi {
            title: String::from("Dune"),
            price: 34,
        },
        BookType::SciFi {
            title: String::from("Doomsday"),
            price: 14,
        },
    ];

    print_books(&book_list)
}
