use crate::event::io::helpers::helpers::start_app;
use std::{thread, time::Duration};

pub mod event;

fn main() {
    for i in (1..=5).rev() {
        println!("Starting in {}...", i);
        thread::sleep(Duration::from_secs(1));
    }
    start_app();
}
