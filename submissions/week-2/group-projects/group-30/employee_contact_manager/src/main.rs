//! Employee Contact Manager
//!
//! A command-line application for managing employee contact information.
//! This application demonstrates Rust best practices including:
//! - Modular code organization
//! - Proper error handling with Result types
//! - Comprehensive documentation
//! - Unit testing
//! - Type safety and memory safety

use employee_contact_manager::UserInterface;

fn main() {
    let mut ui = UserInterface::new();
    ui.run();
}

