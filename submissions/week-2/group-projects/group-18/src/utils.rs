use std::thread;
use std::time::Duration;

pub fn delay_execution() {
    println!("Restarting");
    thread::sleep(Duration::from_secs(3));
}