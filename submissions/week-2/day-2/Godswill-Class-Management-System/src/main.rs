mod student;
mod management_system;
mod ui;

use ui::UserInterface;

fn main() {
    let mut ui = UserInterface::new();
    ui.run();
}
