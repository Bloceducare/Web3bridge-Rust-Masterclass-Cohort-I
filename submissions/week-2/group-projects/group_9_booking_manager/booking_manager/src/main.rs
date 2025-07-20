use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Clone)]
struct Booking {
    room: String,
    date: String,
    time: String,
    capacity: u32,
}

type BookingKey = String;

struct BookingManager {
    bookings: HashMap<BookingKey, Vec<Booking>>,
}

impl BookingManager {
    fn new() -> Self {
        Self {
            bookings: HashMap::new(),
        }
    }

    fn make_key(room: &str, date: &str) -> String {
        format!("{}|{}", room.to_lowercase(), date)
    }

    fn add_booking(&mut self, room: String, date: String, time: String, capacity: u32) {
        let booking = Booking {
            room: room.clone(),
            date: date.clone(),
            time,
            capacity,
        };
        let key = Self::make_key(&room, &date);
        self.bookings.entry(key).or_default().push(booking);
        println!(" Booking added.");
    }

    fn view_bookings(&self) {
        if self.bookings.is_empty() {
            println!("📭 No bookings found.");
            return;
        }

        println!("\n All Bookings:");
        for (_key, bookings) in &self.bookings {
            for b in bookings {
                println!(
                    "Room: {}, Date: {}, Time: {}, Capacity: {}",
                    b.room, b.date, b.time, b.capacity
                );
            }
        }
    }

    fn remove_booking(&mut self, room: String, date: String, time: String) {
        let key = Self::make_key(&room, &date);

        if let Some(bookings) = self.bookings.get_mut(&key) {
            let original_len = bookings.len();
            bookings.retain(|b| b.time != time);

            if bookings.len() < original_len {
                println!("Booking at {} removed successfully.", time);
            } else {
                println!("No booking found at that time.");
            }

            if bookings.is_empty() {
                self.bookings.remove(&key);
            }
        } else {
            println!("No bookings found for room '{}' on date '{}'.", room, date);
        }
    }

    fn edit_booking(&mut self, room: String, date: String, time: String) {
        let key = Self::make_key(&room, &date);

        if let Some(bookings) = self.bookings.get_mut(&key) {
            if let Some(booking) = bookings.iter_mut().find(|b| b.time == time) {
                println!("Editing booking: Room: {}, Date: {}, Time: {}, Capacity: {}",
                         booking.room, booking.date, booking.time, booking.capacity);

                let new_time = prompt_with_default("Enter new time (or leave blank to keep): ", &booking.time);
                let new_capacity = prompt_with_default("Enter new capacity (or leave blank to keep): ", &booking.capacity.to_string());

                if new_time.is_empty() && new_capacity.is_empty() {
                    println!("Edit cancelled.");
                    return;
                }

                if !new_time.is_empty() {
                    booking.time = new_time;
                }

                if !new_capacity.is_empty() {
                    booking.capacity = new_capacity.parse().unwrap_or(booking.capacity);
                }

                println!("Booking updated.");
            } else {
                println!("No booking found at that time.");
            }
        } else {
            println!("No bookings found for room '{}' on date '{}'.", room, date);
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn prompt_with_default(message: &str, default: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let trimmed = buf.trim();
    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}

fn main() {
    let mut manager = BookingManager::new();

    loop {
        println!("\n== Meeting Room Reservation ==");
        println!("1. Add Booking");
        println!("2. View All Bookings");
        println!("3. Remove Booking");
        println!("4. Edit Booking");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => {
                let room = prompt("Enter Room Name: ");
                let date = prompt("Enter Date (YYYY-MM-DD): ");
                let time = prompt("Enter Time (e.g. 14:00): ");
                let capacity_str = prompt("Enter Capacity: ");
                let capacity = capacity_str.trim().parse().unwrap_or(0);

                manager.add_booking(room, date, time, capacity);
            }
            "2" => {
                manager.view_bookings();
            }
            "3" => {
                let room = prompt("Enter Room Name to remove: ");
                let date = prompt("Enter Date (YYYY-MM-DD): ");
                let time = prompt("Enter Time to remove (e.g. 14:00): ");
                manager.remove_booking(room, date, time);
            }
            "4" => {
                let room = prompt("Enter Room Name to edit: ");
                let date = prompt("Enter Date (YYYY-MM-DD): ");
                let time = prompt("Enter Time to edit (e.g. 14:00): ");
                manager.edit_booking(room, date, time);
            }
            "5" => {
                println!("Exiting. Goodbye!");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
