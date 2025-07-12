#[derive(Debug)]
#[derive(PartialEq)]
enum Status {
    NotCompleted,
    Completed,
    Deleted
}

#[derive(Debug)]
struct Task {
    title: String,
    status: Status,
    task_id: u32
}

impl Task {
    fn create_todo(title: String, status: Status, task_id: u32) -> Task {
        Task {
            title,
            status,
            task_id
        }
    }

    fn update_todo(&mut self) {
        println!("Updating task ID {}: {} -> Do a todo task", self.task_id, self.title);
        self.title = String::from("Do a todo task");
        self.status = Status::NotCompleted;
    }

    fn delete_todo(&mut self) {
        println!("Deleting task ID {}: {}", self.task_id, self.title);
        self.status = Status::Deleted;
    }

    fn edit_todo(&mut self, new_title: String) {
        println!("Editing task ID {}: {} -> {}", self.task_id, self.title, new_title);
        self.title = new_title;
    }

    fn mark_completed(&mut self) {
        println!("Marking task ID {} as completed: {}", self.task_id, self.title);
        self.status = Status::Completed;
    }
}

fn main() {

    println!("1. Creating Todo:");
    let mut task1 = Task::create_todo(String::from("Learn Rust Programming"), Status::NotCompleted, 1);
    let mut task2 = Task::create_todo(String::from("Complete Assignment"), Status::NotCompleted, 2);
    let mut task3 = Task::create_todo(String::from("Submit Project"), Status::NotCompleted, 3);
    
    println!("\nPrinting created todo:");
    println!("Task 1: {:#?}", task1);
    println!("Task 2: {:#?}", task2);
    println!("Task 3: {:#?}", task3);

    println!("\n2. UPDATING TODO:");
    task1.update_todo();
    println!("Updated Task 1: {:#?}", task1);

    println!("\n3. EDITING TODO:");
    task2.edit_todo( String::from("Complete Rust Assignment Successfully"));
    println!("Edited Task 2: {:#?}", task2);

    println!("\n4. MARKING TODO AS COMPLETED:");
    task2.mark_completed();
    println!("Completed Task 2: {:#?}", task2);

    println!("\n5. DELETING TODO:");
    task3.delete_todo();
    println!("Deleted Task 3: {:#?}", task3);

    println!("\nAll My Tasks");
    println!("Task 1 (Updated): {:#?}", task1);
    println!("Task 2 (Edited & Completed): {:#?}", task2);
    println!("Task 3 (Deleted): {:#?}", task3);
}
