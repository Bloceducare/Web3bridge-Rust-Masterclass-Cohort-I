use std::io;

struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    fn new_todo() -> TodoList {
        TodoList { list: Vec::new() }
    }

    fn create_todo(&mut self, title: String, description: String) {
        let new_id: u32 = self.list.len().try_into().unwrap();
        let new_todo = Todo {
            id: new_id,
            title,
            description,
            completed: false,
        };
        self.list.push(new_todo);
        print!("Todo added!")
    }

    fn update_todo(&mut self, id: u32, title: String, description: String) {
        for todo in &mut self.list {
            if todo.id == id {
                todo.title = title.clone();
                todo.description = description.clone();
            }
        }
    }

    fn mark_todo_complete(&mut self, id: u32) {
        for todo in &mut self.list {
            if todo.id == id {
                todo.completed = true;
            }
        }
    }
    fn delete_todo(&mut self, id: u32) {
        if let Some(index) = self.list.iter().position(|x| x.id == id) {
            self.list.remove(index);
        } else {
            println!("Todo not found");
        }
    }

    fn return_todos(&self) {
        if self.list.len() > 0 {
            for todo in &self.list {
                println!(
                    "  Id {id}:\n  Title: {title}\n  Description: {description}\n  Completed: {completed}",
                    id = todo.id,
                    title = todo.title.replace('\n', "\n  "),
                    description = todo.description.replace('\n', "\n  "),
                    completed = todo.completed
                );
            }
        } else {
            print!("You have not added any todos")
        }
    }
}

fn main() {
    let mut todo_list = TodoList::new_todo();
    loop {
        println!(" Enter 1 to add a new todo,\n Enter 2 to view all todos,\n Enter 3 to update a todo,\n Enter 4 to mark a todo as complete,\n Enter 5 to delete a todo, \n and Enter 6 to exit this appliction");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the todo title");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Could not read line");

                println!("Enter the todo description");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Could not read line");

                todo_list.create_todo(title, description);
            }
            2 => {
                todo_list.return_todos();
            }
            3 => {
                println!("Enter the id of the Todo you want to update");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Could not read line");

                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter the new title of the Todo");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Could not read line");

                println!("Enter the new description of the Todo");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Could not read line");
                todo_list.update_todo(id, title, description);
            }
            4 => {
                println!("Enter the id of the completed Todo");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Could not read line");

                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                todo_list.mark_todo_complete(id);
            }
            5 => {
                println!("Enter the id of the Todo you want to delete");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Could not read line");

                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                todo_list.delete_todo(id);
            }
            6 => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}
