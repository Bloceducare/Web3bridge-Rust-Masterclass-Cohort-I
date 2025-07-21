use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct TrainingSession {
    pub session_id: i32,
    pub course_name: String,
    pub date: String,
    pub trainer: String,
}

#[derive(Debug)]
pub struct TrainingSessionManager {
    pub next_session_id: i32,
    pub sessions: HashMap<i32, TrainingSession>,
}

impl TrainingSessionManager {
    pub fn new() -> Self {
        TrainingSessionManager {
            next_session_id: 0,
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, course_name: String, date: String, trainer: String) -> i32 {
        self.next_session_id += 1;
        let session_id = self.next_session_id;
        let session = TrainingSession {
            session_id,
            course_name,
            date,
            trainer,
        };
        self.sessions.insert(session_id, session);
        session_id
    }

    pub fn view_all_sessions(&self) {
        if self.sessions.is_empty() {
            println!("No sessions found.\n");
            return;
        }
        for (_, session) in &self.sessions {
            println!("{session:?}");
        }
        println!();
    }

    pub fn delete_session(&mut self, session_id: i32) {
        self.sessions.remove(&session_id);
    }

    pub fn edit_session(
        &mut self,
        session_id: i32,
        course_name: String,
        date: String,
        trainer: String,
    ) {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            if !course_name.is_empty() {
                session.course_name = course_name;
            }
            if !date.is_empty() {
                session.date = date;
            }
            if !trainer.is_empty() {
                session.trainer = trainer;
            }
        }
    }
}

pub fn run_app() {
    let mut session_manager = TrainingSessionManager::new();
    loop {
        println!("Enter 1 to create session,\n Enter 2 to edit a session,\n Enter 3 to delete a session,\n Enter 4 to view all sessions \n and Enter 5 to exit this appliction");

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Could not read line");
        let selection: i8 = match selection.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}\n", e);
                continue;
            }
        };

        match selection {
            1 => {
                println!("Enter the course name");
                let mut course_name = String::new();
                io::stdin()
                    .read_line(&mut course_name)
                    .expect("Could not read course name");

                println!("Enter the session date");
                let mut date = String::new();
                io::stdin()
                    .read_line(&mut date)
                    .expect("Could not read date");

                println!("Enter the trainer");
                let mut trainer = String::new();
                io::stdin()
                    .read_line(&mut trainer)
                    .expect("Could not read trainer");

                let session_id = session_manager.create_session(
                    course_name.trim().to_string(),
                    date.trim().to_string(),
                    trainer.trim().to_string(),
                );
                println!("Training session created with id: {}\n", session_id)
            }

            2 => {
                println!("Enter the session id you want to edit");
                let mut session_id = String::new();
                io::stdin()
                    .read_line(&mut session_id)
                    .expect("Could not read session id");
                let session_id: i32 = match session_id.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Error: {}\n", e);
                        continue;
                    }
                };

                println!("Enter the new course name");
                let mut course_name = String::new();
                io::stdin()
                    .read_line(&mut course_name)
                    .expect("Could not read course name");

                println!("Enter the new session date");
                let mut date = String::new();
                io::stdin()
                    .read_line(&mut date)
                    .expect("Could not read date");

                println!("Enter the new trainer");
                let mut trainer = String::new();
                io::stdin()
                    .read_line(&mut trainer)
                    .expect("Could not read trainer");

                println!("Do you want to save these changes? (y/n): ");
                let mut confirmation = String::new();
                io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Could not read confirmation");

                if confirmation.trim().to_lowercase() == "y"
                    || confirmation.trim().to_lowercase() == "yes"
                {
                    session_manager.edit_session(
                        session_id,
                        course_name.trim().to_string(),
                        date.trim().to_string(),
                        trainer.trim().to_string(),
                    );
                    println!("Edit successful");
                } else {
                    println!("Edit cancelled. No changes were made.");
                    continue;
                }
            }

            3 => {
                println!("Enter the id of the session you want to delete");
                let mut session_id = String::new();
                io::stdin()
                    .read_line(&mut session_id)
                    .expect("Could not read session_id");

                let session_id: i32 = match session_id.trim().parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("Error: {}\n", e);
                        continue;
                    }
                };
                session_manager.delete_session(session_id);

                println!("Session deleted successfully");
            }

            4 => {
                session_manager.view_all_sessions();
            }

            5 => {
                break;
            }

            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn main() {
    run_app()
}