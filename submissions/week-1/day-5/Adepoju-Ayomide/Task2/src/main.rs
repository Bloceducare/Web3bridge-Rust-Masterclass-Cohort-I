enum Book {
    Novel { name: String, writer: String, cost: f32 },
    Journal { name: String, editor: String, cost: f32 },
    SpaceEpic { name: String, cost: f32 },
}

fn display_library(shelf: &Vec<Book>) {
    for item in shelf {
        match item {
            Book::Novel { name, writer, cost } => {
                println!("Novel: \"{}\" by {}, Cost: ${:.2}", name, writer, cost);
            }
            Book::Journal { name, editor, cost } => {
                println!("Journal: \"{}\" edited by {}, Cost: ${:.2}", name, editor, cost);
            }
            Book::SpaceEpic { name, cost } => {
                println!("Space Epic: \"{}\", Cost: ${:.2}", name, cost);
            }
        }
    }
}

fn main() {
    let storybook = Book::Novel {
        name: String::from("Echoes of Eternity"),
        writer: String::from("Luna Rivera"),
        cost: 20.95,
    };

    let periodical = Book::Journal {
        name: String::from("Scientific Weekly"),
        editor: String::from("Dr. Kojo Mensah"),
        cost: 106.90,
    };

    let interstellar = Book::SpaceEpic {
        name: String::from("Starlight Rebellion"),
        cost: 80.99,
    };

    let shelf = vec![storybook, periodical, interstellar];
    display_library(&shelf);
}
